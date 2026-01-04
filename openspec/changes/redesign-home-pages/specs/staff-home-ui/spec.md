# staff-home-ui Specification

## Purpose

员工端首页展示工坊信息、个人统计、趋势图和最近记录。

## ADDED Requirements

### Requirement: Staff Home Overview

员工端首页 SHALL 显示工坊信息和个人统计。

#### Scenario: 工坊信息展示

- Given 员工用户访问首页
- When 页面加载完成
- Then 显示工坊名称和图片
- And 显示工坊地址（如有）

#### Scenario: 本月统计卡片

- Given 员工用户访问首页
- When 页面加载完成
- Then 显示本月完成数量
- And 显示本月预估金额
- And 点击可跳转到统计详情页

#### Scenario: 本周趋势图

- Given 员工用户访问首页
- When 页面加载完成
- Then 显示近7天个人产量迷你折线图

#### Scenario: 快捷操作

- Given 员工在首页
- When 查看快捷操作区域
- Then 显示「录入计件」按钮
- And 点击跳转到计件录入页

#### Scenario: 最近记录

- Given 员工用户访问首页
- When 页面加载完成
- Then 显示最近5条计件记录
- And 显示状态（待审核/已通过/已拒绝）

## Cross-references

- home-api: 使用首页概览 API
- add-stats-charts: 复用 ECharts 组件
