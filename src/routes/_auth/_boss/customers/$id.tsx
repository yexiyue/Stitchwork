import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Form, Input, Button, NavBar, Dialog, Toast } from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useCustomer, useUpdateCustomer } from "@/hooks";
import type { UpdateCustomerDto } from "@/types";

export const Route = createFileRoute("/_auth/_boss/customers/$id")({
  component: EditCustomerPage,
});

function EditCustomerPage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const { data: customer, isLoading } = useCustomer(id);
  const updateMutation = useUpdateCustomer();

  const handleSubmit = async (values: UpdateCustomerDto) => {
    try {
      await updateMutation.mutateAsync({ id, data: values });
      Toast.show({ content: "更新成功" });
      navigate({ to: "/customers" });
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
      <NavBar
        onBack={() => navigate({ to: "/customers" })}
        backIcon={<ChevronLeft size={24} />}
      >
        编辑客户
      </NavBar>
      <div className="p-4">
        <Form
          initialValues={customer}
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
            <Input placeholder="请输入备注" clearable />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}
