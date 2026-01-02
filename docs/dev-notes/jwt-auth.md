# Auth 服务完整实现

## 文件结构

```
server/src/
├── entity/
│   └── user.rs           # 用户实体
└── service/
    └── auth/
        ├── mod.rs        # 模块导出
        ├── dto.rs        # 请求/响应结构
        ├── service.rs    # 业务逻辑
        ├── controller.rs # 路由处理
        └── jwt.rs        # JWT 工具
```

## 依赖

```toml
jsonwebtoken = "9"
argon2 = "0.5"
```

## 1. User 实体

```rust
// entity/user.rs
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime,
}
```

## 2. DTO

```rust
// auth/dto.rs
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}
```

## 3. JWT 工具

```rust
// auth/jwt.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,  // user_id
    pub exp: usize, // 过期时间戳
}

fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_in_production".to_string())
        .into_bytes()
}

pub fn create_token(user_id: Uuid) -> Result<String, Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .timestamp() as usize;

    encode(&Header::default(), &Claims { sub: user_id, exp }, &EncodingKey::from_secret(&get_secret()))
}

pub fn verify_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(&get_secret()), &Validation::default())
        .map(|data| data.claims)
}

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request.headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .filter(|h| h.starts_with("Bearer "))
        .map(|h| &h[7..])
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
```

## 4. Service 层

```rust
// auth/service.rs
pub async fn login(db: &DbConn, req: LoginRequest) -> Result<LoginResponse> {
    // 1. 查找用户
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db).await?
        .ok_or_else(|| AppError::BadRequest("用户名或密码错误".to_string()))?;

    // 2. 验证密码 (Argon2)
    let parsed_hash = PasswordHash::new(&user.password_hash)?;
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::BadRequest("用户名或密码错误".to_string()))?;

    // 3. 生成 token
    let token = create_token(user.id)?;
    Ok(LoginResponse { token, user_id: user.id })
}

pub async fn register(db: &DbConn, req: RegisterRequest) -> Result<Uuid> {
    // 1. 检查用户名是否已存在
    if user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db).await?.is_some() {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    // 2. 密码哈希 (Argon2 + 随机盐)
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)?
        .to_string();

    // 3. 创建用户
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(req.username),
        password_hash: Set(password_hash),
        created_at: Set(chrono::Utc::now()),
    }.insert(db).await?;

    Ok(user.id)
}
```

## 5. Controller 层

```rust
// auth/controller.rs
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(
    State(state): State<Arc<AppState>>,
    AppJson(req): AppJson<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>> {
    let res = service::login(&state.db, req).await?;
    Ok(ApiResponse::ok(res))
}

async fn register(
    State(state): State<Arc<AppState>>,
    AppJson(req): AppJson<RegisterRequest>,
) -> Result<ApiResponse<serde_json::Value>> {
    let user_id = service::register(&state.db, req).await?;
    Ok(ApiResponse::ok(serde_json::json!({ "user_id": user_id })))
}
```

## 6. 路由集成

```rust
// service/mod.rs
pub fn routes() -> Router<Arc<AppState>> {
    let protected = Router::new()
        .merge(customer::router())
        .merge(worker::router())
        // ... 其他需要认证的路由
        .layer(middleware::from_fn(auth::auth_middleware));

    Router::new().nest("/api", Router::new()
        .merge(auth::router())  // 登录注册不需要认证
        .merge(protected),
    )
}
```

## 7. 在 Handler 中获取当前用户

```rust
use axum::Extension;
use crate::service::auth::Claims;

async fn some_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    let current_user_id = claims.sub;
    // ...
}
```

## API

| 方法 | 路径 | 描述 | 认证 |
|------|------|------|------|
| POST | /api/register | 注册 | 否 |
| POST | /api/login | 登录 | 否 |

## 配置

```env
JWT_SECRET=your_strong_secret_key_here
```

## 使用示例

```bash
# 注册
curl -X POST http://localhost:3000/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"123456"}'

# 登录
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"123456"}'

# 访问受保护接口
curl http://localhost:3000/api/customers \
  -H "Authorization: Bearer <token>"
```
