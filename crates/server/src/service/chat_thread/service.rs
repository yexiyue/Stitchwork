use super::dto::UpdateThreadDto;
use crate::common::{ListData, QueryParams, apply_date_filter};
use crate::error::Result;
use anyhow::anyhow;
use chrono::Utc;
use entity::chat_thread::Column;
use sea_orm::{IntoActiveModel, QueryOrder, prelude::*};
use uuid::Uuid;

pub async fn list(
    db: &DbConn,
    params: QueryParams,
    user_id: Uuid,
) -> Result<ListData<entity::chat_thread::Model>> {
    let mut query = entity::chat_thread::Entity::find().filter(Column::UserId.eq(user_id));

    if let Some(ref search) = params.search {
        query = query.filter(Column::Title.contains(search));
    }
    query = apply_date_filter(
        query,
        Column::UpdatedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    let order = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };

    query = match params.sort_by.as_deref() {
        Some("title") => query.order_by(Column::Title, order),
        _ => query.order_by(Column::UpdatedAt, order),
    };

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

pub async fn get_one(db: &DbConn, id: Uuid, user_id: Uuid) -> Result<entity::chat_thread::Model> {
    let thread = entity::chat_thread::Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(db)
        .await?;
    thread.ok_or_else(|| anyhow!("会话不存在或无权访问").into())
}

pub async fn update(
    db: &DbConn,
    id: Uuid,
    user_id: Uuid,
    dto: UpdateThreadDto,
) -> Result<entity::chat_thread::Model> {
    let thread = entity::chat_thread::Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| anyhow!("会话不存在或无权访问"))?;

    let mut model = thread.into_active_model().into_ex();

    if let Some(title) = dto.title {
        model = model.set_title(title);
    }

    if let Some(archived) = dto.archived {
        model = model.set_archived(archived);
    }

    Ok(model.update(db).await?.into())
}

pub async fn create(
    db: &DbConn,
    user_id: Uuid,
) -> Result<entity::chat_thread::Model> {
    let model = entity::chat_thread::ActiveModelEx::new()
        .set_user_id(user_id)
        .set_created_at(Utc::now());
    Ok(model.insert(db).await?.into())
}

pub async fn delete(db: &DbConn, id: Uuid, user_id: Uuid) -> Result<()> {
    entity::chat_thread::Entity::delete_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn add_message(
    db: &DbConn,
    thread_id: Uuid,
    user_id: Uuid,
    message: serde_json::Value,
) -> Result<entity::chat_message::Model> {
    // 验证 thread 存在且属于该用户
    let exists = entity::chat_thread::Entity::find_by_id(thread_id)
        .filter(entity::chat_thread::Column::UserId.eq(user_id))
        .exists(db)
        .await?;

    if !exists {
        return Err(anyhow!("会话不存在或无权访问").into());
    }

    let model = entity::chat_message::ActiveModelEx::new()
        .set_thread_id(thread_id)
        .set_value(message)
        .set_created_at(Utc::now());
    Ok(model.insert(db).await?.into())
}

pub async fn list_messages(
    db: &DbConn,
    thread_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<entity::chat_message::Model>> {
    // 验证 thread 存在且属于该用户
    let exists = entity::chat_thread::Entity::find_by_id(thread_id)
        .filter(entity::chat_thread::Column::UserId.eq(user_id))
        .exists(db)
        .await?;

    if !exists {
        return Err(anyhow!("会话不存在或无权访问").into());
    }

    let data = entity::chat_message::Entity::find()
        .filter(entity::chat_message::COLUMN.thread_id.eq(thread_id))
        .all(db)
        .await?;

    Ok(data)
}
