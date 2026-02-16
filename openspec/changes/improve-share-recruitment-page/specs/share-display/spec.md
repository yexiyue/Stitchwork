# share-display Specification

## Purpose

定义招工分享页面的展示需求，专注于为求职者提供有用的招工信息。

## MODIFIED Requirements

### Requirement: Public Share Access

系统 SHALL 在公开分享页面展示招工信息。

#### Scenario: 访问招工页面

- **WHEN** 请求 `GET /public/share/{token}`
- **AND** token 有效且分享处于激活状态
- **THEN** 返回以下信息：
  - 分享标题
  - 工坊名称
  - 工坊地址
  - 老板联系方式（手机号）
  - 老板头像
  - 工序列表（按 process_ids 筛选）

#### Scenario: 工序信息展示

- **GIVEN** 招工页面已加载
- **THEN** 每个工序展示：
  - 工序名称
  - 单价（piece_price）
  - 剩余数量（订单数量 - 已完成数量）
  - 所属产品名称

#### Scenario: 剩余数量计算

- **GIVEN** 工序属于某订单
- **AND** 订单数量为 N
- **AND** 该工序已审核通过的计件记录总数为 M
- **THEN** 剩余数量 = N - M
- **AND** 剩余数量 >= 0

### Requirement: Contact Information

系统 SHALL 支持求职者联系老板。

#### Scenario: 拨打电话

- **GIVEN** 用户在招工页面
- **WHEN** 点击老板手机号
- **THEN** 调起系统电话拨号

#### Scenario: 无联系方式

- **GIVEN** 老板未设置手机号
- **THEN** 不展示联系方式区域

### Requirement: Share Page Accessibility

招工页面 SHALL 为公开访问，无需登录。

#### Scenario: 无认证访问

- **GIVEN** 用户未登录
- **WHEN** 访问 `/share/{token}`
- **THEN** 正常展示招工信息
- **AND** 不显示登录提示

### Requirement: Image Export

系统 SHALL 支持将招工信息导出为图片。

#### Scenario: 导出招工图片

- **GIVEN** 用户在招工页面
- **WHEN** 点击导出按钮
- **THEN** 生成包含以下内容的图片：
  - 工坊名称和头像
  - 联系方式和地址
  - 工序列表（名称、单价、剩余数量）
  - 底部二维码（指向当前页面）
- **AND** 提示用户保存图片

#### Scenario: 二维码扫描

- **GIVEN** 用户扫描导出图片中的二维码
- **THEN** 跳转到对应的招工页面

## Related Capabilities

- `share`: 分享创建和管理
