# staff-piece-record-api Specification

## Purpose

扩展计件记录 API，支持返回订单图片字段，便于员工端展示。

## MODIFIED Requirements

### Requirement: Piece Record List Response

计件记录列表 SHALL 返回订单图片字段

#### Scenario: 返回订单封面图
- Given 计件记录关联的订单有图片
- When 调用 `GET /piece-records` 列表接口
- Then 每条记录包含 `orderImage` 字段
- And `orderImage` 为订单 `images` 数组的第一个元素

#### Scenario: 订单无图片
- Given 计件记录关联的订单没有图片
- When 调用列表接口
- Then `orderImage` 字段为 null

## Cross-references

- piece_record entity
- order entity (images 字段)
