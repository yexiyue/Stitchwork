use chrono::NaiveDate;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use super::dto::{CreateCustomerDto, UpdateCustomerDto};
use crate::common::{ListData, QueryParams};
use crate::entity::customer::{self, Column, Model};
use crate::error::{AppError, Result};

pub async fn list(db: &DbConn, params: QueryParams, boss_id: Uuid) -> Result<ListData<Model>> {
    let mut query = customer::Entity::find().filter(Column::UserId.eq(boss_id));

    if let Some(ref search) = params.search {
        query = query.filter(Column::Name.contains(search));
    }
    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query.filter(Column::CreatedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(Column::CreatedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()));
        }
    }

    let order = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };
    query = match params.sort_by.as_deref() {
        Some("name") => query.order_by(Column::Name, order),
        _ => query.order_by(Column::CreatedAt, order),
    };

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

pub async fn create(db: &DbConn, user_id: Uuid, dto: CreateCustomerDto) -> Result<Model> {
    let model = customer::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        name: Set(dto.name),
        phone: Set(dto.phone),
        description: Set(dto.description),
        created_at: Set(chrono::Utc::now()),
    };
    Ok(model.insert(db).await?)
}

pub async fn get_one(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<Model> {
    let customer = customer::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Customer {} not found", id)))?;
    if customer.user_id != boss_id {
        return Err(AppError::Forbidden);
    }
    Ok(customer)
}

pub async fn update(db: &DbConn, id: Uuid, dto: UpdateCustomerDto, boss_id: Uuid) -> Result<Model> {
    let customer = customer::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Customer {} not found", id)))?;
    if customer.user_id != boss_id {
        return Err(AppError::Forbidden);
    }

    let mut model: customer::ActiveModel = customer.into();
    if let Some(name) = dto.name {
        model.name = Set(name);
    }
    if let Some(phone) = dto.phone {
        model.phone = Set(Some(phone));
    }
    if let Some(desc) = dto.description {
        model.description = Set(Some(desc));
    }
    Ok(model.update(db).await?)
}

pub async fn delete(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<()> {
    let customer = customer::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Customer {} not found", id)))?;
    if customer.user_id != boss_id {
        return Err(AppError::Forbidden);
    }
    customer::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}
