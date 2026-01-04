# staff-charts-ui Specification

## Purpose

为员工端添加个人产量图表，支持查看时间趋势和订单/工序分布。

## ADDED Requirements

### Requirement: Staff Stats Page

员工端 SHALL 提供个人产量图表页面。

#### Scenario: 页面入口

- Given 员工在 my-records 主页
- When 点击统计卡片或入口
- Then 导航到统计图表页面

#### Scenario: 时间范围切换

- Given 员工在统计页面
- When 点击"本周"或"本月"Tab
- Then 图表数据切换到对应时间范围

#### Scenario: 趋势折线图展示

- Given 统计数据加载完成
- When 渲染趋势图
- Then X轴显示日期
- And Y轴显示数量或金额
- And 支持触摸查看具体数值

#### Scenario: 堆叠柱状图展示

- Given 统计数据加载完成
- When 渲染堆叠柱状图
- Then X轴显示订单或工序名称
- And Y轴显示数量
- And 不同颜色区分不同分组维度

#### Scenario: 图表类型切换

- Given 员工在统计页面
- When 点击图表类型 Tab（趋势图/柱状图）
- Then 切换到对应的图表视图

#### Scenario: 空数据处理

- Given 选定时间段无数据
- When 渲染图表区域
- Then 显示"暂无数据"提示

## Cross-references

- stats-charts-ui: 复用 ECharts 通用组件
- daily-stats-api: 趋势图使用按日统计 API（带用户过滤）
