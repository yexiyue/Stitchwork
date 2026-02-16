use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::ListData;
use crate::error::{AppError, Result};
use entity::{register_code, user, workshop};

use super::dto::{RegisterCodeQueryParams, RegisterCodeResponse};

const CODE_LENGTH: usize = 6;

/// 生成 6 位纯数字注册码，确保不与数据库已有码重复
async fn generate_unique_code(db: &DbConn) -> Result<String> {
    loop {
        let code: String = {
            let mut rng = rand::rng();
            (0..CODE_LENGTH)
                .map(|_| char::from(b'0' + rng.random_range(0..10u8)))
                .collect()
        };
        let exists = register_code::Entity::find()
            .filter(register_code::Column::Code.eq(&code))
            .one(db)
            .await?;
        if exists.is_none() {
            return Ok(code);
        }
    }
}

fn to_response(c: register_code::Model, username: Option<String>) -> RegisterCodeResponse {
    RegisterCodeResponse {
        id: c.id,
        code: c.code,
        created_by: c.created_by,
        workshop_id: c.workshop_id,
        used_by: c.used_by,
        used_at: c.used_at,
        created_at: c.created_at,
        used_by_username: username,
    }
}

/// 创建注册码。超管创建的码无 workshop_id，Boss 创建的码关联 workshop。
pub async fn create_register_code(
    db: &DbConn,
    user_id: Uuid,
    is_super_admin: bool,
) -> Result<RegisterCodeResponse> {
    let workshop_id = if is_super_admin {
        None
    } else {
        // Boss: 查找其工坊
        let ws = workshop::Entity::find()
            .filter(workshop::Column::OwnerId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::BadRequest("请先创建工坊".to_string()))?;
        Some(ws.id)
    };

    let code = generate_unique_code(db).await?;

    let model = register_code::ActiveModel {
        id: Set(Uuid::new_v4()),
        code: Set(code),
        created_by: Set(Some(user_id)),
        workshop_id: Set(workshop_id),
        used_by: Set(None),
        used_at: Set(None),
        created_at: Set(chrono::Utc::now()),
    };

    let result = model.insert(db).await?;
    Ok(to_response(result, None))
}

/// 列出注册码。超管看所有 Boss 码（workshop_id IS NULL），Boss 看自己创建的码。
pub async fn list_register_codes(
    db: &DbConn,
    user_id: Uuid,
    is_super_admin: bool,
    params: RegisterCodeQueryParams,
) -> Result<ListData<RegisterCodeResponse>> {
    let mut query = register_code::Entity::find();

    if is_super_admin {
        // 超管看所有 Boss 码（workshop_id IS NULL）
        query = query.filter(register_code::Column::WorkshopId.is_null());
    } else {
        // Boss 看自己创建的码
        query = query.filter(register_code::Column::CreatedBy.eq(user_id));
    }

    let paginator = query
        .order_by_desc(register_code::Column::CreatedAt)
        .paginate(db, params.page_size);

    let total = paginator.num_items().await?;
    let codes = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    // 批量加载使用者用户名
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
        .map(|c| {
            let username = c.used_by.and_then(|id| users_map.get(&id).cloned());
            to_response(c, username)
        })
        .collect();

    Ok(ListData { list, total })
}

/// 删除注册码。仅未使用的码可删，仅创建者（或超管）可删。
pub async fn delete_register_code(
    db: &DbConn,
    user_id: Uuid,
    is_super_admin: bool,
    code_id: Uuid,
) -> Result<()> {
    let code = register_code::Entity::find_by_id(code_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("注册码不存在".to_string()))?;

    // 权限检查：必须是创建者或超管
    if !is_super_admin && code.created_by != Some(user_id) {
        return Err(AppError::Forbidden);
    }

    // 已使用的码不可删除
    if code.used_by.is_some() {
        return Err(AppError::BadRequest("已使用的注册码不可删除".to_string()));
    }

    code.delete(db).await?;
    Ok(())
}
