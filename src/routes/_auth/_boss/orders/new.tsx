import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Form,
  Input,
  Button,
  NavBar,
  Dialog,
  Toast,
  Picker,
  Stepper,
  TextArea,
} from "antd-mobile";
import { useCreateOrder, useCustomers } from "@/hooks";
import { ImageUploader } from "@/components";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_boss/orders/new")({
  component: NewOrderPage,
});

function NewOrderPage() {
  const navigate = useNavigate();
  const createMutation = useCreateOrder();
  const { data: customersData } = useCustomers({ pageSize: 100 });
  const [images, setImages] = useState<string[]>([]);
  const [customerId, setCustomerId] = useState("");
  const customers = customersData?.list ?? [];
  const selectedCustomer = customers.find((c) => c.id === customerId);

  const handleSelectCustomer = async () => {
    const options = customers.map((c) => ({ label: c.name, value: c.id }));
    const val = await Picker.prompt({ columns: [options] });
    if (val) setCustomerId(val[0] as string);
  };

  const handleSubmit = async (values: Record<string, unknown>) => {
    if (!customerId) {
      Toast.show({ content: "请选择客户" });
      return;
    }
    try {
      await createMutation.mutateAsync({
        productName: values.productName as string,
        description: values.description as string,
        quantity: values.quantity as number,
        unitPrice: String(values.unitPrice),
        customerId,
        images: images.length ? images : undefined,
      });
      Toast.show({ content: "创建成功" });
      navigate({ to: "/orders" });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "创建失败",
        confirmText: "确定",
      });
    }
  };

  return (
    <div>
      <NavBar onBack={() => navigate({ to: "/orders" })}>新增订单</NavBar>
      <div className="p-4">
        <Form
          onFinish={handleSubmit}
          footer={
            <Button
              block
              type="submit"
              color="primary"
              loading={createMutation.isPending}
            >
              保存
            </Button>
          }
        >
          <Form.Item
            label="客户"
            required
            clickable
            onClick={handleSelectCustomer}
            extra={selectedCustomer?.name || "请选择客户"}
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
