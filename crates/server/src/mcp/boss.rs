use super::deserialize_empty_string_as_none;
use crate::common::QueryParams;
use crate::service::{
    home::{dto::BossOverview, service as home_service},
    order::{dto::OrderQueryParams, service as order_service},
    piece_record::{dto::PieceRecordResponse, service as piece_record_service},
    stats::{
        dto::{OrderProgressList, OrderStatsParams, WorkerProductionList, WorkerStatsParams},
        service as stats_service,
    },
};
use crate::{error::AppError, service::auth::Claims};
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{ErrorData, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, Json, ServerHandler,
};
use schemars::JsonSchema;
use sea_orm::DbConn;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

// ============ 工具参数定义 ============

/// 查询订单参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrdersParams {
    /// 按状态筛选，多个状态用逗号分隔: pending,processing,completed,delivered,cancelled。不指定则返回所有状态
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<String>,
    /// 搜索关键词（产品名称或客户名称）。不指定则不筛选
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub search: Option<String>,
    /// 开始日期，格式 YYYY-MM-DD。不指定则不限制开始时间
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYY-MM-DD。不指定则不限制结束时间
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
    /// 页码，从1开始，默认1
    pub page: Option<u64>,
    /// 每页数量，默认20
    pub page_size: Option<u64>,
}

/// 查询计件记录参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryPieceRecordsParams {
    /// 按状态筛选，多个状态用逗号分隔: pending,approved,rejected。不指定则返回所有状态
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<String>,
    /// 开始日期，格式 YYYY-MM-DD。不指定则不限制
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYY-MM-DD。不指定则不限制
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
    /// 页码，从1开始，默认1
    pub page: Option<u64>,
    /// 每页数量，默认20
    pub page_size: Option<u64>,
}

/// 获取员工统计参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkerStatsParams {
    /// 开始日期，格式 YYYY-MM-DD。不指定则统计全部
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYY-MM-DD。不指定则统计全部
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
}

/// 获取订单进度参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderProgressParams {
    /// 开始日期，格式 YYYY-MM-DD。不指定则显示全部
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYY-MM-DD。不指定则显示全部
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub end_date: Option<String>,
}

/// 获取待发工资汇总参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUnpaidSummaryParams {
    /// 员工姓名关键词，不传则查询所有员工
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub user_name: Option<String>,
}

// ============ 响应类型 ============

/// 订单列表响应
#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResponse {
    /// 订单列表
    pub list: Vec<entity::order::Model>,
    /// 总记录数
    pub total: u64,
}

/// 计件记录列表响应
#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PieceRecordListResponse {
    /// 计件记录列表
    pub list: Vec<PieceRecordResponse>,
    /// 总记录数
    pub total: u64,
}

/// 待发工资汇总项
#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnpaidSummaryItem {
    /// 员工ID
    pub user_id: Uuid,
    /// 员工姓名
    pub user_name: String,
    /// 待发数量（件）
    pub total_quantity: i64,
    /// 待发金额（元）
    pub total_amount: rust_decimal::Decimal,
}

/// 待发工资汇总响应
#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnpaidSummaryResponse {
    /// 各员工待发工资汇总列表
    pub list: Vec<UnpaidSummaryItem>,
}

// ============ MCP 工具实现 ============

pub struct BossMcp {
    db: DbConn,
    claims: Claims,
    pub tool_router: ToolRouter<BossMcp>,
}

#[tool_router]
impl BossMcp {
    pub fn new(db: DbConn, claims: Claims) -> Self {
        Self {
            db,
            claims,
            tool_router: Self::tool_router(),
        }
    }

    /// 查询订单列表
    #[tool(description = "查询订单列表，支持按状态、日期范围、关键词筛选")]
    #[instrument(skip(self))]
    pub async fn query_orders(
        &self,
        Parameters(params): Parameters<QueryOrdersParams>,
    ) -> Result<Json<OrderListResponse>, ErrorData> {
        let query_params = QueryParams {
            page: params.page.unwrap_or(1),
            page_size: params.page_size.unwrap_or(20),
            status: params
                .status
                .map(|s| s.split(',').map(String::from).collect()),
            search: params.search,
            start_date: params.start_date,
            end_date: params.end_date,
            ..Default::default()
        };
        let filter = OrderQueryParams { customer_id: None };

        let result = order_service::list(&self.db, query_params, filter, &self.claims).await?;
        Ok(Json(OrderListResponse {
            list: result.list,
            total: result.total,
        }))
    }

    /// 查询计件记录列表
    #[tool(description = "查询计件记录列表，支持按状态、日期范围筛选")]
    pub async fn query_piece_records(
        &self,
        Parameters(params): Parameters<QueryPieceRecordsParams>,
    ) -> Result<Json<PieceRecordListResponse>, ErrorData> {
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
        Ok(Json(PieceRecordListResponse {
            list: result.list,
            total: result.total,
        }))
    }

    /// 获取员工产量统计
    #[tool(description = "获取员工产量统计，包括每个员工的总数量和总金额")]
    pub async fn get_worker_stats(
        &self,
        Parameters(params): Parameters<GetWorkerStatsParams>,
    ) -> Result<Json<WorkerProductionList>, ErrorData> {
        let stats_params = WorkerStatsParams {
            start_date: params.start_date,
            end_date: params.end_date,
        };

        let result =
            stats_service::worker_production(&self.db, self.claims.sub, stats_params).await?;
        Ok(Json(result))
    }

    /// 获取首页概览数据
    #[tool(description = "获取老板首页概览数据，包括待审批数、进行中订单数、今日/本月产量和金额")]
    pub async fn get_overview(&self) -> Result<Json<BossOverview>, ErrorData> {
        let result = home_service::boss_overview(&self.db, self.claims.sub).await?;
        Ok(Json(result))
    }

    /// 获取订单进度列表
    #[tool(description = "获取进行中订单的进度列表，包括完成数量和进度百分比")]
    pub async fn get_order_progress(
        &self,
        Parameters(params): Parameters<GetOrderProgressParams>,
    ) -> Result<Json<OrderProgressList>, ErrorData> {
        let stats_params = OrderStatsParams {
            start_date: params.start_date,
            end_date: params.end_date,
        };

        let result = stats_service::order_progress(&self.db, self.claims.sub, stats_params).await?;
        Ok(Json(result))
    }

    /// 获取待发工资汇总
    #[tool(description = "获取待发工资汇总，统计已批准但未结算的计件记录")]
    pub async fn get_unpaid_summary(
        &self,
        Parameters(params): Parameters<GetUnpaidSummaryParams>,
    ) -> Result<Json<UnpaidSummaryResponse>, ErrorData> {
        use entity::piece_record::{self, PieceRecordStatus};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
        use std::collections::HashMap;

        let query = piece_record::Entity::find()
            .filter(piece_record::Column::BossId.eq(self.claims.sub))
            .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved));

        let records = query.all(&self.db).await.map_err(AppError::from)?;

        // 按用户分组统计
        let mut user_stats: HashMap<Uuid, (i64, rust_decimal::Decimal)> = HashMap::new();
        for rec in &records {
            let entry = user_stats
                .entry(rec.user_id)
                .or_insert((0, rust_decimal::Decimal::ZERO));
            entry.0 += rec.quantity as i64;
            entry.1 += rec.amount;
        }

        // 批量获取用户信息
        let user_ids: Vec<Uuid> = user_stats.keys().copied().collect();
        let users: HashMap<Uuid, entity::user::Model> = entity::user::Entity::find()
            .filter(entity::user::Column::Id.is_in(user_ids))
            .all(&self.db)
            .await
            .map_err(AppError::from)?
            .into_iter()
            .map(|u| (u.id, u))
            .collect();

        // 按名称筛选
        let name_filter = params.user_name.map(|n| n.to_lowercase());

        let list = user_stats
            .into_iter()
            .filter_map(|(user_id, (qty, amt))| {
                let user = users.get(&user_id)?;
                let name = user
                    .display_name
                    .clone()
                    .unwrap_or_else(|| user.username.clone());

                // 如果有名称筛选，检查是否匹配
                if let Some(ref filter) = name_filter
                    && !name.to_lowercase().contains(filter) {
                        return None;
                    }

                Some(UnpaidSummaryItem {
                    user_id,
                    user_name: name,
                    total_quantity: qty,
                    total_amount: amt,
                })
            })
            .collect();
        Ok(Json(UnpaidSummaryResponse { list }))
    }
}

#[tool_handler]
impl ServerHandler for BossMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "服装加工管理助手（老板端）。\n\
                可用工具：\n\
                - query_orders: 查询订单列表\n\
                - query_piece_records: 查询计件记录\n\
                - get_worker_stats: 获取员工产量统计\n\
                - get_overview: 获取首页概览数据\n\
                - get_order_progress: 获取订单进度\n\
                - get_unpaid_summary: 获取待发工资汇总"
                    .into(),
            ),
        }
    }
}
