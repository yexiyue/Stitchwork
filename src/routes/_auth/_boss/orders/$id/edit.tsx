import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Form,
  Input,
  Button,
  NavBar,
  Dialog,
  Toast,
  Picker,
  TextArea,
  Stepper,
} from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useOrder, useUpdateOrder, useCustomers } from "@/hooks";
import type { OrderStatus } from "@/types";
import { ImageUploader } from "@/components";
import { useState, useEffect } from "react";

export const Route = createFileRoute("/_auth/_boss/orders/$id/edit")({
  component: EditOrderPage,
});

const STATUS_OPTIONS = [
  { label: "待处理", value: "pending" },
  { label: "进行中", value: "processing" },
  { label: "已完成", value: "completed" },
  { label: "已交付", value: "delivered" },
  { label: "已取消", value: "cancelled" },
];

function EditOrderPage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const { data: order, isLoading } = useOrder(id);
  const updateMutation = useUpdateOrder();
  const { data: customersData } = useCustomers({ pageSize: 100 });

  const [images, setImages] = useState<string[]>([]);
  const [customerId, setCustomerId] = useState("");
  const [status, setStatus] = useState<OrderStatus>("pending");

  const customers = customersData?.list ?? [];
  const selectedCustomer = customers.find((c) => c.id === customerId);
  const selectedStatus = STATUS_OPTIONS.find((s) => s.value === status);

  useEffect(() => {
    if (order) {
      setImages(order.images ?? []);
      setCustomerId(order.customerId);
      setStatus(order.status);
    }
  }, [order]);

  const handleSelectCustomer = async () => {
    const options = customers.map((c) => ({ label: c.name, value: c.id }));
    const val = await Picker.prompt({ columns: [options] });
    if (val) setCustomerId(val[0] as string);
  };

  const handleSelectStatus = async () => {
    const val = await Picker.prompt({ columns: [STATUS_OPTIONS] });
    if (val) setStatus(val[0] as OrderStatus);
  };

  const handleSubmit = async (values: Record<string, unknown>) => {
    try {
      await updateMutation.mutateAsync({
        id,
        data: {
          productName: values.productName as string,
          description: values.description as string,
          quantity: values.quantity as number,
          unitPrice: String(values.unitPrice),
          images: images.length ? images : undefined,
          status,
        },
      });
      Toast.show({ content: "更新成功" });
      navigate({ to: "/orders/$id", params: { id } });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "更新失败",
        confirmText: "确定",
      });
    }
  };

  if (isLoading) {
    return <div className="p-4">加载中...</div>;
  }

  return (
    <div>
      <NavBar onBack={() => navigate({ to: "/orders/$id", params: { id } })} backIcon={<ChevronLeft size={24} />}>
        编辑订单
      </NavBar>
      <div className="p-4">
        <Form
          initialValues={order}
          onFinish={handleSubmit}
          footer={
            <Button
              block
              type="submit"
              color="primary"
              loading={updateMutation.isPending}
            >
              保存
            </Button>
          }
        >
          <Form.Item
            label="客户"
            clickable
            onClick={handleSelectCustomer}
            extra={selectedCustomer?.name || "请选择客户"}
          />
          <Form.Item
            label="状态"
            clickable
            onClick={handleSelectStatus}
            extra={selectedStatus?.label || "请选择状态"}
          />
          <Form.Item
            name="productName"
            label="产品名称"
            rules={[{ required: true, message: "请输入产品名称" }]}
          >
            <Input placeholder="请输入产品名称" clearable />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <TextArea placeholder="请输入描述" rows={3} />
          </Form.Item>
          <Form.Item name="quantity" label="数量" rules={[{ required: true }]}>
            <Stepper min={1} digits={0} />
          </Form.Item>
          <Form.Item name="unitPrice" label="单价" rules={[{ required: true }]}>
            <Stepper min={0} step={0.1} digits={1} />
          </Form.Item>
          <Form.Item label="产品图片">
            <ImageUploader value={images} onChange={setImages} maxCount={9} />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}
