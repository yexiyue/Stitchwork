# boss-home-ui Specification

## Purpose

老板端首页展示工坊概览、数据统计、趋势图和最近动态。

## ADDED Requirements

### Requirement: Boss Home Overview

老板端首页 SHALL 显示工坊和数据概览。

#### Scenario: 工坊信息展示

- Given 老板用户访问首页
- When 页面加载完成
- Then 显示工坊名称、图片、地址
- And 显示员工数量

#### Scenario: 数据概览卡片

- Given 老板用户访问首页
- When 页面加载完成
- Then 显示待审核数量（实时徽章）
- And 显示进行中订单数量
- And 显示今日产量

#### Scenario: 待审核徽章点击

- Given 老板在首页看到待审核徽章
- When 点击待审核卡片
- Then 跳转到待审核列表页

#### Scenario: 本周趋势图

- Given 老板用户访问首页
- When 页面加载完成
- Then 显示近7天产量迷你折线图
- And 点击图表可跳转到详细统计页

#### Scenario: 快捷操作

- Given 老板在首页
- When 查看快捷操作区域
- Then 显示「新建订单」入口
- And 显示「客户管理」入口
- And 显示「快捷记件」入口

#### Scenario: 最近动态

- Given 老板用户访问首页
- When 页面加载完成
- Then 显示最近5条动态
- And 包含员工提交计件、审核通过/拒绝等事件

## Cross-references

- home-api: 使用首页概览 API
- add-stats-charts: 复用 ECharts 组件
