use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use super::dto::{CreateProcessDto, ProcessQueryParams, UpdateProcessDto};
use crate::common::{ListData, QueryParams};
use entity::process::{self, Column, Model};
use entity::user::Role;
use entity::{order, user, workshop};
use crate::error::{AppError, Result};
use crate::service::auth::Claims;

pub async fn list(
    db: &DbConn,
    params: QueryParams,
    filter: ProcessQueryParams,
    claims: &Claims,
) -> Result<ListData<Model>> {
    let mut query = process::Entity::find();

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
    if let Some(order_id) = filter.order_id {
        query = query.filter(Column::OrderId.eq(order_id));
    }

    if let Some(ref search) = params.search {
        query = query.filter(Column::Name.contains(search));
    }

    let order_dir = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };
    query = match params.sort_by.as_deref() {
        Some("name") => query.order_by(Column::Name, order_dir),
        Some("piece_price") => query.order_by(Column::PiecePrice, order_dir),
        _ => query.order_by(Column::Name, order_dir),
    };

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

pub async fn create(db: &DbConn, dto: CreateProcessDto) -> Result<Model> {
    // 获取 order 的 boss_id
    let ord = order::Entity::find_by_id(dto.order_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", dto.order_id)))?;

    let model = process::ActiveModel {
        id: Set(Uuid::new_v4()),
        order_id: Set(dto.order_id),
        boss_id: Set(ord.boss_id),
        name: Set(dto.name),
        description: Set(dto.description),
        piece_price: Set(dto.piece_price),
        ..Default::default()
    };
    Ok(model.insert(db).await?)
}

pub async fn get_one(db: &DbConn, id: Uuid, claims: &Claims) -> Result<Model> {
    let process = process::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Process {} not found", id)))?;
    // Boss can access their own processes; Staff can access processes in their workshop
    match claims.role {
        Role::Boss => {
            if process.boss_id != claims.sub {
                return Err(AppError::Forbidden);
            }
        }
        Role::Staff => {
            // Verify staff belongs to the workshop that owns this process
            let staff = user::Entity::find_by_id(claims.sub)
                .one(db)
                .await?
                .ok_or(AppError::Forbidden)?;
            let workshop_id = staff.workshop_id.ok_or(AppError::Forbidden)?;
            let ws = workshop::Entity::find_by_id(workshop_id)
                .one(db)
                .await?
                .ok_or(AppError::Forbidden)?;
            if process.boss_id != ws.owner_id {
                return Err(AppError::Forbidden);
            }
        }
    }
    Ok(process)
}

pub async fn update(db: &DbConn, id: Uuid, dto: UpdateProcessDto, boss_id: Uuid) -> Result<Model> {
    let process = process::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Process {} not found", id)))?;
    if process.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }

    let mut model: process::ActiveModel = process.into();
    if let Some(v) = dto.name {
        model.name = Set(v);
    }
    if let Some(v) = dto.description {
        model.description = Set(Some(v));
    }
    if let Some(v) = dto.piece_price {
        model.piece_price = Set(v);
    }
    Ok(model.update(db).await?)
}

pub async fn delete(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<()> {
    let process = process::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Process {} not found", id)))?;
    if process.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    process::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}
