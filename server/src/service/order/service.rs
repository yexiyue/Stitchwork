use chrono::NaiveDate;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use super::dto::{CreateOrderDto, OrderQueryParams, UpdateOrderDto, UpdateOrderStatusDto};
use crate::common::{ListData, QueryParams};
use crate::entity::order::{self, Column, Model, OrderStatus};
use crate::entity::user::Role;
use crate::entity::{customer, user, workshop};
use crate::error::{AppError, Result};
use crate::service::auth::Claims;

pub async fn list(
    db: &DbConn,
    params: QueryParams,
    filter: OrderQueryParams,
    claims: &Claims,
) -> Result<ListData<Model>> {
    let mut query = order::Entity::find();

    // 用户数据隔离
    match claims.role {
        Role::Boss => {
            query = query.filter(Column::BossId.eq(claims.sub));
        }
        Role::Staff => {
            // Staff 通过 workshop_id 找到老板
            let staff = user::Entity::find_by_id(claims.sub)
                .one(db)
                .await?
                .ok_or(AppError::Forbidden)?;
            let workshop_id = staff.workshop_id.ok_or(AppError::Forbidden)?;
            let ws = workshop::Entity::find_by_id(workshop_id)
                .one(db)
                .await?
                .ok_or(AppError::Forbidden)?;
            query = query.filter(Column::BossId.eq(ws.owner_id));
        }
    }

    // 新增过滤参数
    if let Some(customer_id) = filter.customer_id {
        query = query.filter(Column::CustomerId.eq(customer_id));
    }
    if let Some(ref status) = filter.status {
        query = query.filter(Column::Status.eq(status));
    }

    if let Some(ref search) = params.search {
        query = query.filter(Column::ProductName.contains(search));
    }

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query.filter(Column::ReceivedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(Column::ReceivedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()));
        }
    }

    let order_dir = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };
    query = match params.sort_by.as_deref() {
        Some("product_name") => query.order_by(Column::ProductName, order_dir),
        Some("status") => query.order_by(Column::Status, order_dir),
        _ => query.order_by(Column::ReceivedAt, order_dir),
    };

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

pub async fn create(db: &DbConn, dto: CreateOrderDto) -> Result<Model> {
    // 获取 customer 的 user_id 作为 boss_id
    let cust = customer::Entity::find_by_id(dto.customer_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Customer {} not found", dto.customer_id)))?;

    let model = order::ActiveModel {
        id: Set(Uuid::new_v4()),
        customer_id: Set(dto.customer_id),
        boss_id: Set(cust.user_id),
        product_name: Set(dto.product_name),
        description: Set(dto.description),
        images: Set(dto.images),
        quantity: Set(dto.quantity),
        unit_price: Set(dto.unit_price),
        status: Set(OrderStatus::Pending),
        received_at: Set(chrono::Utc::now()),
        delivered_at: Set(None),
    };
    Ok(model.insert(db).await?)
}

pub async fn get_one(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<Model> {
    let order = order::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))?;
    if order.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    Ok(order)
}

pub async fn update(db: &DbConn, id: Uuid, dto: UpdateOrderDto, boss_id: Uuid) -> Result<Model> {
    let order = order::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))?;
    if order.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }

    let mut model: order::ActiveModel = order.into();
    if let Some(v) = dto.product_name {
        model.product_name = Set(v);
    }
    if let Some(v) = dto.description {
        model.description = Set(Some(v));
    }
    if let Some(v) = dto.images {
        model.images = Set(Some(v));
    }
    if let Some(v) = dto.quantity {
        model.quantity = Set(v);
    }
    if let Some(v) = dto.unit_price {
        model.unit_price = Set(v);
    }
    if let Some(v) = dto.status {
        let status: OrderStatus = v
            .parse()
            .map_err(|_| AppError::BadRequest(format!("Invalid status: {}", v)))?;
        if status == OrderStatus::Delivered {
            model.delivered_at = Set(Some(chrono::Utc::now()));
        }
        model.status = Set(status);
    }
    Ok(model.update(db).await?)
}

pub async fn delete(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<()> {
    let order = order::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))?;
    if order.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    order::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn update_status(
    db: &DbConn,
    id: Uuid,
    dto: UpdateOrderStatusDto,
    boss_id: Uuid,
) -> Result<Model> {
    let order = order::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))?;
    if order.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }

    let next: OrderStatus = dto
        .status
        .parse()
        .map_err(|_| AppError::BadRequest(format!("Invalid status: {}", dto.status)))?;
    if !order.status.can_transition_to(next) {
        return Err(AppError::BadRequest(format!(
            "Cannot transition from {:?} to {:?}",
            order.status, next
        )));
    }

    let delivered_at = if next == OrderStatus::Delivered {
        Set(Some(chrono::Utc::now()))
    } else {
        NotSet
    };

    let model = order::ActiveModel {
        id: Set(order.id),
        status: Set(next),
        delivered_at,
        ..Default::default()
    };
    Ok(model.update(db).await?)
}
