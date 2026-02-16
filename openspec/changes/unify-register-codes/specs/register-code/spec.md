## ADDED Requirements

### Requirement: Register Code Entity
系统 SHALL 维护统一的 `register_code` 数据表，包含以下字段：`id`(UUID), `code`(6位数字字符串, unique), `created_by`(UUID, 创建者), `workshop_id`(Option<UUID>, 关联工坊), `used_by`(Option<UUID>), `used_at`(Option<DateTime>), `created_at`(DateTime)。

#### Scenario: 超管创建的码无 workshop_id
- **WHEN** 超管创建注册码
- **THEN** `created_by` 为超管 ID，`workshop_id` 为 NULL

#### Scenario: Boss 创建的码关联 workshop
- **WHEN** Boss 创建注册码
- **THEN** `created_by` 为 Boss ID，`workshop_id` 为该 Boss 的工坊 ID

### Requirement: 6-Digit Code Generation
系统 SHALL 生成 6 位纯数字注册码（000000-999999），确保与数据库中已有码不重复。

#### Scenario: 生成唯一码
- **WHEN** 系统生成新注册码
- **THEN** 码为 6 位纯数字
- **AND** 码在 `register_code` 表中不存在

#### Scenario: 碰撞重试
- **WHEN** 生成的码已存在于数据库
- **THEN** 系统自动重试生成新码直到唯一

### Requirement: Create Register Code API
系统 SHALL 提供 `POST /api/register-codes` 接口，超管和 Boss 均可调用。

#### Scenario: 超管创建 Boss 注册码
- **WHEN** 超管调用创建接口
- **THEN** 创建 `workshop_id = NULL` 的注册码
- **AND** 返回码详情

#### Scenario: Boss 创建 Staff 注册码
- **WHEN** Boss 调用创建接口
- **THEN** 创建 `workshop_id = Boss 的工坊 ID` 的注册码
- **AND** 返回码详情

#### Scenario: Boss 未绑定工坊时创建失败
- **WHEN** Boss 未创建工坊就调用创建接口
- **THEN** 返回错误「请先创建工坊」

#### Scenario: Staff 角色无权创建
- **WHEN** Staff 角色调用创建接口
- **THEN** 返回 403 权限不足

### Requirement: List Register Codes API
系统 SHALL 提供 `GET /api/register-codes` 接口，返回当前用户有权查看的注册码列表（分页）。

#### Scenario: 超管查看所有 Boss 码
- **WHEN** 超管调用列表接口
- **THEN** 返回所有 `workshop_id = NULL` 的注册码
- **AND** 包含使用者用户名（如已使用）

#### Scenario: Boss 查看自己创建的 Staff 码
- **WHEN** Boss 调用列表接口
- **THEN** 仅返回 `created_by = 当前 Boss ID` 的注册码

#### Scenario: 分页支持
- **WHEN** 请求包含 page 和 page_size 参数
- **THEN** 返回对应页数据和总数

### Requirement: Delete Register Code API
系统 SHALL 提供 `DELETE /api/register-codes/{id}` 接口，仅允许删除未使用的码。

#### Scenario: 删除未使用的码
- **WHEN** 用户删除一个 `used_by = NULL` 的码
- **THEN** 从数据库物理删除该记录

#### Scenario: 删除已使用的码被拒绝
- **WHEN** 用户尝试删除 `used_by` 不为空的码
- **THEN** 返回错误「已使用的注册码不可删除」

#### Scenario: 只能删除自己创建的码
- **WHEN** 用户尝试删除非自己创建的码
- **THEN** 返回 404 或 403

### Requirement: Unified Registration Endpoint
系统 SHALL 提供统一的 `POST /api/register` 接口，根据注册码的 `workshop_id` 自动决定用户角色。

#### Scenario: 使用 Boss 码注册
- **WHEN** 提交的注册码 `workshop_id = NULL`
- **THEN** 创建 Boss 角色用户
- **AND** 标记码为已使用
- **AND** 返回 `LoginResponse { token, user }`（自动登录）
- **AND** 通知所有超管

#### Scenario: 使用 Staff 码注册
- **WHEN** 提交的注册码 `workshop_id` 有值
- **THEN** 创建 Staff 角色用户，`workshop_id` 绑定到对应工坊
- **AND** 标记码为已使用
- **AND** 返回 `LoginResponse { token, user }`（自动登录）
- **AND** 通知工坊老板

#### Scenario: 无效码注册失败
- **WHEN** 提交的注册码不存在或已被使用
- **THEN** 返回错误「注册码无效或已被使用」

#### Scenario: 用户名或手机号重复
- **WHEN** 提交的用户名或手机号已存在
- **THEN** 返回对应错误信息
