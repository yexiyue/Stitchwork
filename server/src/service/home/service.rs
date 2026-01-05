use chrono::{Datelike, NaiveDate, Utc};
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, DbConn, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use uuid::Uuid;

use super::dto::{Activity, ActivityList, ActivityType, BossOverview, StaffOverview};
use crate::entity::order::OrderStatus;
use crate::entity::piece_record::PieceRecordStatus;
use crate::entity::{order, piece_record, process, user, workshop};
use crate::error::Result;

pub async fn boss_overview(db: &DbConn, boss_id: Uuid) -> Result<BossOverview> {
    // Pending piece records count
    let pending_count: i64 = piece_record::Entity::find()
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Pending))
        .count(db)
        .await? as i64;

    // Processing orders count
    let processing_order_count: i64 = order::Entity::find()
        .filter(order::Column::BossId.eq(boss_id))
        .filter(order::Column::Status.eq(OrderStatus::Processing))
        .count(db)
        .await? as i64;

    // Today's stats
    let today = Utc::now().date_naive();
    let today_start = today.and_hms_opt(0, 0, 0).unwrap();
    let today_end = today.and_hms_opt(23, 59, 59).unwrap();

    let today_records = piece_record::Entity::find()
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved))
        .filter(piece_record::Column::RecordedAt.gte(today_start))
        .filter(piece_record::Column::RecordedAt.lte(today_end))
        .all(db)
        .await?;

    let today_quantity: i64 = today_records.iter().map(|r| r.quantity as i64).sum();
    let today_amount: Decimal = today_records.iter().map(|r| r.amount).sum();

    // This month's stats
    let month_start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let month_records = piece_record::Entity::find()
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved))
        .filter(piece_record::Column::RecordedAt.gte(month_start))
        .all(db)
        .await?;

    let month_quantity: i64 = month_records.iter().map(|r| r.quantity as i64).sum();
    let month_amount: Decimal = month_records.iter().map(|r| r.amount).sum();

    // Staff count
    let ws = workshop::Entity::find()
        .filter(workshop::Column::OwnerId.eq(boss_id))
        .one(db)
        .await?;

    let staff_count = if let Some(w) = ws {
        user::Entity::find()
            .filter(user::Column::WorkshopId.eq(w.id))
            .count(db)
            .await? as i64
    } else {
        0
    };

    Ok(BossOverview {
        pending_count,
        processing_order_count,
        today_quantity,
        today_amount,
        month_quantity,
        month_amount,
        staff_count,
    })
}

pub async fn staff_overview(db: &DbConn, user_id: Uuid) -> Result<StaffOverview> {
    let today = Utc::now().date_naive();
    let month_start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let month_records = piece_record::Entity::find()
        .filter(piece_record::Column::UserId.eq(user_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Approved))
        .filter(piece_record::Column::RecordedAt.gte(month_start))
        .all(db)
        .await?;

    let month_quantity: i64 = month_records.iter().map(|r| r.quantity as i64).sum();
    let month_amount: Decimal = month_records.iter().map(|r| r.amount).sum();

    Ok(StaffOverview {
        month_quantity,
        month_amount,
    })
}

async fn build_activities(
    db: &DbConn,
    records: Vec<piece_record::Model>,
) -> Result<Vec<Activity>> {
    use std::collections::HashMap;

    if records.is_empty() {
        return Ok(Vec::new());
    }

    // Use LoaderTrait for direct relationships (SeaORM 2.0)
    let users: Vec<Option<user::Model>> = records.load_one(user::Entity, db).await?;
    let processes: Vec<Option<process::Model>> = records.load_one(process::Entity, db).await?;

    // For orders (indirect: records -> processes -> orders), use batch query
    let order_ids: Vec<Uuid> = processes
        .iter()
        .filter_map(|p| p.as_ref().map(|pr| pr.order_id))
        .collect();
    let orders: HashMap<Uuid, order::Model> = order::Entity::find()
        .filter(order::Column::Id.is_in(order_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|o| (o.id, o))
        .collect();

    let list = records
        .into_iter()
        .zip(users.into_iter())
        .zip(processes.into_iter())
        .map(|((rec, user), proc)| {
            let user_name = user
                .map(|u| u.display_name.unwrap_or(u.username))
                .unwrap_or_default();

            let (order_name, process_name) = if let Some(p) = proc {
                let order_name = orders
                    .get(&p.order_id)
                    .map(|o| o.product_name.clone())
                    .unwrap_or_default();
                (order_name, p.name)
            } else {
                (String::new(), String::new())
            };

            let activity_type = match rec.status {
                PieceRecordStatus::Pending => ActivityType::Submit,
                PieceRecordStatus::Approved => ActivityType::Approve,
                PieceRecordStatus::Rejected => ActivityType::Reject,
            };

            Activity {
                id: rec.id,
                activity_type,
                user_name,
                order_name,
                process_name,
                quantity: rec.quantity,
                created_at: rec.recorded_at,
            }
        })
        .collect();

    Ok(list)
}

pub async fn boss_activities(db: &DbConn, boss_id: Uuid) -> Result<ActivityList> {
    let records = piece_record::Entity::find()
        .filter(piece_record::Column::BossId.eq(boss_id))
        .order_by_desc(piece_record::Column::RecordedAt)
        .limit(10)
        .all(db)
        .await?;

    let list = build_activities(db, records).await?;
    Ok(ActivityList { list })
}

pub async fn staff_activities(db: &DbConn, user_id: Uuid) -> Result<ActivityList> {
    let records = piece_record::Entity::find()
        .filter(piece_record::Column::UserId.eq(user_id))
        .order_by_desc(piece_record::Column::RecordedAt)
        .limit(5)
        .all(db)
        .await?;

    let list = build_activities(db, records).await?;
    Ok(ActivityList { list })
}
