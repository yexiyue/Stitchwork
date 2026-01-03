use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::{
    employment,
    user::{self, Role},
};
use crate::error::{AppError, Result};
use crate::InviteCodes;

use super::dto::{
    BindBossRequest, CreateStaffRequest, InviteCodeResponse, LoginRequest, LoginResponse,
    LoginUser, RegisterRequest, UpdateProfileRequest,
};
use super::jwt::create_token;

pub async fn login(db: &DbConn, req: LoginRequest) -> Result<LoginResponse> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("用户名或密码错误".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Internal("密码验证失败".to_string()))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::BadRequest("用户名或密码错误".to_string()))?;

    let token = create_token(user.id, user.role.clone())
        .map_err(|_| AppError::Internal("Token生成失败".to_string()))?;

    Ok(LoginResponse {
        token,
        user: LoginUser {
            id: user.id,
            username: user.username,
            role: user.role,
            display_name: user.display_name,
            phone: user.phone,
            avatar: user.avatar,
            workshop_name: user.workshop_name,
            workshop_desc: user.workshop_desc,
        },
    })
}

pub async fn register(db: &DbConn, req: RegisterRequest) -> Result<Uuid> {
    if user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    let password_hash = hash_password(&req.password)?;

    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(req.username),
        password_hash: Set(password_hash),
        role: Set(Role::Boss),
        display_name: Set(None),
        phone: Set(None),
        avatar: Set(None),
        workshop_name: Set(None),
        workshop_desc: Set(None),
        created_at: Set(chrono::Utc::now()),
    };

    let user = user.insert(db).await?;
    Ok(user.id)
}

// 老板创建员工账号
pub async fn create_staff(db: &DbConn, boss_id: Uuid, req: CreateStaffRequest) -> Result<Uuid> {
    if user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    let password_hash = hash_password(&req.password)?;

    let staff = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(req.username),
        password_hash: Set(password_hash),
        role: Set(Role::Staff),
        display_name: Set(req.display_name),
        phone: Set(req.phone),
        avatar: Set(None),
        workshop_name: Set(None),
        workshop_desc: Set(None),
        created_at: Set(chrono::Utc::now()),
    }
    .insert(db)
    .await?;

    // 创建雇佣关系
    employment::ActiveModel {
        id: Set(Uuid::new_v4()),
        boss_id: Set(boss_id),
        staff_id: Set(staff.id),
        created_at: Set(chrono::Utc::now()),
    }
    .insert(db)
    .await?;

    Ok(staff.id)
}

// 生成邀请码
pub async fn generate_invite_code(
    invite_codes: &Arc<InviteCodes>,
    boss_id: Uuid,
) -> InviteCodeResponse {
    let code = Uuid::new_v4().to_string()[..8].to_string();
    let expires_at = chrono::Utc::now().timestamp() + 3600 * 24; // 24小时有效

    invite_codes
        .write()
        .await
        .insert(code.clone(), (boss_id, expires_at));

    InviteCodeResponse { code, expires_at }
}

// 员工绑定老板
pub async fn bind_boss(
    db: &DbConn,
    invite_codes: &Arc<InviteCodes>,
    staff_id: Uuid,
    req: BindBossRequest,
) -> Result<()> {
    let mut codes = invite_codes.write().await;
    let (boss_id, expires_at) = codes
        .remove(&req.invite_code)
        .ok_or_else(|| AppError::BadRequest("邀请码无效".to_string()))?;

    if chrono::Utc::now().timestamp() > expires_at {
        return Err(AppError::BadRequest("邀请码已过期".to_string()));
    }

    // 检查是否已绑定
    if employment::Entity::find()
        .filter(employment::Column::BossId.eq(boss_id))
        .filter(employment::Column::StaffId.eq(staff_id))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("已绑定该老板".to_string()));
    }

    employment::ActiveModel {
        id: Set(Uuid::new_v4()),
        boss_id: Set(boss_id),
        staff_id: Set(staff_id),
        created_at: Set(chrono::Utc::now()),
    }
    .insert(db)
    .await?;

    Ok(())
}

pub async fn update_profile(db: &DbConn, user_id: Uuid, req: UpdateProfileRequest) -> Result<()> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    let mut active: user::ActiveModel = user.into();
    if let Some(v) = req.display_name {
        active.display_name = Set(Some(v));
    }
    if let Some(v) = req.phone {
        active.phone = Set(Some(v));
    }
    if let Some(v) = req.avatar {
        active.avatar = Set(Some(v));
    }
    if let Some(v) = req.workshop_name {
        active.workshop_name = Set(Some(v));
    }
    if let Some(v) = req.workshop_desc {
        active.workshop_desc = Set(Some(v));
    }
    active.update(db).await?;
    Ok(())
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("密码加密失败".to_string()))
        .map(|h| h.to_string())
}
