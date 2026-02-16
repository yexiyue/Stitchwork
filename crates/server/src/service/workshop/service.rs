use sea_orm::{
    ColumnTrait, DbConn, EntityLoaderTrait, EntityTrait, ExprTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::common::{ListData, QueryParams};
use entity::{user, workshop};
use crate::error::{AppError, Result};
use crate::InviteCodes;

use super::dto::{
    BindWorkshopRequest, CreateWorkshopRequest, InviteCodeResponse, StaffResponse,
    UpdateWorkshopRequest, WorkshopResponse,
};

// 辅助函数：获取老板的工坊
async fn get_boss_workshop(db: &DbConn, boss_id: Uuid) -> Result<workshop::ModelEx> {
    workshop::Entity::load()
        .filter(workshop::Column::OwnerId.eq(boss_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("请先创建工坊".to_string()))
}

pub fn to_response(ws: &workshop::ModelEx) -> WorkshopResponse {
    WorkshopResponse {
        id: ws.id,
        name: ws.name.clone(),
        desc: ws.desc.clone(),
        address: ws.address.clone(),
        image: ws.image.clone(),
        piece_unit: ws.piece_unit.clone(),
        business_label: ws.business_label.clone(),
    }
}

pub async fn get_workshop(db: &DbConn, owner_id: Uuid) -> Result<Option<WorkshopResponse>> {
    let ws = workshop::Entity::load()
        .filter(workshop::Column::OwnerId.eq(owner_id))
        .one(db)
        .await?;
    Ok(ws.as_ref().map(to_response))
}

pub async fn create_workshop(
    db: &DbConn,
    owner_id: Uuid,
    req: CreateWorkshopRequest,
) -> Result<WorkshopResponse> {
    if workshop::Entity::find()
        .filter(workshop::Column::OwnerId.eq(owner_id))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("已创建工坊".to_string()));
    }

    let ws = workshop::ActiveModelEx::new()
        .set_id(Uuid::new_v4())
        .set_owner_id(owner_id)
        .set_name(req.name)
        .set_desc(req.desc)
        .set_address(req.address)
        .set_image(req.image)
        .set_piece_unit(req.piece_unit.unwrap_or_else(|| "打".to_string()))
        .set_business_label(req.business_label.unwrap_or_else(|| "工坊".to_string()))
        .set_created_at(chrono::Utc::now())
        .insert(db)
        .await?;

    Ok(to_response(&ws))
}

pub async fn update_workshop(
    db: &DbConn,
    owner_id: Uuid,
    req: UpdateWorkshopRequest,
) -> Result<WorkshopResponse> {
    let ws = get_boss_workshop(db, owner_id).await?;

    let mut active: workshop::ActiveModelEx = ws.into();
    if let Some(v) = req.name {
        active.name = Set(v);
    }
    if let Some(v) = req.desc {
        active.desc = Set(Some(v));
    }
    if let Some(v) = req.address {
        active.address = Set(Some(v));
    }
    if let Some(v) = req.image {
        active.image = Set(Some(v));
    }
    if let Some(v) = req.piece_unit {
        active.piece_unit = Set(v);
    }
    if let Some(v) = req.business_label {
        active.business_label = Set(v);
    }

    let ws = active.update(db).await?;
    Ok(to_response(&ws))
}

// 生成邀请码
pub async fn generate_invite_code(
    db: &DbConn,
    invite_codes: &InviteCodes,
    boss_id: Uuid,
) -> Result<InviteCodeResponse> {
    let ws = get_boss_workshop(db, boss_id).await?;

    // 使用 16 位邀请码增加安全性 (8位太短容易被暴力破解)
    let code = Uuid::new_v4().to_string().replace("-", "")[..16].to_string();
    let expires_at = chrono::Utc::now().timestamp() + 3600 * 24;

    invite_codes
        .write()
        .await
        .insert(code.clone(), (ws.id, expires_at));

    Ok(InviteCodeResponse { code, expires_at })
}

// 员工绑定工坊
pub async fn bind_workshop(
    db: &DbConn,
    invite_codes: &InviteCodes,
    staff_id: Uuid,
    req: BindWorkshopRequest,
) -> Result<()> {
    let mut codes = invite_codes.write().await;
    let (workshop_id, expires_at) = codes
        .remove(&req.invite_code)
        .ok_or_else(|| AppError::BadRequest("邀请码无效".to_string()))?;

    if chrono::Utc::now().timestamp() > expires_at {
        return Err(AppError::BadRequest("邀请码已过期".to_string()));
    }

    let staff = user::Entity::load()
        .filter_by_id(staff_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    if staff.workshop_id.is_some() {
        return Err(AppError::BadRequest("已绑定工坊".to_string()));
    }

    if staff.role != user::Role::Staff {
        return Err(AppError::BadRequest("用户角色错误".to_string()));
    }

    let active: user::ActiveModelEx = staff.into();
    active.set_workshop_id(workshop_id).update(db).await?;

    Ok(())
}

// 获取员工列表
pub async fn get_staff_list(
    db: &DbConn,
    boss_id: Uuid,
    params: QueryParams,
) -> Result<ListData<StaffResponse>> {
    let ws = get_boss_workshop(db, boss_id).await?;

    let mut query = user::Entity::find().filter(user::Column::WorkshopId.eq(ws.id));

    if let Some(ref search) = params.search {
        query = query.filter(
            user::Column::Username
                .contains(search)
                .or(user::Column::DisplayName.contains(search)),
        );
    }

    let order = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };
    query = query.order_by(user::Column::CreatedAt, order);

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.page.saturating_sub(1))
        .await?
        .into_iter()
        .map(|s| StaffResponse {
            id: s.id,
            username: s.username,
            display_name: s.display_name,
            phone: Some(s.phone),
            avatar: s.avatar,
        })
        .collect();

    Ok(ListData { list, total })
}

// 移除员工
pub async fn remove_staff(db: &DbConn, boss_id: Uuid, staff_id: Uuid) -> Result<()> {
    let ws = get_boss_workshop(db, boss_id).await?;

    let staff = user::Entity::load()
        .filter_by_id(staff_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("员工不存在".to_string()))?;

    if staff.workshop_id != Some(ws.id) {
        return Err(AppError::BadRequest("该员工不属于您的工坊".to_string()));
    }

    let active: user::ActiveModelEx = staff.into();
    active.set_workshop_id(None).update(db).await?;
    Ok(())
}
