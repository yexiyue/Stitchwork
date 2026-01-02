# SeaORM 2.0 实体优先模式集成指南

## 概述

SeaORM 2.0 引入了 **Entity First Workflow（实体优先工作流）**，允许你先定义 Rust 实体，然后自动生成数据库表结构。这与传统的 Schema First（先建表再生成实体）形成互补。

## 核心特性

- 定义实体后自动检测变更
- 自动创建表、列、唯一键、外键
- 支持 dense 格式的简洁实体定义
- 拓扑排序处理外键依赖
- ActiveModel Builder 嵌套创建
- Entity Loader 关联查询
- 新的 COLUMN 常量（类型安全）

## 依赖配置

```toml
[dependencies]
sea-orm = { version = "2", features = [
    "sqlx-postgres",
    "runtime-tokio-rustls",
    "entity-registry",   # 实体注册
    "schema-sync"        # Schema 同步
] }
```

## 实体定义

### Dense 格式（2.0 新增）

```rust
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub phone: Option<String>,
    pub created_at: DateTime,

    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
```

## Schema 同步

```rust
let db = Database::connect("postgres://user:pass@localhost/stitchwork").await?;

// 自动同步所有实体到数据库
// SeaORM 会按拓扑顺序创建表（处理外键依赖）
db.get_schema_registry("my_crate::entity::*")
    .sync(&db)
    .await?;
```

注意：`"my_crate"` 必须与 `Cargo.toml` 中的 `name` 一致（`-` 转为 `_`）。

## Schema Sync 功能

### 添加表

新增 Entity 后，下次启动自动创建表：

```sql
CREATE TABLE "upvote" ( "id" integer NOT NULL PRIMARY KEY, .. )
```

### 添加列

```rust
pub struct Model {
    // ...
    pub date_of_birth: Option<DateTimeUtc>, // ⬅ 新列
}
```

执行：`ALTER TABLE "profile" ADD COLUMN "date_of_birth" timestamp`

### 添加非空列（带默认值）

```rust
#[sea_orm(default_value = 0)]
pub post_count: i32,

#[sea_orm(default_expr = "Expr::current_timestamp()")]
pub updated_at: DateTimeUtc,
```

### 重命名列

```rust
#[sea_orm(renamed_from = "date_of_birth")]
pub dob: Option<DateTimeUtc>,
```

执行：`ALTER TABLE "profile" RENAME COLUMN "date_of_birth" TO "dob"`

### 添加唯一约束

```rust
#[sea_orm(unique)]
pub email: String,
```

执行：`CREATE UNIQUE INDEX "idx-user-email" ON "user" ("email")`

## ActiveModel Builder（2.0 新增）

### 嵌套创建

```rust
// 创建用户同时创建 profile
let bob = user::ActiveModel::builder()
    .set_name("Bob")
    .set_email("bob@example.com")
    .set_profile(
        profile::ActiveModel::builder().set_picture("avatar.jpg")
    )
    .insert(db)
    .await?;
```

### 添加关联数据

```rust
// 给用户添加多个订单
bob.orders
    .push(order::ActiveModel::builder().set_product_name("产品A"))
    .push(order::ActiveModel::builder().set_product_name("产品B"));
bob.save(db).await?;
```

## Entity Loader（2.0 新增）

### 关联查询

```rust
// 查询用户及其订单（单次查询）
let user = user::Entity::load()
    .filter(user::COLUMN.name.eq("Bob"))
    .with(order::Entity)
    .one(db)
    .await?;

// 访问关联数据
for order in &user.orders {
    println!("{}", order.product_name);
}
```

### 唯一键查询

```rust
// 自动生成 filter_by_xxx 方法
let user = user::Entity::load()
    .filter_by_email("bob@example.com")
    .one(db)
    .await?;
```

## 新 COLUMN 常量（2.0 新增）

```rust
// 旧写法：CamelCase 枚举
user::Entity::find().filter(user::Column::Name.contains("Bob"))

// 新写法：类型安全的常量
user::Entity::find().filter(user::COLUMN.name.contains("Bob"))

// 编译时类型检查
user::Entity::find().filter(user::COLUMN.name.like(2)) // ❌ 编译错误
```

## 关系类型

### 一对一 (1-1)

```rust
#[sea_orm(has_one)]
pub profile: HasOne<super::profile::Entity>,

// 反向
#[sea_orm(belongs_to, from = "user_id", to = "id")]
pub user: HasOne<super::user::Entity>,
```

### 一对多 (1-N)

```rust
#[sea_orm(has_many)]
pub orders: HasMany<super::order::Entity>,

// 反向
#[sea_orm(belongs_to, from = "customer_id", to = "id")]
pub customer: HasOne<super::customer::Entity>,
```

### 多对多 (M-N)

```rust
// 通过中间表
#[sea_orm(has_many, via = "post_tag")]
pub tags: HasMany<super::tag::Entity>,
```

中间表定义：

```rust
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post_tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i32,
    #[sea_orm(belongs_to, from = "post_id", to = "id")]
    pub post: Option<super::post::Entity>,
    #[sea_orm(belongs_to, from = "tag_id", to = "id")]
    pub tag: Option<super::tag::Entity>,
}
```

## 常用属性

| 属性 | 说明 |
|------|------|
| `#[sea_orm(primary_key)]` | 主键（默认自增） |
| `#[sea_orm(primary_key, auto_increment = false)]` | 非自增主键 |
| `#[sea_orm(unique)]` | 唯一约束 |
| `#[sea_orm(column_type = "...")]` | 指定列类型 |
| `#[sea_orm(default_value = "...")]` | 默认值 |
| `#[sea_orm(default_expr = "...")]` | 默认表达式 |
| `#[sea_orm(renamed_from = "...")]` | 重命名列 |
| `#[sea_orm(has_one)]` | 一对一关系 |
| `#[sea_orm(has_many)]` | 一对多关系 |
| `#[sea_orm(belongs_to)]` | 多对一关系 |
| `#[sea_orm(has_many, via = "...")]` | 多对多关系 |

## 类型映射

| Rust 类型 | PostgreSQL 类型 |
|-----------|-----------------|
| `i32` | INTEGER |
| `i64` | BIGINT |
| `String` | VARCHAR |
| `bool` | BOOLEAN |
| `Uuid` | UUID |
| `DateTime` | TIMESTAMP |
| `Decimal` | DECIMAL |
| `Option<T>` | NULLABLE |

## 参考

- [SeaORM 官方文档](https://www.sea-ql.org/SeaORM/)
- [Entity First 文档](https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-first/)
- [SeaORM 2.0 攻略](https://www.sea-ql.org/blog/2025-12-05-sea-orm-2.0/)
- [GitHub 示例](https://github.com/SeaQL/sea-orm/tree/master/examples/basic)
