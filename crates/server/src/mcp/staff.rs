use super::deserialize_empty_string_as_none;
use crate::common::QueryParams;
use crate::service::{
    payroll::service as payroll_service,
    piece_record::{dto::PieceRecordResponse, service as piece_record_service},
    stats::{dto::WorkerStatsParams, service as stats_service},
};
use crate::{error::AppError, service::auth::Claims};
use entity::order::OrderStatus;
use entity::piece_record::PieceRecordStatus;
use entity::{order, piece_record, process, user, workshop};
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData, Json, ServerHandler,
};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============ 工具参数定义 ============

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMyRecordsParams {
    #[schemars(description = "按状态筛选: pending,approved,rejected。不指定则返回所有状态")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<String>,
    #[schemars(description = "开始日期，格式 YYYY-MM-DD。不指定则不限制")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD。不指定则不限制")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
    #[schemars(description = "页码，从1开始，默认1")]
    pub page: Option<u64>,
    #[schemars(description = "每页数量，默认20")]
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMyEarningsParams {
    #[schemars(description = "开始日期，格式 YYYY-MM-DD。不指定则统计全部")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD。不指定则统计全部")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMyPayrollsParams {
    #[schemars(description = "开始日期，格式 YYYY-MM-DD。不指定则不限制")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD。不指定则不限制")]
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
    #[schemars(description = "页码，从1开始，默认1")]
    pub page: Option<u64>,
    #[schemars(description = "每页数量，默认20")]
    pub page_size: Option<u64>,
}

// ============ 响应类型 ============

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyRecordsResponse {
    pub list: Vec<PieceRecordResponse>,
    pub total: u64,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyEarningsSummary {
    pub total_quantity: i64,
    pub total_amount: Decimal,
    pub pending_quantity: i64,
    pub pending_amount: Decimal,
    pub daily_stats: Vec<crate::service::stats::dto::DailyStat>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayrollListResponse {
    pub list: Vec<entity::payroll::Model>,
    pub total: u64,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTask {
    pub process_id: Uuid,
    pub process_name: String,
    pub piece_price: Decimal,
    pub order_id: Uuid,
    pub order_name: String,
    pub order_image: Option<String>,
    pub remaining_quantity: i32,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTasksResponse {
    pub list: Vec<AvailableTask>,
}

// ============ MCP 工具实现 ============

pub struct StaffMcp {
    db: DbConn,
    claims: Claims,
    pub tool_router: ToolRouter<StaffMcp>,
}

#[tool_router]
impl StaffMcp {
    pub fn new(db: DbConn, claims: Claims) -> Self {
        Self {
            db,
            claims,
            tool_router: Self::tool_router(),
        }
    }

    /// 查询我的计件记录
    #[tool(description = "查询我的计件记录，支持按状态、日期范围筛选")]
    pub async fn get_my_records(
        &self,
        Parameters(params): Parameters<GetMyRecordsParams>,
    ) -> Result<Json<MyRecordsResponse>, ErrorData> {
        let query_params = QueryParams {
            page: params.page.unwrap_or(1),
            page_size: params.page_size.unwrap_or(20),
            status: params
                .status
                .map(|s| s.split(',').map(String::from).collect()),
            start_date: params.start_date,
            end_date: params.end_date,
            ..Default::default()
        };

        let result = piece_record_service::list(&self.db, query_params, &self.claims).await?;
        Ok(Json(MyRecordsResponse {
            list: result.list,
            total: result.total,
        }))
    }

    /// 查询我的收入统计
    #[tool(description = "查询我的收入统计，包括总数量、总金额、待审批金额和每日趋势")]
    pub async fn get_my_earnings(
        &self,
        Parameters(params): Parameters<GetMyEarningsParams>,
    ) -> Result<Json<MyEarningsSummary>, ErrorData> {
        let stats_params = WorkerStatsParams {
            start_date: params.start_date.clone(),
            end_date: params.end_date.clone(),
        };

        // 获取每日统计
        let daily_result =
            stats_service::daily_stats(&self.db, None, Some(self.claims.sub), stats_params).await?;

        // 计算总计
        let mut total_quantity: i64 = 0;
        let mut total_amount = Decimal::ZERO;
        for stat in &daily_result.list {
            total_quantity += stat.total_quantity;
            total_amount += stat.total_amount;
        }

        // 查询待审批的记录
        let pending_records = piece_record::Entity::find()
            .filter(piece_record::Column::UserId.eq(self.claims.sub))
            .filter(piece_record::Column::Status.eq(PieceRecordStatus::Pending))
            .all(&self.db)
            .await
            .map_err(AppError::from)?;

        let pending_quantity: i64 = pending_records.iter().map(|r| r.quantity as i64).sum();
        let pending_amount: Decimal = pending_records.iter().map(|r| r.amount).sum();

        Ok(Json(MyEarningsSummary {
            total_quantity,
            total_amount,
            pending_quantity,
            pending_amount,
            daily_stats: daily_result.list,
        }))
    }

    /// 查询我的工资单
    #[tool(description = "查询我的工资单列表")]
    pub async fn get_my_payrolls(
        &self,
        Parameters(params): Parameters<GetMyPayrollsParams>,
    ) -> Result<Json<PayrollListResponse>, ErrorData> {
        let query_params = QueryParams {
            page: params.page.unwrap_or(1),
            page_size: params.page_size.unwrap_or(20),
            start_date: params.start_date,
            end_date: params.end_date,
            ..Default::default()
        };

        let result =
            payroll_service::list(&self.db, query_params, Some(self.claims.sub), None).await?;
        Ok(Json(PayrollListResponse {
            list: result.list,
            total: result.total,
        }))
    }

    /// 查看可接工序
    #[tool(description = "查看工坊内进行中订单的可接工序列表")]
    pub async fn get_available_tasks(&self) -> Result<Json<AvailableTasksResponse>, ErrorData> {
        use std::collections::HashMap;

        // 获取员工所属工坊
        let staff = user::Entity::find_by_id(self.claims.sub)
            .one(&self.db)
            .await
            .map_err(|e| crate::error::AppError::Database(e.into()))?
            .ok_or_else(|| crate::error::AppError::NotFound("用户不存在".into()))?;

        let workshop_id = staff
            .workshop_id
            .ok_or_else(|| crate::error::AppError::NotFound("员工未指定所属工坊".into()))?;

        let ws = workshop::Entity::find_by_id(workshop_id)
            .one(&self.db)
            .await
            .map_err(|e| crate::error::AppError::Database(e.into()))?
            .ok_or_else(|| crate::error::AppError::NotFound("工坊不存在".into()))?;

        // 查询进行中的订单
        let orders = order::Entity::find()
            .filter(order::Column::BossId.eq(ws.owner_id))
            .filter(order::Column::Status.eq(OrderStatus::Processing))
            .all(&self.db)
            .await
            .map_err(AppError::from)?;

        if orders.is_empty() {
            return Ok(Json(AvailableTasksResponse { list: vec![] }));
        }

        let order_ids: Vec<Uuid> = orders.iter().map(|o| o.id).collect();
        let orders_map: HashMap<Uuid, order::Model> =
            orders.into_iter().map(|o| (o.id, o)).collect();

        // 查询这些订单的工序
        let processes = process::Entity::find()
            .filter(process::Column::OrderId.is_in(order_ids.clone()))
            .order_by_asc(process::Column::Name)
            .all(&self.db)
            .await
            .map_err(AppError::from)?;

        // 批量查询已完成数量
        let process_ids: Vec<Uuid> = processes.iter().map(|p| p.id).collect();
        let completed_sums: Vec<(Uuid, Option<i64>)> = piece_record::Entity::find()
            .select_only()
            .column(piece_record::Column::ProcessId)
            .column_as(piece_record::Column::Quantity.sum(), "sum")
            .filter(piece_record::Column::ProcessId.is_in(process_ids))
            .filter(
                piece_record::Column::Status
                    .is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]),
            )
            .group_by(piece_record::Column::ProcessId)
            .into_tuple()
            .all(&self.db)
            .await
            .map_err(AppError::from)?;

        let completed_map: HashMap<Uuid, i64> = completed_sums
            .into_iter()
            .map(|(id, sum)| (id, sum.unwrap_or(0)))
            .collect();

        // 组装结果
        let list = processes
            .into_iter()
            .filter_map(|p| {
                let ord = orders_map.get(&p.order_id)?;
                let completed = completed_map.get(&p.id).copied().unwrap_or(0);
                let remaining = ord.quantity - completed as i32;

                if remaining <= 0 {
                    return None;
                }

                let order_image = ord.images.as_ref().and_then(|imgs| {
                    imgs.as_array()
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                });

                Some(AvailableTask {
                    process_id: p.id,
                    process_name: p.name,
                    piece_price: p.piece_price,
                    order_id: p.order_id,
                    order_name: ord.product_name.clone(),
                    order_image,
                    remaining_quantity: remaining,
                })
            })
            .collect();

        Ok(Json(AvailableTasksResponse { list }))
    }
}

#[tool_handler]
impl ServerHandler for StaffMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "服装加工管理助手（员工端）。\n\
                可用工具：\n\
                - get_my_records: 查询我的计件记录\n\
                - get_my_earnings: 查询我的收入统计\n\
                - get_my_payrolls: 查询我的工资单\n\
                - get_available_tasks: 查看可接工序"
                    .into(),
            ),
        }
    }
}
