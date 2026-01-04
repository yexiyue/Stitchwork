use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, ModelTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

use super::dto::{
    CustomerSummary, CustomerSummaryList, DailyStat, DailyStatsList, GroupStat, GroupStatsList,
    OrderStats, ProcessProgress, WorkerProduction, WorkerProductionList, WorkerStatsParams,
};
use crate::entity::order::OrderStatus;
use crate::entity::piece_record::PieceRecordStatus;
use crate::entity::{customer, order, piece_record, process, user};
use crate::error::{AppError, Result};

pub async fn order_stats(db: &DbConn, order_id: Uuid, boss_id: Uuid) -> Result<OrderStats> {
    let ord = order::Entity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", order_id)))?;
    if ord.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }

    let processes = ord.find_related(process::Entity).all(db).await?;

    let mut process_stats = Vec::new();
    let mut total_completed: i64 = 0;

    for proc in processes {
        let qty: Option<i64> = piece_record::Entity::find()
            .select_only()
            .column_as(piece_record::Column::Quantity.sum(), "sum")
            .filter(piece_record::Column::ProcessId.eq(proc.id))
            .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved))
            .into_tuple()
            .one(db)
            .await?;
        let completed = qty.unwrap_or(0);
        total_completed += completed;
        process_stats.push(ProcessProgress {
            process_id: proc.id,
            name: proc.name,
            completed_quantity: completed,
        });
    }

    let progress = if ord.quantity > 0 {
        (total_completed as f64) / (ord.quantity as f64)
    } else {
        0.0
    };

    Ok(OrderStats {
        order_id,
        total_quantity: ord.quantity,
        completed_quantity: total_completed,
        progress,
        processes: process_stats,
    })
}

pub async fn customer_summary(db: &DbConn, boss_id: Uuid) -> Result<CustomerSummaryList> {
    let customers = customer::Entity::find()
        .filter(customer::Column::UserId.eq(boss_id))
        .all(db)
        .await?;

    let mut list = Vec::new();
    for cust in customers {
        let orders = cust.find_related(order::Entity).all(db).await?;

        let total = orders.len() as i64;
        let pending = orders
            .iter()
            .filter(|o| o.status == OrderStatus::Pending)
            .count() as i64;
        let processing = orders
            .iter()
            .filter(|o| o.status == OrderStatus::Processing)
            .count() as i64;
        let completed = orders
            .iter()
            .filter(|o| o.status == OrderStatus::Completed || o.status == OrderStatus::Delivered)
            .count() as i64;

        list.push(CustomerSummary {
            customer_id: cust.id,
            customer_name: cust.name,
            total_orders: total,
            pending_orders: pending,
            processing_orders: processing,
            completed_orders: completed,
        });
    }

    Ok(CustomerSummaryList { list })
}

pub async fn worker_production(
    db: &DbConn,
    boss_id: Uuid,
    params: WorkerStatsParams,
) -> Result<WorkerProductionList> {
    let mut query = piece_record::Entity::find()
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved));

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query
                .filter(piece_record::Column::RecordedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(
                piece_record::Column::RecordedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()),
            );
        }
    }

    let records = query.all(db).await?;

    // Group by user_id
    use std::collections::HashMap;
    let mut user_stats: HashMap<Uuid, (i64, Decimal)> = HashMap::new();
    for rec in &records {
        let entry = user_stats.entry(rec.user_id).or_insert((0, Decimal::ZERO));
        entry.0 += rec.quantity as i64;
        entry.1 += rec.amount;
    }

    let mut list = Vec::new();
    for (user_id, (qty, amt)) in user_stats {
        let u = user::Entity::find_by_id(user_id).one(db).await?;
        let name = u
            .map(|u| u.display_name.unwrap_or(u.username))
            .unwrap_or_default();
        list.push(WorkerProduction {
            user_id,
            user_name: name,
            total_quantity: qty,
            total_amount: amt,
        });
    }

    Ok(WorkerProductionList { list })
}

/// Daily stats for trend chart
/// For boss: returns aggregated stats for all workers
/// For staff: returns stats for the individual user
pub async fn daily_stats(
    db: &DbConn,
    boss_id: Option<Uuid>,
    user_id: Option<Uuid>,
    params: WorkerStatsParams,
) -> Result<DailyStatsList> {
    use std::collections::BTreeMap;

    let mut query = piece_record::Entity::find()
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved));

    // For boss, filter by boss_id; for staff, filter by user_id
    if let Some(bid) = boss_id {
        query = query.filter(piece_record::Column::BossId.eq(bid));
    }
    if let Some(uid) = user_id {
        query = query.filter(piece_record::Column::UserId.eq(uid));
    }

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query
                .filter(piece_record::Column::RecordedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(
                piece_record::Column::RecordedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()),
            );
        }
    }

    let records = query.all(db).await?;

    // Group by date
    let mut daily_map: BTreeMap<String, (i64, Decimal)> = BTreeMap::new();
    for rec in &records {
        let date_str = rec.recorded_at.format("%Y-%m-%d").to_string();
        let entry = daily_map.entry(date_str).or_insert((0, Decimal::ZERO));
        entry.0 += rec.quantity as i64;
        entry.1 += rec.amount;
    }

    let list = daily_map
        .into_iter()
        .map(|(date, (qty, amt))| DailyStat {
            date,
            total_quantity: qty,
            total_amount: amt,
        })
        .collect();

    Ok(DailyStatsList { list })
}

/// Stats grouped by order
pub async fn stats_by_order(
    db: &DbConn,
    boss_id: Option<Uuid>,
    user_id: Option<Uuid>,
    params: WorkerStatsParams,
) -> Result<GroupStatsList> {
    use std::collections::HashMap;

    let mut query = piece_record::Entity::find()
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved));

    if let Some(bid) = boss_id {
        query = query.filter(piece_record::Column::BossId.eq(bid));
    }
    if let Some(uid) = user_id {
        query = query.filter(piece_record::Column::UserId.eq(uid));
    }

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query
                .filter(piece_record::Column::RecordedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(
                piece_record::Column::RecordedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()),
            );
        }
    }

    let records = query.all(db).await?;

    // Get order_id via process_id, then group by order_id
    let mut order_map: HashMap<Uuid, (i64, Decimal)> = HashMap::new();
    for rec in &records {
        // Get process to find order_id
        let proc = process::Entity::find_by_id(rec.process_id).one(db).await?;
        if let Some(p) = proc {
            let entry = order_map.entry(p.order_id).or_insert((0, Decimal::ZERO));
            entry.0 += rec.quantity as i64;
            entry.1 += rec.amount;
        }
    }

    let mut list = Vec::new();
    for (order_id, (qty, amt)) in order_map {
        let ord = order::Entity::find_by_id(order_id).one(db).await?;
        let name = ord.map(|o| o.product_name).unwrap_or_default();
        list.push(GroupStat {
            id: order_id,
            name,
            total_quantity: qty,
            total_amount: amt,
        });
    }

    // Sort by quantity descending
    list.sort_by(|a, b| b.total_quantity.cmp(&a.total_quantity));

    Ok(GroupStatsList { list })
}

/// Stats grouped by process
pub async fn stats_by_process(
    db: &DbConn,
    boss_id: Option<Uuid>,
    user_id: Option<Uuid>,
    params: WorkerStatsParams,
) -> Result<GroupStatsList> {
    use std::collections::HashMap;

    let mut query = piece_record::Entity::find()
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved));

    if let Some(bid) = boss_id {
        query = query.filter(piece_record::Column::BossId.eq(bid));
    }
    if let Some(uid) = user_id {
        query = query.filter(piece_record::Column::UserId.eq(uid));
    }

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query
                .filter(piece_record::Column::RecordedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(
                piece_record::Column::RecordedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()),
            );
        }
    }

    let records = query.all(db).await?;

    // Group by process_id
    let mut process_map: HashMap<Uuid, (i64, Decimal)> = HashMap::new();
    for rec in &records {
        let entry = process_map
            .entry(rec.process_id)
            .or_insert((0, Decimal::ZERO));
        entry.0 += rec.quantity as i64;
        entry.1 += rec.amount;
    }

    let mut list = Vec::new();
    for (process_id, (qty, amt)) in process_map {
        let proc = process::Entity::find_by_id(process_id).one(db).await?;
        let name = proc.map(|p| p.name).unwrap_or_default();
        list.push(GroupStat {
            id: process_id,
            name,
            total_quantity: qty,
            total_amount: amt,
        });
    }

    // Sort by quantity descending
    list.sort_by(|a, b| b.total_quantity.cmp(&a.total_quantity));

    Ok(GroupStatsList { list })
}
