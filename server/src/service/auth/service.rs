use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entity::{
    user::{self, Role},
    workshop,
};
use crate::error::{AppError, Result};
use crate::service::workshop::service::to_response;

use super::dto::{
    LoginRequest, LoginResponse, LoginUser, RegisterRequest, UpdateProfileRequest, WorkshopResponse,
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
            workshop,
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
        workshop_id: Set(None),
        created_at: Set(chrono::Utc::now()),
    };

    let user = user.insert(db).await?;
    Ok(user.id)
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
            Some(id) => workshop::Entity::find_by_id(id).one(db).await?,
            None => None,
        }
    } else {
        // 老板：查找自己拥有的工坊
        workshop::Entity::find()
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
