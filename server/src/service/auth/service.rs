use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityLoaderTrait, EntityTrait, ExprTrait, QueryFilter,
    Set,
};
use uuid::Uuid;

use crate::entity::{
    register_code,
    user::{self, Role},
    workshop,
};
use crate::error::{AppError, Result};
use crate::service::notification::{Notification, SharedNotifier};
use crate::service::workshop::service::to_response;

use std::sync::Arc;

use super::dto::{
    ChangePasswordRequest, LoginRequest, LoginResponse, LoginUser, RegisterRequest,
    RegisterStaffRequest, UpdateProfileRequest, WorkshopResponse,
};
use super::jwt::create_token;
use crate::InviteCodes;

pub async fn login(db: &DbConn, req: LoginRequest) -> Result<LoginResponse> {
    // 支持用户名或手机号登录
    let user = user::Entity::find()
        .filter(
            user::Column::Username
                .eq(&req.username)
                .or(user::Column::Phone.eq(&req.username)),
        )
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("用户名或密码错误".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Internal("密码验证失败".to_string()))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::BadRequest("用户名或密码错误".to_string()))?;

    let token = create_token(user.id, user.role)
        .map_err(|_| AppError::Internal("Token生成失败".to_string()))?;

    let workshop = get_workshop_for_user(db, &user).await?;

    Ok(LoginResponse {
        token,
        user: LoginUser {
            id: user.id,
            username: user.username,
            role: user.role,
            display_name: user.display_name,
            phone: user.phone,
            avatar: user.avatar,
            is_super_admin: user.is_super_admin,
            workshop,
        },
    })
}

pub async fn register(db: &DbConn, notifier: &SharedNotifier, req: RegisterRequest) -> Result<Uuid> {
    // 验证注册码
    let code = register_code::Entity::find()
        .filter(register_code::Column::Code.eq(&req.register_code))
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("注册码无效".to_string()))?;

    if !code.is_active {
        return Err(AppError::BadRequest("注册码已禁用".to_string()));
    }

    if code.used_by.is_some() {
        return Err(AppError::BadRequest("注册码已被使用".to_string()));
    }

    // 检查用户名是否已存在
    if user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    // 检查手机号是否已存在
    if user::Entity::find()
        .filter(user::Column::Phone.eq(&req.phone))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("手机号已被使用".to_string()));
    }

    let password_hash = hash_password(&req.password)?;
    let user_id = Uuid::new_v4();

    let username = req.username.clone();
    let phone = req.phone.clone();

    let user = user::ActiveModel {
        id: Set(user_id),
        username: Set(req.username),
        password_hash: Set(password_hash),
        role: Set(Role::Boss),
        display_name: Set(None),
        phone: Set(req.phone),
        avatar: Set(None),
        workshop_id: Set(None),
        is_super_admin: Set(false),
        created_at: Set(chrono::Utc::now()),
    };

    let user = user.insert(db).await?;

    // 标记注册码为已使用
    let mut code_active: register_code::ActiveModel = code.into();
    code_active.used_by = Set(Some(user_id));
    code_active.used_at = Set(Some(chrono::Utc::now()));
    code_active.update(db).await?;

    // 通知所有超管
    let super_admins = user::Entity::find()
        .filter(user::Column::IsSuperAdmin.eq(true))
        .all(db)
        .await?;
    let admin_ids: Vec<Uuid> = super_admins.iter().map(|u| u.id).collect();
    notifier.send_many(&admin_ids, Notification::UserRegistered { username, phone });

    Ok(user.id)
}

pub async fn register_staff(
    db: &DbConn,
    invite_codes: &Arc<InviteCodes>,
    notifier: &SharedNotifier,
    req: RegisterStaffRequest,
) -> Result<LoginResponse> {
    // 验证邀请码
    let mut codes = invite_codes.write().await;
    let (workshop_id, expires_at) = codes
        .remove(&req.invite_code)
        .ok_or_else(|| AppError::BadRequest("邀请码无效".to_string()))?;

    if chrono::Utc::now().timestamp() > expires_at {
        return Err(AppError::BadRequest("邀请码已过期".to_string()));
    }
    drop(codes);

    // 检查用户名是否已存在
    if user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    // 检查手机号是否已存在
    if user::Entity::find()
        .filter(user::Column::Phone.eq(&req.phone))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("手机号已被使用".to_string()));
    }

    let password_hash = hash_password(&req.password)?;

    let username = req.username.clone();
    let phone = req.phone.clone();

    // 创建 Staff 用户并绑定工坊
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(req.username.clone()),
        password_hash: Set(password_hash),
        role: Set(Role::Staff),
        display_name: Set(None),
        phone: Set(req.phone),
        avatar: Set(None),
        workshop_id: Set(Some(workshop_id)),
        is_super_admin: Set(false),
        created_at: Set(chrono::Utc::now()),
    };

    let user = new_user.insert(db).await?;

    // 生成 token
    let token = create_token(user.id, user.role)
        .map_err(|_| AppError::Internal("Token生成失败".to_string()))?;

    // 获取工坊信息
    let ws = workshop::Entity::load()
        .filter_by_id(workshop_id)
        .one(db)
        .await?;

    // 通知工坊老板
    if let Some(ref workshop) = ws {
        notifier.send(workshop.owner_id, Notification::StaffJoined { username, phone });
    }

    Ok(LoginResponse {
        token,
        user: LoginUser {
            id: user.id,
            username: user.username,
            role: user.role,
            display_name: user.display_name,
            phone: user.phone,
            avatar: user.avatar,
            is_super_admin: user.is_super_admin,
            workshop: ws.as_ref().map(to_response),
        },
    })
}

pub async fn update_profile(db: &DbConn, user_id: Uuid, req: UpdateProfileRequest) -> Result<()> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 检查手机号唯一性
    if let Some(ref new_phone) = req.phone {
        if new_phone != &user.phone {
            let existing = user::Entity::find()
                .filter(user::Column::Phone.eq(new_phone))
                .filter(user::Column::Id.ne(user_id))
                .one(db)
                .await?;
            if existing.is_some() {
                return Err(AppError::BadRequest("手机号已被使用".to_string()));
            }
        }
    }

    let mut active: user::ActiveModel = user.into();
    if let Some(v) = req.display_name {
        active.display_name = Set(Some(v));
    }
    if let Some(v) = req.phone {
        active.phone = Set(v);
    }
    if let Some(v) = req.avatar {
        active.avatar = Set(Some(v));
    }
    active.update(db).await?;
    Ok(())
}

pub async fn change_password(
    db: &DbConn,
    user_id: Uuid,
    req: ChangePasswordRequest,
) -> Result<()> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 验证旧密码
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Internal("密码验证失败".to_string()))?;

    Argon2::default()
        .verify_password(req.old_password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::BadRequest("旧密码错误".to_string()))?;

    // 更新新密码
    let new_hash = hash_password(&req.new_password)?;
    let mut active: user::ActiveModel = user.into();
    active.password_hash = Set(new_hash);
    active.update(db).await?;

    Ok(())
}

pub async fn get_profile(db: &DbConn, user_id: Uuid) -> Result<LoginUser> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    let workshop = get_workshop_for_user(db, &user).await?;

    Ok(LoginUser {
        id: user.id,
        username: user.username,
        role: user.role,
        display_name: user.display_name,
        phone: user.phone,
        avatar: user.avatar,
        is_super_admin: user.is_super_admin,
        workshop,
    })
}

// 获取用户关联的工坊信息（简化版：一次查询）
async fn get_workshop_for_user(
    db: &DbConn,
    user: &user::Model,
) -> Result<Option<WorkshopResponse>> {
    let ws = if user.role == Role::Staff {
        // 员工：通过 workshop_id 查找
        match user.workshop_id {
            Some(id) => workshop::Entity::load().filter_by_id(id).one(db).await?,
            None => None,
        }
    } else {
        // 老板：查找自己拥有的工坊
        workshop::Entity::load()
            .filter(workshop::Column::OwnerId.eq(user.id))
            .one(db)
            .await?
    };

    Ok(ws.as_ref().map(to_response))
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("密码加密失败".to_string()))
        .map(|h| h.to_string())
}
