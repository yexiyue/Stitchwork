# stats-charts-ui Specification

## Purpose

为统计页面添加 ECharts 图表，直观展示员工产量数据。

## ADDED Requirements

### Requirement: Stats Page Charts

统计页面 SHALL 显示员工产量图表。

#### Scenario: 维度切换
- Given 老板在统计页面
- When 点击"数量"或"金额"Tab
- Then 图表数据切换到对应维度

#### Scenario: 柱状图展示
- Given 统计数据加载完成
- When 渲染柱状图
- Then X轴显示员工名称
- And Y轴显示数量或金额
- And 支持触摸滑动查看更多

#### Scenario: 饼图展示
- Given 统计数据加载完成
- When 渲染饼图
- Then 显示各员工占比
- And 显示百分比标签
- And 点击扇区高亮显示

#### Scenario: 趋势折线图展示
- Given 统计数据加载完成
- When 渲染趋势图
- Then X轴显示日期
- And Y轴显示数量或金额
- And 支持触摸滑动查看更多日期

#### Scenario: 图表类型切换
- Given 老板在统计页面
- When 点击图表类型 Tab（柱状图/饼图/趋势图）
- Then 切换到对应的图表视图

#### Scenario: 空数据处理
- Given 选定时间段无数据
- When 渲染图表区域
- Then 显示"暂无数据"提示

#### Scenario: 响应式布局
- Given 图表渲染完成
- When 屏幕尺寸变化
- Then 图表自动调整大小适应容器

## Cross-references

- boss-records-statistics-ui: 复用现有统计页面结构
- daily-stats-api: 趋势图使用按日统计 API
