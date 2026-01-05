import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Form,
  Input,
  Button,
  NavBar,
  Dialog,
  Toast,
  TextArea,
} from "antd-mobile";
import { useCreateCustomer } from "@/hooks";
import type { CreateCustomerDto } from "@/types";

export const Route = createFileRoute("/_auth/_boss/customers/new")({
  component: NewCustomerPage,
});

function NewCustomerPage() {
  const navigate = useNavigate();
  const createMutation = useCreateCustomer();

  const handleSubmit = async (values: CreateCustomerDto) => {
    try {
      await createMutation.mutateAsync(values);
      Toast.show({ content: "创建成功" });
      navigate({ to: "/customers" });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "创建失败",
        confirmText: "确定",
      });
    }
  };

  return (
    <div>
      <NavBar onBack={() => navigate({ to: "/customers" })}>新增客户</NavBar>
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
            name="name"
            label="名称"
            rules={[{ required: true, message: "请输入客户名称" }]}
          >
            <Input placeholder="请输入客户名称" clearable />
          </Form.Item>
          <Form.Item name="phone" label="电话">
            <Input placeholder="请输入电话" clearable />
          </Form.Item>
          <Form.Item name="description" label="备注">
            <TextArea placeholder="请输入备注" rows={3} />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}
