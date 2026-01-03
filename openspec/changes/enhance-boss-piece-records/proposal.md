# Change: 完善老板端计件管理功能

## Why

1. 当前计件列表只显示工序ID，无法直观看到工序名称、员工姓名等关键信息
2. 缺少按员工、订单筛选功能，难以快速定位特定记录
3. 老板无法帮员工创建计件记录，需要员工自己操作

## What Changes

### 前端 - 计件列表增强
- 显示工序名称（关联 Process）、员工姓名（关联 User）
- 添加按员工筛选功能
- 添加按订单筛选功能（通过工序关联订单）
- 卡片显示更多信息：员工名、工序名、订单名、数量、金额、状态、时间

### 前端 - 新增计件功能
- 添加"新增"按钮
- 弹窗表单：选择员工、选择订单、选择工序、输入数量
- 工序选择依赖订单选择（级联）
- 老板创建的记录 recordedBy = "boss"，自动 approved

### 后端 - API 增强
- 计件列表 API 返回关联的工序名称、员工姓名、订单名称
- 添加员工列表 API（老板获取自己的员工）
- 支持按 userId、orderId 筛选计件记录

## Impact

- Affected specs: frontend-api, data-access
- Affected code:
  - `src/routes/_auth/_boss/records/index.tsx` - 增强列表和筛选
  - `src/api/piece-record.ts` - 可能需要调整
  - `src/api/auth.ts` - 添加获取员工列表
  - `src/hooks/` - 添加相关 hooks
  - `src/types/` - 可能需要扩展类型
