use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityLoaderTrait, EntityTrait, ExprTrait, QueryFilter,
    Set,
};
use uuid::Uuid;

use entity::{
    register_code,
    user::{self, Role},
    workshop,
};
use crate::error::{AppError, Result};
use crate::service::notification::{Notification, Notifier};
use crate::service::workshop::service::to_response;

use super::dto::{
    ChangePasswordRequest, LoginRequest, LoginResponse, LoginUser, RegisterRequest,
    UpdateProfileRequest, WorkshopResponse,
};
use super::jwt::create_token;

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

pub async fn register(
    db: &DbConn,
    notifier: &Notifier,
    req: RegisterRequest,
) -> Result<LoginResponse> {
    // 验证注册码
    let code = register_code::Entity::find()
        .filter(register_code::Column::Code.eq(&req.code))
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("注册码无效".to_string()))?;

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

    // 根据 workshop_id 决定角色：有值 = Staff，无值 = Boss
    let (role, workshop_id) = match code.workshop_id {
        Some(wid) => (Role::Staff, Some(wid)),
        None => (Role::Boss, None),
    };

    let new_user = user::ActiveModel {
        id: Set(user_id),
        username: Set(req.username),
        password_hash: Set(password_hash),
        role: Set(role),
        display_name: Set(None),
        phone: Set(req.phone),
        avatar: Set(None),
        workshop_id: Set(workshop_id),
        is_super_admin: Set(false),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    // 标记注册码为已使用
    let mut code_active: register_code::ActiveModel = code.into();
    code_active.used_by = Set(Some(user_id));
    code_active.used_at = Set(Some(chrono::Utc::now()));
    code_active.update(db).await?;

    // 生成 token（统一自动登录）
    let token = create_token(user.id, user.role)
        .map_err(|_| AppError::Internal("Token生成失败".to_string()))?;

    // 获取工坊信息并发送通知
    let ws = match workshop_id {
        Some(wid) => {
            let ws = workshop::Entity::load().filter_by_id(wid).one(db).await?;
            // 通知工坊老板有新员工加入
            if let Some(ref workshop) = ws {
                notifier.send(
                    workshop.owner_id,
                    Notification::StaffJoined {
                        username: username.clone(),
                        phone: phone.clone(),
                    },
                );
            }
            ws
        }
        None => {
            // 通知所有超管有新老板注册
            let super_admins = user::Entity::find()
                .filter(user::Column::IsSuperAdmin.eq(true))
                .all(db)
                .await?;
            let admin_ids: Vec<Uuid> = super_admins.iter().map(|u| u.id).collect();
            notifier.send_many(
                &admin_ids,
                Notification::UserRegistered {
                    username: username.clone(),
                    phone: phone.clone(),
                },
            );
            None
        }
    };

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
    if let Some(ref new_phone) = req.phone
        && new_phone != &user.phone {
            let existing = user::Entity::find()
                .filter(user::Column::Phone.eq(new_phone))
                .filter(user::Column::Id.ne(user_id))
                .one(db)
                .await?;
            if existing.is_some() {
                return Err(AppError::BadRequest("手机号已被使用".to_string()));
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

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("密码加密失败".to_string()))
        .map(|h| h.to_string())
}
