use chrono::{Datelike, Utc};
use sea_orm::{
    ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::common::ListData;
use entity::{order, piece_record, register_code, user, user::Role, workshop};
use crate::error::Result;

use super::dto::{AdminQueryParams, AdminStats, UserListItem};

pub async fn get_stats(db: &DbConn) -> Result<AdminStats> {
    let now = Utc::now();
    let today_start = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    let week_start = now
        .date_naive()
        .week(chrono::Weekday::Mon)
        .first_day()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    let month_start = now
        .date_naive()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    // User stats
    let total_users = user::Entity::find().count(db).await? as i64;
    let boss_count = user::Entity::find()
        .filter(user::Column::Role.eq(Role::Boss))
        .count(db)
        .await? as i64;
    let staff_count = user::Entity::find()
        .filter(user::Column::Role.eq(Role::Staff))
        .count(db)
        .await? as i64;
    let today_new_users = user::Entity::find()
        .filter(user::Column::CreatedAt.gte(today_start))
        .count(db)
        .await? as i64;
    let week_new_users = user::Entity::find()
        .filter(user::Column::CreatedAt.gte(week_start))
        .count(db)
        .await? as i64;
    let month_new_users = user::Entity::find()
        .filter(user::Column::CreatedAt.gte(month_start))
        .count(db)
        .await? as i64;

    // Workshop stats
    let total_workshops = workshop::Entity::find().count(db).await? as i64;
    // Active workshops: workshops that have orders
    let active_workshops = order::Entity::find()
        .select_only()
        .column(order::Column::BossId)
        .distinct()
        .count(db)
        .await? as i64;

    // Register code stats
    let total_codes = register_code::Entity::find().count(db).await? as i64;
    let used_codes = register_code::Entity::find()
        .filter(register_code::Column::UsedBy.is_not_null())
        .count(db)
        .await? as i64;
    let available_codes = register_code::Entity::find()
        .filter(register_code::Column::UsedBy.is_null())
        .count(db)
        .await? as i64;

    // Platform activity
    let today_orders = order::Entity::find()
        .filter(order::Column::ReceivedAt.gte(today_start))
        .count(db)
        .await? as i64;
    let month_orders = order::Entity::find()
        .filter(order::Column::ReceivedAt.gte(month_start))
        .count(db)
        .await? as i64;
    let today_records = piece_record::Entity::find()
        .filter(piece_record::Column::RecordedAt.gte(today_start))
        .count(db)
        .await? as i64;
    let month_records = piece_record::Entity::find()
        .filter(piece_record::Column::RecordedAt.gte(month_start))
        .count(db)
        .await? as i64;

    Ok(AdminStats {
        total_users,
        boss_count,
        staff_count,
        today_new_users,
        week_new_users,
        month_new_users,
        total_workshops,
        active_workshops,
        total_codes,
        used_codes,
        available_codes,
        today_orders,
        month_orders,
        today_records,
        month_records,
    })
}

pub async fn list_users(db: &DbConn, params: AdminQueryParams) -> Result<ListData<UserListItem>> {
    let paginator = user::Entity::find()
        .order_by_desc(user::Column::CreatedAt)
        .paginate(db, params.page_size);

    let total = paginator.num_items().await?;
    let users = paginator.fetch_page(params.page - 1).await?;

    let list = users
        .into_iter()
        .map(|u| UserListItem {
            id: u.id,
            username: u.username,
            role: u.role,
            display_name: u.display_name,
            phone: u.phone,
            avatar: u.avatar,
            is_super_admin: u.is_super_admin,
            created_at: u.created_at,
        })
        .collect();

    Ok(ListData { list, total })
}
