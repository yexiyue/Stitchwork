use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entity::{order, process, share, user};
use crate::error::{AppError, Result};

use super::dto::{
    CreateShareRequest, PublicOrderInfo, PublicProcessInfo, PublicShareResponse, UpdateShareRequest,
};

pub async fn create(db: &DbConn, boss_id: Uuid, req: CreateShareRequest) -> Result<share::Model> {
    let token = Uuid::new_v4().to_string()[..8].to_string();
    let share = share::ActiveModel {
        id: Set(Uuid::new_v4()),
        boss_id: Set(boss_id),
        title: Set(req.title),
        token: Set(token),
        order_ids: Set(serde_json::to_value(&req.order_ids).unwrap()),
        process_ids: Set(serde_json::to_value(&req.process_ids).unwrap()),
        is_active: Set(true),
        created_at: Set(chrono::Utc::now()),
    }
    .insert(db)
    .await?;
    Ok(share)
}

pub async fn list(db: &DbConn, boss_id: Uuid) -> Result<Vec<share::Model>> {
    let shares = share::Entity::find()
        .filter(share::Column::BossId.eq(boss_id))
        .all(db)
        .await?;
    Ok(shares)
}

pub async fn update(db: &DbConn, boss_id: Uuid, id: Uuid, req: UpdateShareRequest) -> Result<share::Model> {
    let share = share::Entity::find_by_id(id)
        .filter(share::Column::BossId.eq(boss_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("分享不存在".to_string()))?;

    let mut active: share::ActiveModel = share.into();
    if let Some(v) = req.title { active.title = Set(v); }
    if let Some(v) = req.order_ids { active.order_ids = Set(serde_json::to_value(&v).unwrap()); }
    if let Some(v) = req.process_ids { active.process_ids = Set(serde_json::to_value(&v).unwrap()); }
    if let Some(v) = req.is_active { active.is_active = Set(v); }
    let share = active.update(db).await?;
    Ok(share)
}

pub async fn delete(db: &DbConn, boss_id: Uuid, id: Uuid) -> Result<()> {
    let res = share::Entity::delete_many()
        .filter(share::Column::Id.eq(id))
        .filter(share::Column::BossId.eq(boss_id))
        .exec(db)
        .await?;
    if res.rows_affected == 0 {
        return Err(AppError::NotFound("分享不存在".to_string()));
    }
    Ok(())
}

pub async fn get_public(db: &DbConn, token: &str) -> Result<PublicShareResponse> {
    let share = share::Entity::find()
        .filter(share::Column::Token.eq(token))
        .filter(share::Column::IsActive.eq(true))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("分享不存在或已停用".to_string()))?;

    let boss = user::Entity::find_by_id(share.boss_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    let order_ids: Vec<Uuid> = serde_json::from_value(share.order_ids).unwrap_or_default();
    let process_ids: Vec<Uuid> = serde_json::from_value(share.process_ids).unwrap_or_default();

    let orders: Vec<PublicOrderInfo> = if !order_ids.is_empty() {
        order::Entity::find()
            .filter(order::Column::Id.is_in(order_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|o| PublicOrderInfo {
                id: o.id,
                product_name: o.product_name,
                description: o.description,
                images: o.images,
                quantity: o.quantity,
            })
            .collect()
    } else {
        vec![]
    };

    let processes: Vec<PublicProcessInfo> = if !process_ids.is_empty() {
        let procs = process::Entity::find()
            .filter(process::Column::Id.is_in(process_ids.clone()))
            .all(db)
            .await?;

        let related_order_ids: Vec<Uuid> = procs.iter().map(|p| p.order_id).collect();
        let related_orders: std::collections::HashMap<Uuid, String> = order::Entity::find()
            .filter(order::Column::Id.is_in(related_order_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|o| (o.id, o.product_name))
            .collect();

        procs.into_iter().map(|p| PublicProcessInfo {
            id: p.id,
            name: p.name,
            description: p.description,
            piece_price: p.piece_price,
            order_product_name: related_orders.get(&p.order_id).cloned().unwrap_or_default(),
        }).collect()
    } else {
        vec![]
    };

    Ok(PublicShareResponse {
        title: share.title,
        workshop_name: boss.workshop_name,
        workshop_desc: boss.workshop_desc,
        avatar: boss.avatar,
        orders,
        processes,
    })
}
