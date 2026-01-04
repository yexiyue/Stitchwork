# boss-records-list-ui Specification

## Purpose

增强老板端计件列表卡片，显示订单图片并支持点击进入详情。

## MODIFIED Requirements

### Requirement: Boss Piece Record List Card

老板端计件列表卡片 SHALL 显示订单图片。

#### Scenario: 卡片显示订单图片
- Given 计件记录有关联的订单图片
- When 渲染列表卡片
- Then 卡片左侧显示订单图片缩略图（56×56px）
- And 卡片右侧显示：员工名、订单·工序、数量·金额、状态、时间

#### Scenario: 订单无图片
- Given 计件记录关联的订单没有图片
- When 渲染列表卡片
- Then 左侧显示占位图标

#### Scenario: 点击卡片进入详情
- Given 老板在计件列表页
- When 点击某条计件卡片
- Then 跳转到该计件的详情页 `/records/$id`

## Cross-references

- staff-piece-record-api: orderImage 字段已实现
