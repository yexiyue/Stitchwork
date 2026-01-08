use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, DbConn, EntityTrait, LoaderTrait, ModelTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use uuid::Uuid;

use super::dto::{
    CustomerContribution, CustomerContributionList, CustomerSummary, CustomerSummaryList,
    DailyOrderStat, DailyOrderStatsList, DailyStat, DailyStatsList, GroupStat, GroupStatsList,
    MonthlyOrderStat, MonthlyOrderStatsList, OrderOverview, OrderProgressItem, OrderProgressList,
    OrderStats, OrderStatsParams, ProcessProgress, WorkerProduction, WorkerProductionList,
    WorkerStatsParams,
};
use crate::common::{apply_date_filter, OwnedByBoss};
use crate::entity::order::OrderStatus;
use crate::entity::piece_record::PieceRecordStatus;
use crate::entity::{customer, order, piece_record, process, user};
use crate::error::{AppError, Result};

pub async fn order_stats(db: &DbConn, order_id: Uuid, boss_id: Uuid) -> Result<OrderStats> {
    let ord = order::Entity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", order_id)))?;
    ord.verify_owner(boss_id)?;

    let processes = ord.find_related(process::Entity).all(db).await?;

    // Batch query: get completed quantities for all processes at once (avoid N+1)
    use std::collections::HashMap;
    let process_ids: Vec<Uuid> = processes.iter().map(|p| p.id).collect();
    let completed_sums: Vec<(Uuid, Option<i64>)> = piece_record::Entity::find()
        .select_only()
        .column(piece_record::Column::ProcessId)
        .column_as(piece_record::Column::Quantity.sum(), "sum")
        .filter(piece_record::Column::ProcessId.is_in(process_ids))
        .filter(piece_record::Column::Status.is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]))
        .group_by(piece_record::Column::ProcessId)
        .into_tuple()
        .all(db)
        .await?;
    let completed_by_process: HashMap<Uuid, i64> = completed_sums
        .into_iter()
        .map(|(id, sum)| (id, sum.unwrap_or(0)))
        .collect();

    let mut process_stats = Vec::new();
    let mut total_completed: i64 = 0;

    for proc in processes {
        let completed = completed_by_process.get(&proc.id).copied().unwrap_or(0);
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

    // Use LoaderTrait for batch loading (SeaORM 2.0)
    let customer_orders: Vec<Vec<order::Model>> = customers.load_many(order::Entity, db).await?;

    let list = customers
        .into_iter()
        .zip(customer_orders.into_iter())
        .map(|(cust, orders)| {
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

            CustomerSummary {
                customer_id: cust.id,
                customer_name: cust.name,
                total_orders: total,
                pending_orders: pending,
                processing_orders: processing,
                completed_orders: completed,
            }
        })
        .collect();

    Ok(CustomerSummaryList { list })
}

pub async fn worker_production(
    db: &DbConn,
    boss_id: Uuid,
    params: WorkerStatsParams,
) -> Result<WorkerProductionList> {
    let query = piece_record::Entity::find()
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]));

    let query = apply_date_filter(
        query,
        piece_record::Column::RecordedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let records = query.all(db).await?;

    // Group by user_id
    use std::collections::HashMap;
    let mut user_stats: HashMap<Uuid, (i64, Decimal)> = HashMap::new();
    for rec in &records {
        let entry = user_stats.entry(rec.user_id).or_insert((0, Decimal::ZERO));
        entry.0 += rec.quantity as i64;
        entry.1 += rec.amount;
    }

    // Batch load all users at once (avoid N+1 query)
    let user_ids: Vec<Uuid> = user_stats.keys().copied().collect();
    let users: HashMap<Uuid, user::Model> = user::Entity::find()
        .filter(user::Column::Id.is_in(user_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|u| (u.id, u))
        .collect();

    let mut list = Vec::new();
    for (user_id, (qty, amt)) in user_stats {
        let name = users
            .get(&user_id)
            .map(|u| u.display_name.clone().unwrap_or_else(|| u.username.clone()))
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
        .filter(piece_record::Column::Status.is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]));

    // For boss, filter by boss_id; for staff, filter by user_id
    if let Some(bid) = boss_id {
        query = query.filter(piece_record::Column::BossId.eq(bid));
    }
    if let Some(uid) = user_id {
        query = query.filter(piece_record::Column::UserId.eq(uid));
    }

    let query = apply_date_filter(
        query,
        piece_record::Column::RecordedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

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
        .filter(piece_record::Column::Status.is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]));

    if let Some(bid) = boss_id {
        query = query.filter(piece_record::Column::BossId.eq(bid));
    }
    if let Some(uid) = user_id {
        query = query.filter(piece_record::Column::UserId.eq(uid));
    }

    let query = apply_date_filter(
        query,
        piece_record::Column::RecordedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let records = query.all(db).await?;

    // Batch load all processes at once (avoid N+1 query)
    let process_ids: Vec<Uuid> = records.iter().map(|r| r.process_id).collect();
    let processes: HashMap<Uuid, process::Model> = process::Entity::find()
        .filter(process::Column::Id.is_in(process_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.id, p))
        .collect();

    // Group by order_id via process
    let mut order_map: HashMap<Uuid, (i64, Decimal)> = HashMap::new();
    for rec in &records {
        if let Some(proc) = processes.get(&rec.process_id) {
            let entry = order_map.entry(proc.order_id).or_insert((0, Decimal::ZERO));
            entry.0 += rec.quantity as i64;
            entry.1 += rec.amount;
        }
    }

    // Batch load all orders at once (avoid N+1 query)
    let order_ids: Vec<Uuid> = order_map.keys().copied().collect();
    let orders: HashMap<Uuid, order::Model> = order::Entity::find()
        .filter(order::Column::Id.is_in(order_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|o| (o.id, o))
        .collect();

    let mut list = Vec::new();
    for (order_id, (qty, amt)) in order_map {
        let name = orders
            .get(&order_id)
            .map(|o| o.product_name.clone())
            .unwrap_or_default();
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
        .filter(piece_record::Column::Status.is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]));

    if let Some(bid) = boss_id {
        query = query.filter(piece_record::Column::BossId.eq(bid));
    }
    if let Some(uid) = user_id {
        query = query.filter(piece_record::Column::UserId.eq(uid));
    }

    let query = apply_date_filter(
        query,
        piece_record::Column::RecordedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

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

    // Batch load all processes at once (avoid N+1 query)
    let process_ids: Vec<Uuid> = process_map.keys().copied().collect();
    let processes: HashMap<Uuid, process::Model> = process::Entity::find()
        .filter(process::Column::Id.is_in(process_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.id, p))
        .collect();

    let mut list = Vec::new();
    for (process_id, (qty, amt)) in process_map {
        let name = processes
            .get(&process_id)
            .map(|p| p.name.clone())
            .unwrap_or_default();
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

// ============ Order Stats ============

/// 订单概览统计
pub async fn order_overview(
    db: &DbConn,
    boss_id: Uuid,
    params: OrderStatsParams,
) -> Result<OrderOverview> {
    let query = order::Entity::find().filter(order::Column::BossId.eq(boss_id));
    let query = apply_date_filter(
        query,
        order::Column::ReceivedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let orders = query.all(db).await?;

    let total_orders = orders.len() as i64;
    let pending_orders = orders.iter().filter(|o| o.status == OrderStatus::Pending).count() as i64;
    let processing_orders = orders
        .iter()
        .filter(|o| o.status == OrderStatus::Processing)
        .count() as i64;
    let completed_orders = orders
        .iter()
        .filter(|o| o.status == OrderStatus::Completed)
        .count() as i64;
    let delivered_orders = orders
        .iter()
        .filter(|o| o.status == OrderStatus::Delivered)
        .count() as i64;
    let cancelled_orders = orders
        .iter()
        .filter(|o| o.status == OrderStatus::Cancelled)
        .count() as i64;

    let total_quantity: i64 = orders.iter().map(|o| o.quantity as i64).sum();
    let total_amount: Decimal = orders
        .iter()
        .map(|o| Decimal::from(o.quantity) * o.unit_price)
        .sum();

    Ok(OrderOverview {
        total_orders,
        pending_orders,
        processing_orders,
        completed_orders,
        delivered_orders,
        cancelled_orders,
        total_quantity,
        total_amount,
    })
}

/// 月度订单趋势
pub async fn monthly_order_stats(
    db: &DbConn,
    boss_id: Uuid,
    params: OrderStatsParams,
) -> Result<MonthlyOrderStatsList> {
    use std::collections::BTreeMap;

    let query = order::Entity::find().filter(order::Column::BossId.eq(boss_id));
    let query = apply_date_filter(
        query,
        order::Column::ReceivedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let orders = query.all(db).await?;

    // Group by month
    let mut monthly_map: BTreeMap<String, (i64, i64, Decimal)> = BTreeMap::new();
    for ord in &orders {
        let month_str = ord.received_at.format("%Y-%m").to_string();
        let entry = monthly_map
            .entry(month_str)
            .or_insert((0, 0, Decimal::ZERO));
        entry.0 += 1; // order count
        entry.1 += ord.quantity as i64; // total quantity
        entry.2 += Decimal::from(ord.quantity) * ord.unit_price; // total amount
    }

    let list = monthly_map
        .into_iter()
        .map(|(month, (count, qty, amt))| MonthlyOrderStat {
            month,
            order_count: count,
            total_quantity: qty,
            total_amount: amt,
        })
        .collect();

    Ok(MonthlyOrderStatsList { list })
}

/// 客户贡献度
pub async fn customer_contribution(
    db: &DbConn,
    boss_id: Uuid,
    params: OrderStatsParams,
) -> Result<CustomerContributionList> {
    let customers = customer::Entity::find()
        .filter(customer::Column::UserId.eq(boss_id))
        .all(db)
        .await?;

    let mut list = Vec::new();
    for cust in customers {
        let query = order::Entity::find().filter(order::Column::CustomerId.eq(cust.id));
        let query = apply_date_filter(
            query,
            order::Column::ReceivedAt,
            params.start_date.as_deref(),
            params.end_date.as_deref(),
        );

        let orders = query.all(db).await?;

        if orders.is_empty() {
            continue;
        }

        let order_count = orders.len() as i64;
        let total_quantity: i64 = orders.iter().map(|o| o.quantity as i64).sum();
        let total_amount: Decimal = orders
            .iter()
            .map(|o| Decimal::from(o.quantity) * o.unit_price)
            .sum();

        list.push(CustomerContribution {
            customer_id: cust.id,
            customer_name: cust.name,
            order_count,
            total_quantity,
            total_amount,
        });
    }

    // Sort by total_amount descending
    list.sort_by(|a, b| b.total_amount.cmp(&a.total_amount));

    Ok(CustomerContributionList { list })
}

/// 订单进度概览
pub async fn order_progress(
    db: &DbConn,
    boss_id: Uuid,
    params: OrderStatsParams,
) -> Result<OrderProgressList> {
    let query = order::Entity::find()
        .filter(order::Column::BossId.eq(boss_id))
        // Only show active orders (not delivered/cancelled)
        .filter(order::Column::Status.is_in([OrderStatus::Pending, OrderStatus::Processing]))
        .order_by_desc(order::Column::ReceivedAt);

    let query = apply_date_filter(
        query,
        order::Column::ReceivedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let orders = query.all(db).await?;

    // Use LoaderTrait for batch loading (SeaORM 2.0)
    let customers: Vec<Option<customer::Model>> = orders.load_one(customer::Entity, db).await?;
    let order_processes: Vec<Vec<process::Model>> = orders.load_many(process::Entity, db).await?;

    // Batch load completed quantities for all processes at once (GROUP BY aggregation)
    use std::collections::HashMap;
    let all_process_ids: Vec<Uuid> = order_processes.iter().flatten().map(|p| p.id).collect();
    let completed_sums: Vec<(Uuid, Option<i64>)> = piece_record::Entity::find()
        .select_only()
        .column(piece_record::Column::ProcessId)
        .column_as(piece_record::Column::Quantity.sum(), "sum")
        .filter(piece_record::Column::ProcessId.is_in(all_process_ids))
        .filter(piece_record::Column::Status.is_in([PieceRecordStatus::Approved, PieceRecordStatus::Settled]))
        .group_by(piece_record::Column::ProcessId)
        .into_tuple()
        .all(db)
        .await?;
    let completed_by_process: HashMap<Uuid, i64> = completed_sums
        .into_iter()
        .map(|(id, sum)| (id, sum.unwrap_or(0)))
        .collect();

    let mut list = Vec::new();
    for ((ord, cust), procs) in orders
        .into_iter()
        .zip(customers.into_iter())
        .zip(order_processes.into_iter())
    {
        let customer_name = cust.map(|c| c.name).unwrap_or_default();

        // Calculate completed quantity from pre-loaded data
        let completed_quantity: i64 = procs
            .iter()
            .map(|p| completed_by_process.get(&p.id).copied().unwrap_or(0))
            .sum();

        let progress = if ord.quantity > 0 {
            (completed_quantity as f64) / (ord.quantity as f64)
        } else {
            0.0
        };

        list.push(OrderProgressItem {
            order_id: ord.id,
            product_name: ord.product_name,
            customer_name,
            total_quantity: ord.quantity,
            completed_quantity,
            progress,
            status: ord.status.to_string(),
        });
    }

    Ok(OrderProgressList { list })
}

/// 每日订单趋势
pub async fn daily_order_stats(
    db: &DbConn,
    boss_id: Uuid,
    params: OrderStatsParams,
) -> Result<DailyOrderStatsList> {
    use std::collections::BTreeMap;

    let query = order::Entity::find().filter(order::Column::BossId.eq(boss_id));
    let query = apply_date_filter(
        query,
        order::Column::ReceivedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let orders = query.all(db).await?;

    // Group by date
    let mut daily_map: BTreeMap<String, (i64, i64, Decimal)> = BTreeMap::new();
    for ord in &orders {
        let date_str = ord.received_at.format("%Y-%m-%d").to_string();
        let entry = daily_map
            .entry(date_str)
            .or_insert((0, 0, Decimal::ZERO));
        entry.0 += 1; // order count
        entry.1 += ord.quantity as i64; // total quantity
        entry.2 += Decimal::from(ord.quantity) * ord.unit_price; // total amount
    }

    let list = daily_map
        .into_iter()
        .map(|(date, (count, qty, amt))| DailyOrderStat {
            date,
            order_count: count,
            total_quantity: qty,
            total_amount: amt,
        })
        .collect();

    Ok(DailyOrderStatsList { list })
}
