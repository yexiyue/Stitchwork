use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use super::dto::{CreatePieceRecordDto, UpdatePieceRecordDto};
use crate::common::{ListData, QueryParams};
use crate::entity::order::OrderStatus;
use crate::entity::user::Role;
use crate::entity::{
    order,
    piece_record::{self, Column, Model, PieceRecordStatus, RecordedBy},
    process,
};
use crate::error::{AppError, Result};
use crate::service::auth::Claims;

pub async fn list(db: &DbConn, params: QueryParams, claims: &Claims) -> Result<ListData<Model>> {
    let mut query = piece_record::Entity::find();

    // 用户数据隔离
    match claims.role {
        Role::Boss => {
            query = query.filter(Column::BossId.eq(claims.sub));
        }
        Role::Staff => {
            query = query.filter(Column::UserId.eq(claims.sub));
        }
    }

    // 按状态过滤
    if let Some(ref status) = params.status {
        let status_enum = match status.as_str() {
            "pending" => Some(PieceRecordStatus::Pending),
            "approved" => Some(PieceRecordStatus::Approved),
            "rejected" => Some(PieceRecordStatus::Rejected),
            _ => None,
        };
        if let Some(s) = status_enum {
            query = query.filter(Column::Status.eq(s));
        }
    }

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query.filter(Column::RecordedAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(Column::RecordedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()));
        }
    }

    let order_dir = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };
    query = match params.sort_by.as_deref() {
        Some("quantity") => query.order_by(Column::Quantity, order_dir),
        Some("amount") => query.order_by(Column::Amount, order_dir),
        _ => query.order_by(Column::RecordedAt, order_dir),
    };

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

pub async fn create(db: &DbConn, dto: CreatePieceRecordDto, claims: &Claims) -> Result<Model> {
    let proc = process::Entity::find_by_id(dto.process_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Process {} not found", dto.process_id)))?;

    // 自动更新订单状态: pending → processing
    let ord = order::Entity::find_by_id(proc.order_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", proc.order_id)))?;
    if ord.status == OrderStatus::Pending {
        let order_model = order::ActiveModel {
            id: Set(ord.id),
            status: Set(OrderStatus::Processing),
            ..Default::default()
        };
        order_model.update(db).await?;
    }

    let amount = proc.piece_price * Decimal::from(dto.quantity);

    // 根据角色设置 status 和 recorded_by
    let (status, recorded_by) = match claims.role {
        Role::Boss => (PieceRecordStatus::Approved, RecordedBy::ByBoss),
        Role::Staff => (PieceRecordStatus::Pending, RecordedBy::BySelf),
    };

    let model = piece_record::ActiveModel {
        id: Set(Uuid::new_v4()),
        process_id: Set(dto.process_id),
        user_id: Set(dto.user_id),
        boss_id: Set(proc.boss_id),
        quantity: Set(dto.quantity),
        amount: Set(amount),
        status: Set(status),
        recorded_by: Set(recorded_by),
        recorded_at: Set(chrono::Utc::now()),
    };
    Ok(model.insert(db).await?)
}

pub async fn get_one(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<Model> {
    let record = piece_record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("PieceRecord {} not found", id)))?;
    if record.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    Ok(record)
}

pub async fn update(
    db: &DbConn,
    id: Uuid,
    dto: UpdatePieceRecordDto,
    boss_id: Uuid,
) -> Result<Model> {
    let record = piece_record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("PieceRecord {} not found", id)))?;
    if record.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }

    let proc = process::Entity::find_by_id(record.process_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Process {} not found", record.process_id)))?;

    let mut model: piece_record::ActiveModel = record.into();
    if let Some(qty) = dto.quantity {
        model.quantity = Set(qty);
        model.amount = Set(proc.piece_price * Decimal::from(qty));
    }
    Ok(model.update(db).await?)
}

pub async fn delete(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<()> {
    let record = piece_record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("PieceRecord {} not found", id)))?;
    if record.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    piece_record::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn approve(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<Model> {
    let record = piece_record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("PieceRecord {} not found", id)))?;
    if record.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    if record.status != PieceRecordStatus::Pending {
        return Err(AppError::BadRequest(
            "Only pending records can be approved".to_string(),
        ));
    }

    let mut model: piece_record::ActiveModel = record.into();
    model.status = Set(PieceRecordStatus::Approved);
    Ok(model.update(db).await?)
}

pub async fn reject(db: &DbConn, id: Uuid, boss_id: Uuid) -> Result<Model> {
    let record = piece_record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("PieceRecord {} not found", id)))?;
    if record.boss_id != boss_id {
        return Err(AppError::Forbidden);
    }
    if record.status != PieceRecordStatus::Pending {
        return Err(AppError::BadRequest(
            "Only pending records can be rejected".to_string(),
        ));
    }

    let mut model: piece_record::ActiveModel = record.into();
    model.status = Set(PieceRecordStatus::Rejected);
    Ok(model.update(db).await?)
}
