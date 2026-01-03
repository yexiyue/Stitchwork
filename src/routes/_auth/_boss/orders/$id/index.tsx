import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  NavBar,
  Button,
  Tag,
  Image,
  ImageViewer,
  List,
  SwipeAction,
  Dialog,
  Toast,
  Form,
  Input,
  Stepper,
  TextArea,
  Popup,
  Divider,
} from "antd-mobile";
import { Edit, Plus, ImageOff } from "lucide-react";
import {
  useOrder,
  useProcesses,
  useCreateProcess,
  useUpdateProcess,
  useDeleteProcess,
} from "@/hooks";
import type {
  OrderStatus,
  Process,
  CreateProcessDto,
  UpdateProcessDto,
} from "@/types";
import { RelativeTime } from "@/components";
import { useQueryClient } from "@tanstack/react-query";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_boss/orders/$id/")({
  component: OrderDetailPage,
});

const STATUS_MAP: Record<OrderStatus, { label: string; color: string }> = {
  pending: { label: "待处理", color: "#faad14" },
  processing: { label: "进行中", color: "#1890ff" },
  completed: { label: "已完成", color: "#52c41a" },
  delivered: { label: "已交付", color: "#722ed1" },
  cancelled: { label: "已取消", color: "#999" },
};

function OrderDetailPage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const { data: order, isLoading } = useOrder(id);
  const { data: processesData } = useProcesses({ orderId: id });
  const createMutation = useCreateProcess();
  const updateMutation = useUpdateProcess();
  const deleteMutation = useDeleteProcess();
  const queryClient = useQueryClient();

  const [showForm, setShowForm] = useState(false);
  const [editingProcess, setEditingProcess] = useState<Process | null>(null);

  const processes = processesData?.list ?? [];

  // 统计信息
  const totalProcesses = processes.length;
  const totalAmount = processes.reduce(
    (sum, p) => sum + parseFloat(p.piecePrice),
    0
  );

  const handleDelete = (process: Process) => {
    Dialog.confirm({
      content: `确定删除工序「${process.name}」？`,
      confirmText: "删除",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await deleteMutation.mutateAsync(process.id);
          Toast.show({ content: "删除成功" });
          queryClient.invalidateQueries({ queryKey: ["processes"] });
        } catch (e) {
          Dialog.alert({
            content: e instanceof Error ? e.message : "删除失败",
            confirmText: "确定",
          });
        }
      },
    });
  };

  const handleSubmit = async (values: Record<string, unknown>) => {
    try {
      if (editingProcess) {
        await updateMutation.mutateAsync({
          id: editingProcess.id,
          data: {
            name: values.name as string,
            description: values.description as string,
            piecePrice: String(values.piecePrice),
          } as UpdateProcessDto,
        });
        Toast.show({ content: "更新成功" });
      } else {
        await createMutation.mutateAsync({
          orderId: id,
          name: values.name as string,
          description: values.description as string,
          piecePrice: String(values.piecePrice),
        } as CreateProcessDto);
        Toast.show({ content: "创建成功" });
      }
      setShowForm(false);
      setEditingProcess(null);
      queryClient.invalidateQueries({ queryKey: ["processes"] });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "操作失败",
        confirmText: "确定",
      });
    }
  };

  const openEditForm = (process: Process) => {
    setEditingProcess(process);
    setShowForm(true);
  };

  const openCreateForm = () => {
    setEditingProcess(null);
    setShowForm(true);
  };

  if (isLoading) {
    return <div className="p-4">加载中...</div>;
  }

  if (!order) {
    return <div className="p-4">订单不存在</div>;
  }

  const surplus = parseFloat(order.unitPrice) - totalAmount;

  return (
    <div className="flex flex-col h-full">
      <NavBar
        onBack={() => navigate({ to: "/orders" })}
        right={
          <Button
            size="small"
            fill="none"
            onClick={() => navigate({ to: "/orders/$id/edit", params: { id } })}
          >
            <Edit size={20} />
          </Button>
        }
      >
        订单详情
      </NavBar>

      <div className="flex-1 overflow-y-auto p-4">
        {/* 订单信息卡片 */}
        <div className="bg-white rounded-lg p-4 mb-4 shadow-sm">
          <div className="flex justify-between items-start mb-3">
            <h2 className="text-lg font-medium">{order.productName}</h2>
            <Tag
              color={STATUS_MAP[order.status].color}
              fill="outline"
              style={{ "--border-radius": "4px" }}
            >
              {STATUS_MAP[order.status].label}
            </Tag>
          </div>
          {order.description && (
            <p className="text-gray-500 text-sm mb-3">{order.description}</p>
          )}
          <div className="flex gap-4 text-sm text-gray-600 mb-3">
            <span>数量: {order.quantity}</span>
            <span>单价: ¥{order.unitPrice}</span>
          </div>
          {order.images?.length ? (
            <div className="flex gap-2 overflow-x-auto">
              {order.images.map((url, i) => (
                <Image
                  key={i}
                  src={url}
                  width={80}
                  height={80}
                  fit="cover"
                  className="rounded shrink-0"
                  onClick={() =>
                    ImageViewer.Multi.show({
                      images: order.images!,
                      defaultIndex: i,
                    })
                  }
                />
              ))}
            </div>
          ) : (
            <div className="w-20 h-20 bg-gray-100 rounded flex items-center justify-center">
              <ImageOff size={24} className="text-gray-400" />
            </div>
          )}
          <div className="text-xs text-gray-400 mt-3">
            创建于 <RelativeTime date={order.createdAt} />
          </div>
        </div>

        {/* 工序列表 */}
        <div className="flex items-center justify-between mb-2">
          <span className="font-medium">工序列表</span>
          <Button size="small" color="primary" onClick={openCreateForm}>
            <div className="flex items-center">
              <Plus size={16} className="mr-1" />
              新增
            </div>
          </Button>
        </div>
        {processes.length === 0 ? (
          <div className="bg-white rounded-lg p-8 text-center text-gray-400">
            暂无工序，点击新增添加
          </div>
        ) : (
          <List className="rounded-lg overflow-hidden">
            {processes.map((process) => (
              <SwipeAction
                key={process.id}
                rightActions={[
                  {
                    key: "edit",
                    text: "编辑",
                    color: "primary",
                    onClick: () => openEditForm(process),
                  },
                  {
                    key: "delete",
                    text: "删除",
                    color: "danger",
                    onClick: () => handleDelete(process),
                  },
                ]}
              >
                <List.Item
                  description={process.description || "无描述"}
                  extra={
                    <span className="text-blue-500">¥{process.piecePrice}</span>
                  }
                >
                  {process.name}
                </List.Item>
              </SwipeAction>
            ))}
          </List>
        )}
      </div>

      {/* 底部统计栏 */}
      <Divider style={{ margin: 0 }} />
      <div className="bg-white px-4 py-3 flex justify-between items-center">
        <div className="flex gap-4 text-gray-500">
          <span>共 {totalProcesses} 个</span>
          <span>合计 ¥{totalAmount.toFixed(2)}</span>
        </div>
        <span className={surplus >= 0 ? "text-green-500" : "text-red-500"}>
          盈余 ¥{surplus.toFixed(2)}
        </span>
      </div>

      {/* 新增/编辑工序弹窗 */}
      <Popup
        visible={showForm}
        onMaskClick={() => {
          setShowForm(false);
          setEditingProcess(null);
        }}
        bodyStyle={{ height: "60vh" }}
      >
        <div className="p-4">
          <h3 className="text-lg font-medium mb-4">
            {editingProcess ? "编辑工序" : "新增工序"}
          </h3>
          <Form
            initialValues={editingProcess || { piecePrice: 0 }}
            onFinish={handleSubmit}
            footer={
              <Button
                block
                type="submit"
                color="primary"
                loading={createMutation.isPending || updateMutation.isPending}
              >
                保存
              </Button>
            }
          >
            <Form.Item
              name="name"
              label="工序名称"
              rules={[{ required: true, message: "请输入工序名称" }]}
            >
              <Input placeholder="请输入工序名称" clearable />
            </Form.Item>
            <Form.Item name="description" label="描述">
              <TextArea placeholder="请输入描述" rows={2} />
            </Form.Item>
            <Form.Item
              name="piecePrice"
              label="计件单价"
              rules={[{ required: true, message: "请输入计件单价" }]}
            >
              <Stepper min={0} step={0.1} digits={2} />
            </Form.Item>
          </Form>
        </div>
      </Popup>
    </div>
  );
}
