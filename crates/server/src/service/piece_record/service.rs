use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityLoaderTrait, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use super::dto::{CreatePieceRecordDto, PieceRecordResponse, UpdatePieceRecordDto};
use crate::common::{ListData, QueryParams};
use crate::error::{AppError, Result};
use crate::service::auth::Claims;
use entity::order::OrderStatus;
use entity::user::{self, Role};
use entity::{
    order,
    piece_record::{self, Column, Model, PieceRecordStatus, RecordedBy},
    process,
};
use std::collections::HashMap;

pub async fn list(
    db: &DbConn,
    params: QueryParams,
    claims: &Claims,
) -> Result<ListData<PieceRecordResponse>> {
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
    if let Some(ref statuses) = params.status {
        let status_enums: Vec<PieceRecordStatus> = statuses
            .iter()
            .filter_map(|s| match s.as_str() {
                "pending" => Some(PieceRecordStatus::Pending),
                "approved" => Some(PieceRecordStatus::Approved),
                "rejected" => Some(PieceRecordStatus::Rejected),
                _ => None,
            })
            .collect();
        if !status_enums.is_empty() {
            query = query.filter(Column::Status.is_in(status_enums));
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
    let records = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    // 批量获取关联数据
    let process_ids: Vec<Uuid> = records.iter().map(|r| r.process_id).collect();
    let user_ids: Vec<Uuid> = records.iter().map(|r| r.user_id).collect();

    let processes: HashMap<Uuid, process::Model> = process::Entity::find()
        .filter(process::Column::Id.is_in(process_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.id, p))
        .collect();

    let order_ids: Vec<Uuid> = processes.values().map(|p| p.order_id).collect();
    let orders: HashMap<Uuid, order::Model> = order::Entity::find()
        .filter(order::Column::Id.is_in(order_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|o| (o.id, o))
        .collect();

    let users: HashMap<Uuid, user::Model> = user::Entity::find()
        .filter(user::Column::Id.is_in(user_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|u| (u.id, u))
        .collect();

    // 组装响应
    let list = records
        .into_iter()
        .map(|r| {
            let proc = processes.get(&r.process_id);
            let ord = proc.and_then(|p| orders.get(&p.order_id));
            let usr = users.get(&r.user_id);

            // 获取订单第一张图片
            let order_image = ord.and_then(|o| {
                o.images.as_ref().and_then(|imgs| {
                    imgs.as_array()
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
            });

            PieceRecordResponse {
                id: r.id,
                process_id: r.process_id,
                user_id: r.user_id,
                boss_id: r.boss_id,
                quantity: r.quantity,
                amount: r.amount,
                status: r.status,
                recorded_by: r.recorded_by,
                recorded_at: r.recorded_at,
                process_name: proc.map(|p| p.name.clone()),
                user_name: usr.map(|u| u.display_name.clone().unwrap_or(u.username.clone())),
                order_id: proc.map(|p| p.order_id),
                order_name: ord.map(|o| o.product_name.clone()),
                order_image,
                piece_price: proc.map(|p| p.piece_price),
            }
        })
        .collect();

    Ok(ListData { list, total })
}

pub async fn create(db: &DbConn, dto: CreatePieceRecordDto, claims: &Claims) -> Result<Model> {
    let proc = process::Entity::find_by_id(dto.process_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("工序不存在".to_string()))?;

    // 确定实际的 user_id，防止越权
    let actual_user_id = match claims.role {
        Role::Staff => {
            // 员工只能为自己创建记录
            claims.sub
        }
        Role::Boss => {
            // 老板必须是该工序的所有者
            if proc.boss_id != claims.sub {
                return Err(AppError::Forbidden);
            }
            // 验证目标用户是本工坊的员工
            let target_user = user::Entity::find_by_id(dto.user_id)
                .one(db)
                .await?
                .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
            // 获取老板的工坊
            let workshop = entity::workshop::Entity::find()
                .filter(entity::workshop::Column::OwnerId.eq(claims.sub))
                .one(db)
                .await?
                .ok_or_else(|| AppError::NotFound("工坊不存在".to_string()))?;
            // 目标用户必须属于该工坊
            if target_user.workshop_id != Some(workshop.id) {
                return Err(AppError::BadRequest("该用户不属于您的工坊".to_string()));
            }
            dto.user_id
        }
    };

    // 自动更新订单状态: pending → processing
    let ord = order::Entity::find_by_id(proc.order_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("订单不存在".to_string()))?;
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
        user_id: Set(actual_user_id),
        boss_id: Set(proc.boss_id),
        quantity: Set(dto.quantity),
        amount: Set(amount),
        status: Set(status),
        recorded_by: Set(recorded_by),
        recorded_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    Ok(model.insert(db).await?)
}

pub async fn get_one(db: &DbConn, id: Uuid, claims: &Claims) -> Result<PieceRecordResponse> {
    let record = piece_record::Entity::load()
        .filter_by_id(id)
        .with(entity::user::Entity)
        .with((entity::process::Entity, entity::order::Entity))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("PieceRecord {} not found", id)))?;

    // Boss can access records they own; Staff can access their own records
    match claims.role {
        Role::Boss => {
            if record.boss_id != claims.sub {
                return Err(AppError::Forbidden);
            }
        }
        Role::Staff => {
            if record.user_id != claims.sub {
                return Err(AppError::Forbidden);
            }
        }
    }

    let proc = record.process.as_ref();
    let ord = proc.and_then(|p| p.order.as_ref());
    let usr = record.user.as_ref();

    let order_image = ord.and_then(|o| {
        o.images.as_ref().and_then(|imgs| {
            imgs.as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
    });

    Ok(PieceRecordResponse {
        id: record.id,
        process_id: record.process_id,
        user_id: record.user_id,
        boss_id: record.boss_id,
        quantity: record.quantity,
        amount: record.amount,
        status: record.status,
        recorded_by: record.recorded_by,
        recorded_at: record.recorded_at,
        process_name: proc.map(|p| p.name.clone()),
        user_name: usr.map(|u| u.display_name.clone().unwrap_or(u.username.clone())),
        order_id: proc.map(|p| p.order_id),
        order_name: ord.map(|o| o.product_name.clone()),
        order_image,
        piece_price: proc.map(|p| p.piece_price),
    })
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

/// 查询待处理的记录（用于批量操作前获取通知所需信息）
pub async fn get_pending_records(
    db: &DbConn,
    ids: &[Uuid],
    boss_id: Uuid,
) -> Result<Vec<PieceRecordResponse>> {
    let records = piece_record::Entity::load()
        .with(process::Entity)
        .filter(piece_record::Column::Id.is_in(ids.to_vec()))
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Pending))
        .all(db)
        .await?;

    Ok(records
        .into_iter()
        .map(|r| PieceRecordResponse {
            id: r.id,
            process_id: r.process_id,
            user_id: r.user_id,
            boss_id: r.boss_id,
            quantity: r.quantity,
            amount: r.amount,
            status: r.status,
            recorded_by: r.recorded_by,
            recorded_at: r.recorded_at,
            process_name: r.process.as_ref().map(|p| p.name.clone()),
            user_name: None,
            order_id: r.process.as_ref().map(|p| p.order_id),
            order_name: None,
            order_image: None,
            piece_price: r.process.as_ref().map(|p| p.piece_price),
        })
        .collect())
}

pub async fn batch_approve(db: &DbConn, ids: Vec<Uuid>, boss_id: Uuid) -> Result<u64> {
    let result = piece_record::Entity::update_many()
        .col_expr(
            piece_record::Column::Status,
            sea_orm::sea_query::Expr::value(PieceRecordStatus::Approved),
        )
        .filter(piece_record::Column::Id.is_in(ids))
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Pending))
        .exec(db)
        .await?;
    Ok(result.rows_affected)
}

pub async fn batch_reject(db: &DbConn, ids: Vec<Uuid>, boss_id: Uuid) -> Result<u64> {
    let result = piece_record::Entity::update_many()
        .col_expr(
            piece_record::Column::Status,
            sea_orm::sea_query::Expr::value(PieceRecordStatus::Rejected),
        )
        .filter(piece_record::Column::Id.is_in(ids))
        .filter(piece_record::Column::BossId.eq(boss_id))
        .filter(piece_record::Column::Status.eq(PieceRecordStatus::Pending))
        .exec(db)
        .await?;
    Ok(result.rows_affected)
}
