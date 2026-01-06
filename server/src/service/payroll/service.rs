use chrono::NaiveDate;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set, TransactionTrait,
};
use uuid::Uuid;

use super::dto::{CreatePayrollDto, UpdatePayrollDto};
use crate::common::{ListData, QueryParams};
use crate::entity::payroll::{self, Column, Model};
use crate::entity::{payroll_record, piece_record};
use crate::error::{AppError, Result};

pub async fn list(
    db: &DbConn,
    params: QueryParams,
    user_id: Option<Uuid>,
    boss_id: Option<Uuid>,
) -> Result<ListData<Model>> {
    let mut query = payroll::Entity::find();

    if let Some(uid) = user_id {
        query = query.filter(Column::UserId.eq(uid));
    }
    if let Some(bid) = boss_id {
        query = query.filter(Column::BossId.eq(bid));
    }

    if let Some(ref start) = params.start_date {
        if let Ok(date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
            query = query.filter(Column::PaidAt.gte(date.and_hms_opt(0, 0, 0).unwrap()));
        }
    }
    if let Some(ref end) = params.end_date {
        if let Ok(date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
            query = query.filter(Column::PaidAt.lte(date.and_hms_opt(23, 59, 59).unwrap()));
        }
    }

    let order_dir = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };
    query = match params.sort_by.as_deref() {
        Some("amount") => query.order_by(Column::Amount, order_dir),
        _ => query.order_by(Column::PaidAt, order_dir),
    };

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

pub async fn create(db: &DbConn, dto: CreatePayrollDto, boss_id: Uuid) -> Result<Model> {
    // 验证计件记录状态
    if dto.record_ids.is_empty() {
        return Err(AppError::BadRequest("至少选择一条计件记录".into()));
    }

    let records = piece_record::Entity::find()
        .filter(piece_record::Column::Id.is_in(dto.record_ids.clone()))
        .filter(piece_record::Column::BossId.eq(boss_id))
        .all(db)
        .await?;

    if records.len() != dto.record_ids.len() {
        return Err(AppError::BadRequest("部分计件记录不存在或无权限".into()));
    }

    for rec in &records {
        if rec.status != piece_record::PieceRecordStatus::Approved {
            return Err(AppError::BadRequest(format!(
                "计件记录 {} 状态不是已批准",
                rec.id
            )));
        }
    }

    // 使用事务
    let txn = db.begin().await?;

    // 创建工资单
    let payroll_id = Uuid::new_v4();
    let model = payroll::ActiveModel {
        id: Set(payroll_id),
        user_id: Set(dto.user_id),
        boss_id: Set(boss_id),
        amount: Set(dto.amount),
        payment_image: Set(dto.payment_image),
        note: Set(dto.note),
        paid_at: Set(chrono::Utc::now()),
    };
    let payroll = model.insert(&txn).await?;

    // 创建关联记录并更新计件状态
    for rec in &records {
        // 创建关联
        let pr = payroll_record::ActiveModel {
            id: Set(Uuid::new_v4()),
            payroll_id: Set(payroll_id),
            piece_record_id: Set(rec.id),
        };
        pr.insert(&txn).await?;

        // 更新计件状态为已结算
        let mut active: piece_record::ActiveModel = rec.clone().into();
        active.status = Set(piece_record::PieceRecordStatus::Settled);
        active.update(&txn).await?;
    }

    txn.commit().await?;
    Ok(payroll)
}

pub async fn get_one(db: &DbConn, id: Uuid, user_id: Option<Uuid>) -> Result<Model> {
    let mut query = payroll::Entity::find_by_id(id);
    if let Some(uid) = user_id {
        query = query.filter(Column::UserId.eq(uid));
    }
    query
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Payroll {} not found", id)))
}

pub async fn update(db: &DbConn, id: Uuid, dto: UpdatePayrollDto) -> Result<Model> {
    let payroll = payroll::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Payroll {} not found", id)))?;

    let mut model: payroll::ActiveModel = payroll.into();
    if let Some(v) = dto.amount {
        model.amount = Set(v);
    }
    if let Some(v) = dto.note {
        model.note = Set(Some(v));
    }
    if let Some(v) = dto.payment_image {
        model.payment_image = Set(Some(v));
    }
    Ok(model.update(db).await?)
}

pub async fn delete(db: &DbConn, id: Uuid) -> Result<()> {
    let result = payroll::Entity::delete_by_id(id).exec(db).await?;
    if result.rows_affected == 0 {
        return Err(AppError::NotFound(format!("Payroll {} not found", id)));
    }
    Ok(())
}
