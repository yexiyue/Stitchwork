use chrono::{Datelike, Utc};
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::ListData;
use crate::entity::{order, piece_record, register_code, user, user::Role, workshop};
use crate::error::{AppError, Result};

use super::dto::{AdminQueryParams, AdminStats, RegisterCodeResponse, UserListItem};

const CODE_CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const CODE_LENGTH: usize = 8;

fn generate_code() -> String {
    let mut rng = rand::rng();
    let random_part: String = (0..CODE_LENGTH)
        .map(|_| {
            let idx = rng.random_range(0..CODE_CHARSET.len());
            CODE_CHARSET[idx] as char
        })
        .collect();
    format!("B-{}", random_part)
}

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
    let disabled_codes = register_code::Entity::find()
        .filter(register_code::Column::IsActive.eq(false))
        .filter(register_code::Column::UsedBy.is_null())
        .count(db)
        .await? as i64;
    let available_codes = register_code::Entity::find()
        .filter(register_code::Column::IsActive.eq(true))
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
        disabled_codes,
        today_orders,
        month_orders,
        today_records,
        month_records,
    })
}

pub async fn create_register_code(db: &DbConn) -> Result<RegisterCodeResponse> {
    let code = generate_code();

    let model = register_code::ActiveModel {
        id: Set(Uuid::new_v4()),
        code: Set(code.clone()),
        is_active: Set(true),
        used_by: Set(None),
        used_at: Set(None),
        created_at: Set(chrono::Utc::now()),
    };

    let result = model.insert(db).await?;

    Ok(RegisterCodeResponse {
        id: result.id,
        code: result.code,
        is_active: result.is_active,
        used_by: result.used_by,
        used_at: result.used_at,
        created_at: result.created_at,
        used_by_username: None,
    })
}

pub async fn list_register_codes(
    db: &DbConn,
    params: AdminQueryParams,
) -> Result<ListData<RegisterCodeResponse>> {
    let paginator = register_code::Entity::find()
        .order_by_desc(register_code::Column::CreatedAt)
        .paginate(db, params.page_size);

    let total = paginator.num_items().await?;
    let codes = paginator.fetch_page(params.page - 1).await?;

    // Collect used_by ids to batch fetch usernames
    let user_ids: Vec<Uuid> = codes.iter().filter_map(|c| c.used_by).collect();

    let users_map: HashMap<Uuid, String> = if !user_ids.is_empty() {
        user::Entity::find()
            .filter(user::Column::Id.is_in(user_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|u| (u.id, u.username))
            .collect()
    } else {
        HashMap::new()
    };

    let list = codes
        .into_iter()
        .map(|c| RegisterCodeResponse {
            id: c.id,
            code: c.code,
            is_active: c.is_active,
            used_by: c.used_by,
            used_at: c.used_at,
            created_at: c.created_at,
            used_by_username: c.used_by.and_then(|id| users_map.get(&id).cloned()),
        })
        .collect();

    Ok(ListData { list, total })
}

pub async fn disable_register_code(db: &DbConn, id: Uuid) -> Result<()> {
    let code = register_code::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("注册码不存在".to_string()))?;

    let mut active: register_code::ActiveModel = code.into();
    active.is_active = Set(false);
    active.update(db).await?;

    Ok(())
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
