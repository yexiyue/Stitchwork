# Tasks: add-stats-charts

## Setup

- [x] 1. 安装 echarts 和 echarts-for-react
  - `pnpm add echarts echarts-for-react`

## Backend

- [x] 2. 添加时间序列统计 API
  - `GET /api/stats/daily` 返回按日统计数据
  - 参数: startDate, endDate
  - 返回: 每日数量和金额

- [x] 3. 添加分组统计 API
  - `GET /api/stats/by-order` 按订单分组
  - `GET /api/stats/by-process` 按工序分组
  - 支持员工权限过滤

- [x] 4. cargo check 验证后端编译

## Boss Frontend

- [x] 5. 创建 ECharts 通用组件
  - 按需引入减小包体积
  - 支持响应式容器

- [x] 6. 添加维度切换功能
  - 数量/金额 Tab 切换

- [x] 7. 添加柱状图
  - X轴: 员工名称
  - Y轴: 数量或金额

- [x] 8. 添加饼图
  - 展示各员工占比

- [x] 9. 添加趋势折线图
  - X轴: 日期
  - Y轴: 数量或金额
  - 调用新的时间序列 API

- [x] 10. 调整老板端页面布局
  - Tab 切换图表类型
  - 移动端自适应

## Staff Frontend

- [x] 11. 添加员工统计页面入口
  - 从 my-records 主页可进入

- [x] 12. 添加员工端趋势折线图
  - X轴: 日期
  - Y轴: 数量或金额
  - 时间范围: 本周/本月切换

- [x] 13. 添加员工端堆叠柱状图
  - 按订单/工序分组展示产量
  - X轴: 订单或工序
  - Y轴: 数量

- [x] 14. 调整员工端页面布局
  - Tab 切换图表类型
  - 移动端自适应

## Validation

- [x] 15. pnpm build 验证编译
- [ ] 16. 手动测试图表交互
