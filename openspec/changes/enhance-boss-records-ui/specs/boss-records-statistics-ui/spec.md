# boss-records-statistics-ui Specification

## Purpose

提供老板端计件统计页面，直观展示员工产量。

## ADDED Requirements

### Requirement: Boss Records Statistics Page

老板端 SHALL 提供计件统计页面 `/records/stats`

#### Scenario: 入口按钮
- Given 老板在计件列表页
- When 查看页面
- Then 顶部显示"统计"入口按钮

#### Scenario: 员工产量统计
- Given 老板访问统计页面
- When 页面加载
- Then 显示每个员工的计件统计
- And 包含：员工名称、总数量、总金额

#### Scenario: 日期范围筛选
- Given 老板在统计页面
- When 选择开始日期和结束日期
- Then 统计数据按所选日期范围重新计算

#### Scenario: 默认时间范围
- Given 老板访问统计页面
- When 未选择日期范围
- Then 默认显示本月的统计数据

#### Scenario: 统计卡片样式
- Given 统计数据加载完成
- When 渲染员工统计列表
- Then 每个员工显示为一个卡片
- And 卡片包含头像、姓名、数量、金额

## Cross-references

- statistics spec: 复用 Worker Production Statistics API
- frontend-api: Stats API 已定义
