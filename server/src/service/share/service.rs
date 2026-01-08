use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, ExprTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entity::{order, piece_record, process, share, user, workshop};
use crate::error::{AppError, Result};

use super::dto::{
    CreateShareRequest, PublicProcessInfo, PublicShareResponse, UpdateShareRequest,
};

pub async fn create(db: &DbConn, boss_id: Uuid, req: CreateShareRequest) -> Result<share::Model> {
    let token = Uuid::new_v4().to_string()[..8].to_string();
    let share = share::ActiveModel {
        id: Set(Uuid::new_v4()),
        boss_id: Set(boss_id),
        title: Set(req.title),
        description: Set(req.description),
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

pub async fn update(
    db: &DbConn,
    boss_id: Uuid,
    id: Uuid,
    req: UpdateShareRequest,
) -> Result<share::Model> {
    let share = share::Entity::find_by_id(id)
        .filter(share::Column::BossId.eq(boss_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("分享不存在".to_string()))?;

    let mut active: share::ActiveModel = share.into();
    if let Some(v) = req.title {
        active.title = Set(v);
    }
    if let Some(v) = req.description {
        active.description = Set(Some(v));
    }
    if let Some(v) = req.order_ids {
        active.order_ids = Set(serde_json::to_value(&v).unwrap());
    }
    if let Some(v) = req.process_ids {
        active.process_ids = Set(serde_json::to_value(&v).unwrap());
    }
    if let Some(v) = req.is_active {
        active.is_active = Set(v);
    }
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

    // 获取老板的工坊信息
    let ws = workshop::Entity::find()
        .filter(workshop::Column::OwnerId.eq(boss.id))
        .one(db)
        .await?;

    let process_ids: Vec<Uuid> = serde_json::from_value(share.process_ids).unwrap_or_default();

    let processes: Vec<PublicProcessInfo> = if !process_ids.is_empty() {
        let procs = process::Entity::find()
            .filter(process::Column::Id.is_in(process_ids.clone()))
            .all(db)
            .await?;

        // 获取关联订单信息（产品名、数量、图片）
        let related_order_ids: Vec<Uuid> = procs.iter().map(|p| p.order_id).collect();
        let related_orders: std::collections::HashMap<Uuid, (String, i32, Vec<String>)> = order::Entity::find()
            .filter(order::Column::Id.is_in(related_order_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|o| {
                let images: Vec<String> = o.images
                    .and_then(|v| serde_json::from_value(v).ok())
                    .unwrap_or_default();
                (o.id, (o.product_name, o.quantity, images))
            })
            .collect();

        // 计算每个工序的已完成数量（Approved + Settled）
        let completed_records = piece_record::Entity::find()
            .filter(piece_record::Column::ProcessId.is_in(process_ids.clone()))
            .filter(
                piece_record::Column::Status
                    .eq(piece_record::PieceRecordStatus::Approved)
                    .or(piece_record::Column::Status.eq(piece_record::PieceRecordStatus::Settled)),
            )
            .all(db)
            .await?;

        // 按工序ID汇总已完成数量
        let mut completed_map: std::collections::HashMap<Uuid, i32> =
            std::collections::HashMap::new();
        for record in completed_records {
            *completed_map.entry(record.process_id).or_insert(0) += record.quantity;
        }

        procs
            .into_iter()
            .map(|p| {
                let (product_name, order_quantity, images) = related_orders
                    .get(&p.order_id)
                    .cloned()
                    .unwrap_or_else(|| (String::new(), 0, vec![]));
                let completed = completed_map.get(&p.id).copied().unwrap_or(0);
                let remaining = std::cmp::Ord::max(order_quantity - completed, 0);

                PublicProcessInfo {
                    id: p.id,
                    name: p.name,
                    description: p.description,
                    piece_price: p.piece_price,
                    order_product_name: product_name,
                    order_images: images,
                    remaining_quantity: remaining,
                }
            })
            .collect()
    } else {
        vec![]
    };

    Ok(PublicShareResponse {
        title: share.title,
        description: share.description,
        workshop_name: ws.as_ref().map(|w| w.name.clone()),
        workshop_desc: ws.as_ref().and_then(|w| w.desc.clone()),
        workshop_address: ws.as_ref().and_then(|w| w.address.clone()),
        workshop_image: ws.as_ref().and_then(|w| w.image.clone()),
        boss_phone: Some(boss.phone),
        avatar: boss.avatar,
        processes,
    })
}
