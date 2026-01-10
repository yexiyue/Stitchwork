use crate::error::AppError;
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{ErrorData, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, Json, ServerHandler,
};
use schemars::JsonSchema;
use sea_orm::DbConn;
use serde::Deserialize;
use uuid::Uuid;

use super::AuthClaims;

// 复用 server 的 DTO 和 service
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

// ============ 工具参数定义 ============

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrdersParams {
    #[schemars(description = "按客户ID筛选")]
    pub customer_id: Option<Uuid>,
    #[schemars(
        description = "按状态筛选，多个状态用逗号分隔: pending,processing,completed,delivered,cancelled"
    )]
    pub status: Option<String>,
    #[schemars(description = "搜索关键词（产品名称）")]
    pub search: Option<String>,
    #[schemars(description = "开始日期，格式 YYYY-MM-DD")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD")]
    pub end_date: Option<String>,
    #[schemars(description = "页码，从1开始，默认1")]
    pub page: Option<u64>,
    #[schemars(description = "每页数量，默认20")]
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryPieceRecordsParams {
    #[schemars(description = "按状态筛选，多个状态用逗号分隔: pending,approved,rejected")]
    pub status: Option<String>,
    #[schemars(description = "开始日期，格式 YYYY-MM-DD")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD")]
    pub end_date: Option<String>,
    #[schemars(description = "页码，从1开始，默认1")]
    pub page: Option<u64>,
    #[schemars(description = "每页数量，默认20")]
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkerStatsParams {
    #[schemars(description = "开始日期，格式 YYYY-MM-DD")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD")]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderProgressParams {
    #[schemars(description = "开始日期，格式 YYYY-MM-DD")]
    pub start_date: Option<String>,
    #[schemars(description = "结束日期，格式 YYYY-MM-DD")]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUnpaidSummaryParams {
    #[schemars(description = "员工ID，不传则查询所有员工")]
    pub user_id: Option<Uuid>,
}

// ============ 响应类型 ============

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResponse {
    pub list: Vec<entity::order::Model>,
    pub total: u64,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PieceRecordListResponse {
    pub list: Vec<PieceRecordResponse>,
    pub total: u64,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnpaidSummaryItem {
    pub user_id: Uuid,
    pub user_name: String,
    pub total_quantity: i64,
    pub total_amount: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnpaidSummaryResponse {
    pub list: Vec<UnpaidSummaryItem>,
}

// ============ MCP 工具实现 ============

pub struct BossMcp {
    db: DbConn,
    pub tool_router: ToolRouter<BossMcp>,
}

#[tool_router]
impl BossMcp {
    pub fn new(db: DbConn) -> Self {
        Self {
            db,
            tool_router: Self::tool_router(),
        }
    }

    /// 查询订单列表
    #[tool(description = "查询订单列表，支持按客户、状态、日期范围筛选")]
    pub async fn query_orders(
        &self,
        Parameters(params): Parameters<QueryOrdersParams>,
        AuthClaims(claims): AuthClaims,
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
        let filter = OrderQueryParams {
            customer_id: params.customer_id,
        };

        let result = order_service::list(&self.db, query_params, filter, &claims).await?;
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
        AuthClaims(claims): AuthClaims,
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

        let result = piece_record_service::list(&self.db, query_params, &claims).await?;
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
        AuthClaims(claims): AuthClaims,
    ) -> Result<Json<WorkerProductionList>, ErrorData> {
        let stats_params = WorkerStatsParams {
            start_date: params.start_date,
            end_date: params.end_date,
        };

        let result = stats_service::worker_production(&self.db, claims.sub, stats_params).await?;
        Ok(Json(result))
    }

    /// 获取首页概览数据
    #[tool(description = "获取老板首页概览数据，包括待审批数、进行中订单数、今日/本月产量和金额")]
    pub async fn get_overview(
        &self,
        AuthClaims(claims): AuthClaims,
    ) -> Result<Json<BossOverview>, ErrorData> {
        let result = home_service::boss_overview(&self.db, claims.sub).await?;
        Ok(Json(result))
    }

    /// 获取订单进度列表
    #[tool(description = "获取进行中订单的进度列表，包括完成数量和进度百分比")]
    pub async fn get_order_progress(
        &self,
        Parameters(params): Parameters<GetOrderProgressParams>,
        AuthClaims(claims): AuthClaims,
    ) -> Result<Json<OrderProgressList>, ErrorData> {
        let stats_params = OrderStatsParams {
            start_date: params.start_date,
            end_date: params.end_date,
        };

        let result = stats_service::order_progress(&self.db, claims.sub, stats_params).await?;
        Ok(Json(result))
    }

    /// 获取待发工资汇总
    #[tool(description = "获取待发工资汇总，统计已批准但未结算的计件记录")]
    pub async fn get_unpaid_summary(
        &self,
        Parameters(params): Parameters<GetUnpaidSummaryParams>,
        AuthClaims(claims): AuthClaims,
    ) -> Result<Json<UnpaidSummaryResponse>, ErrorData> {
        use entity::piece_record::{self, PieceRecordStatus};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
        use std::collections::HashMap;

        let mut query = piece_record::Entity::find()
            .filter(piece_record::Column::BossId.eq(claims.sub))
            .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved));

        if let Some(uid) = params.user_id {
            query = query.filter(piece_record::Column::UserId.eq(uid));
        }

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

        let list = user_stats
            .into_iter()
            .map(|(user_id, (qty, amt))| {
                let name = users
                    .get(&user_id)
                    .map(|u| u.display_name.clone().unwrap_or_else(|| u.username.clone()))
                    .unwrap_or_default();
                UnpaidSummaryItem {
                    user_id,
                    user_name: name,
                    total_quantity: qty,
                    total_amount: amt,
                }
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
