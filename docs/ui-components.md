# UI 组件使用规范

## 表单组件

### 新增/编辑页面表单

使用 antd-mobile 的 `Form` 组件，配合 `Form.Item`。

```tsx
<Form
  onFinish={handleSubmit}
  footer={
    <Button block type="submit" color="primary" loading={isPending}>
      保存
    </Button>
  }
>
  <Form.Item name="fieldName" label="字段名" rules={[{ required: true }]}>
    <Input placeholder="请输入" clearable />
  </Form.Item>
</Form>
```

### 选择器（Picker）

用于从列表中选择单个值，使用 `Picker.prompt` + `Form.Item clickable`：

```tsx
const [customerId, setCustomerId] = useState("");
const selectedCustomer = customers.find((c) => c.id === customerId);

const handleSelectCustomer = async () => {
  const options = customers.map((c) => ({ label: c.name, value: c.id }));
  const val = await Picker.prompt({ columns: [options] });
  if (val) setCustomerId(val[0] as string);
};

<Form.Item
  label="客户"
  required
  clickable
  onClick={handleSelectCustomer}
  extra={selectedCustomer?.name || "请选择客户"}
/>
```

### 级联选择

当选择项依赖另一个选择时（如工序依赖订单）：

```tsx
const [orderId, setOrderId] = useState("");
const [processId, setProcessId] = useState("");

// 工序数据依赖订单
const { data: processesData } = useQuery({
  queryKey: ["processes", orderId],
  queryFn: () => processApi.list({ orderId }),
  enabled: !!orderId,
});

const handleSelectOrder = async () => {
  const val = await Picker.prompt({ columns: [orderOptions] });
  if (val) {
    setOrderId(val[0] as string);
    setProcessId(""); // 清空依赖项
  }
};

const handleSelectProcess = async () => {
  if (!orderId) {
    Toast.show({ content: "请先选择订单" });
    return;
  }
  const val = await Picker.prompt({ columns: [processOptions] });
  if (val) setProcessId(val[0] as string);
};
```

### 数量输入（Stepper）

```tsx
<Form.Item label="数量" required>
  <Stepper min={1} value={quantity} onChange={setQuantity} />
</Form.Item>

// 带小数
<Form.Item label="单价" required>
  <Stepper min={0} step={0.1} digits={1} />
</Form.Item>
```

### 多行文本

```tsx
<Form.Item name="description" label="描述">
  <TextArea placeholder="请输入描述" rows={3} />
</Form.Item>
```

## 筛选组件

### 下拉筛选（Dropdown）

用于列表页的多条件筛选：

```tsx
const dropdownRef = useRef<DropdownRef>(null);
const [statusFilter, setStatusFilter] = useState("");

<Dropdown ref={dropdownRef}>
  <Dropdown.Item key="filter" title={<Filter size={20} />}>
    <div className="p-2">
      <div className="text-gray-500 text-sm mb-2">状态</div>
      {STATUS_OPTIONS.map((opt) => (
        <div
          key={opt.key}
          className={`p-3 rounded ${statusFilter === opt.key ? "bg-blue-50 text-blue-500" : ""}`}
          onClick={() => setStatusFilter(opt.key)}
        >
          {opt.title}
        </div>
      ))}
      <Button block color="primary" className="mt-3" onClick={() => dropdownRef.current?.close()}>
        确定
      </Button>
    </div>
  </Dropdown.Item>
</Dropdown>
```

### 搜索框（SearchBar）

用于简单的文本搜索：

```tsx
const [search, setSearch] = useState("");

<SearchBar placeholder="搜索" value={search} onChange={setSearch} />
```

## 列表组件

### 虚拟列表（VirtualList）

用于大数据量的无限滚动列表：

```tsx
<VirtualList
  data={list}
  loading={isFetching}
  hasMore={!!hasNextPage}
  onLoadMore={fetchNextPage}
  onRefresh={refetch}
  keyExtractor={(item) => item.id}
  emptyText="暂无数据"
  searchEmpty={!!hasFilters && !list.length}
  estimateSize={100}
  renderItem={(item) => <ItemCard item={item} />}
/>
```

### 滑动操作（SwipeAction）

用于列表项的快捷操作：

```tsx
<SwipeAction
  rightActions={[
    { key: "edit", text: "编辑", color: "primary", onClick: () => {} },
    { key: "delete", text: "删除", color: "danger", onClick: () => {} },
  ]}
>
  <div>列表项内容</div>
</SwipeAction>
```

## 弹窗组件

### 确认对话框

```tsx
Dialog.confirm({
  content: "确定删除？",
  confirmText: "删除",
  cancelText: "取消",
  onConfirm: () => deleteMutation.mutate(id),
});
```

### 提示信息

```tsx
Toast.show({ content: "操作成功" });
```

## 页面布局

### 标准列表页

```tsx
<div className="flex flex-col h-full overflow-hidden">
  {/* 头部：标题 + 操作按钮 */}
  <div className="p-4 pb-2">
    <div className="flex items-center justify-between mb-3">
      <h1 className="text-xl">页面标题</h1>
      <div className="flex items-center gap-2">
        <div className="flex items-center text-blue-500" onClick={handleAdd}>
          <Plus size={20} />
          <span>新增</span>
        </div>
        {/* 筛选按钮 */}
      </div>
    </div>
  </div>
  {/* 列表区域 */}
  <div className="flex flex-1 overflow-hidden">
    <VirtualList ... />
  </div>
</div>
```

### 标准新增/编辑页

```tsx
<div>
  <NavBar onBack={() => navigate({ to: "/list" })}>新增XXX</NavBar>
  <div className="p-4">
    <Form footer={<Button>保存</Button>}>
      {/* 表单项 */}
    </Form>
  </div>
</div>
```

## 状态标签（Tag）

```tsx
const STATUS_MAP = {
  pending: { label: "待审核", color: "#faad14" },
  approved: { label: "已通过", color: "#52c41a" },
  rejected: { label: "已拒绝", color: "#ff4d4f" },
};

<Tag color={STATUS_MAP[status].color} fill="outline" style={{ "--border-radius": "4px" }}>
  {STATUS_MAP[status].label}
</Tag>
```
