use rand::Rng;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, QueryOrder, Set};
use std::collections::HashMap;
use uuid::Uuid;

use crate::entity::{register_code, user};
use crate::error::{AppError, Result};

use super::dto::{RegisterCodeResponse, UserListItem};

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

pub async fn list_register_codes(db: &DbConn) -> Result<Vec<RegisterCodeResponse>> {
    let codes = register_code::Entity::find()
        .order_by_desc(register_code::Column::CreatedAt)
        .all(db)
        .await?;

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

    Ok(codes
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
        .collect())
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

pub async fn list_users(db: &DbConn) -> Result<Vec<UserListItem>> {
    let users = user::Entity::find()
        .order_by_desc(user::Column::CreatedAt)
        .all(db)
        .await?;

    Ok(users
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
        .collect())
}
