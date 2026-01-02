use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;

use super::dto::{
    CustomerSummary, CustomerSummaryList, OrderStats, ProcessProgress, WorkerProduction,
    WorkerProductionList, WorkerStatsParams,
};
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

    let processes = process::Entity::find()
        .filter(process::Column::OrderId.eq(order_id))
        .all(db)
        .await?;

    let mut process_stats = Vec::new();
    let mut total_completed: i64 = 0;

    for proc in processes {
        let qty: Option<i64> = piece_record::Entity::find()
            .select_only()
            .column_as(piece_record::Column::Quantity.sum(), "sum")
            .filter(piece_record::Column::ProcessId.eq(proc.id))
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
        let orders = order::Entity::find()
            .filter(order::Column::CustomerId.eq(cust.id))
            .all(db)
            .await?;

        let total = orders.len() as i64;
        let pending = orders.iter().filter(|o| o.status == "pending").count() as i64;
        let processing = orders.iter().filter(|o| o.status == "processing").count() as i64;
        let completed = orders
            .iter()
            .filter(|o| o.status == "completed" || o.status == "delivered")
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
    let mut query = piece_record::Entity::find().filter(piece_record::Column::BossId.eq(boss_id));

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query.filter(
                piece_record::Column::RecordedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()),
            );
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
