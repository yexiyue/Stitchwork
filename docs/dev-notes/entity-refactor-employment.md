# 实体重构：Worker → Employment 中间表

## 背景

原设计中 `worker` 是独立实体，有自己的 `id`、`name`、`phone`，同时关联 `user_id`。这导致：
- 数据冗余：worker 和 user 都存储用户信息
- 关系混乱：user → worker → piece_record/payroll

## 重构方案

### 删除 worker 实体

将 worker 的 `name`、`phone` 字段移到 user 表，piece_record 和 payroll 直接关联 user。

### 新增 employment 中间表

用于表达 boss-staffs 雇佣关系，替代原来 user 表的 `boss_id` 自引用字段。

## 新实体结构

### user.rs

```rust
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub password_hash: String,
    pub role: Role,
    pub display_name: Option<String>,  // 从 worker 移入
    pub phone: Option<String>,          // 从 worker 移入
    pub created_at: DateTime,

    #[sea_orm(has_many)]
    pub customers: HasMany<super::customer::Entity>,

    #[sea_orm(has_many)]
    pub piece_records: HasMany<super::piece_record::Entity>,

    #[sea_orm(has_many)]
    pub payrolls: HasMany<super::payroll::Entity>,

    #[sea_orm(self_ref, via = "employment", from = "Boss", to = "Staff")]
    pub staffs: HasMany<Entity>,
}
```

### employment.rs

```rust
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "employment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub boss_id: Uuid,
    pub staff_id: Uuid,
    pub created_at: DateTime,

    #[sea_orm(belongs_to, relation_enum = "Boss", from = "boss_id", to = "id")]
    pub boss: Option<super::user::Entity>,
    #[sea_orm(belongs_to, relation_enum = "Staff", from = "staff_id", to = "id")]
    pub staffs: Option<super::user::Entity>,
}
```

### piece_record.rs / payroll.rs

```rust
// worker_id 改为 user_id
pub user_id: Uuid,

#[sea_orm(belongs_to, from = "user_id", to = "id")]
pub user: HasOne<super::user::Entity>,
```

## 关键点

1. **relation_enum** - 当中间表有多个外键指向同一实体时必须指定，用于区分不同的关系
2. **self_ref** - 自引用关系需要配合 `via` 指定中间表，`from`/`to` 指定关系方向
3. **belongs_to 类型** - 中间表的 belongs_to 字段可以用 `HasOne<Entity>` 或 `Option<Entity>`，两者都可以

## 变更文件

- 删除: `entity/worker.rs`, `service/worker/*`
- 新增: `entity/employment.rs`
- 修改: `entity/user.rs`, `entity/piece_record.rs`, `entity/payroll.rs`
- 修改: `service/piece_record/dto.rs`, `service/payroll/dto.rs` (worker_id → user_id)
- 修改: `service/auth/service.rs` (注册时添加新字段)
