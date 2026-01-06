use chrono::{Datelike, NaiveDate, Utc};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
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

fn build_activity(rec: piece_record::ModelEx) -> Activity {
    let user_name = rec
        .user
        .as_ref()
        .map(|u| u.display_name.clone().unwrap_or_else(|| u.username.clone()))
        .unwrap_or_default();

    let proc = rec.process.as_ref();
    let ord = proc.and_then(|p| p.order.as_ref());

    let order_name = ord.map(|o| o.product_name.clone()).unwrap_or_default();
    let order_image = ord.and_then(|o| {
        o.images.as_ref().and_then(|imgs: &serde_json::Value| {
            imgs.as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
    });
    let process_name = proc.map(|p| p.name.clone()).unwrap_or_default();

    let activity_type = match rec.status {
        PieceRecordStatus::Pending => ActivityType::Submit,
        PieceRecordStatus::Approved => ActivityType::Approve,
        PieceRecordStatus::Rejected => ActivityType::Reject,
        PieceRecordStatus::Settled => ActivityType::Approve,
    };

    Activity {
        id: rec.id,
        activity_type,
        user_name,
        order_name,
        order_image,
        process_name,
        quantity: rec.quantity,
        created_at: rec.recorded_at,
    }
}

pub async fn boss_activities(db: &DbConn, boss_id: Uuid) -> Result<ActivityList> {
    let records: Vec<piece_record::ModelEx> = piece_record::Entity::load()
        .with(user::Entity)
        .with((process::Entity, order::Entity))
        .filter(piece_record::Column::BossId.eq(boss_id))
        .order_by_desc(piece_record::Column::RecordedAt)
        .all(db)
        .await?
        .into_iter()
        .take(10)
        .collect();

    let list = records.into_iter().map(build_activity).collect();
    Ok(ActivityList { list })
}

pub async fn staff_activities(db: &DbConn, user_id: Uuid) -> Result<ActivityList> {
    let records: Vec<piece_record::ModelEx> = piece_record::Entity::load()
        .with(user::Entity)
        .with((process::Entity, order::Entity))
        .filter(piece_record::Column::UserId.eq(user_id))
        .order_by_desc(piece_record::Column::RecordedAt)
        .all(db)
        .await?
        .into_iter()
        .take(5)
        .collect();

    let list = records.into_iter().map(build_activity).collect();
    Ok(ActivityList { list })
}
