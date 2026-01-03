import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Form, Input, Button, Toast, TextArea } from "antd-mobile";
import { useAuthStore } from "@/stores/auth";
import { authApi } from "@/api";
import { PageHeader, ImageUploader } from "@/components";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_boss/workshop")({
  component: WorkshopPage,
});

function WorkshopPage() {
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);
  const updateUser = useAuthStore((s) => s.updateUser);
  const [saving, setSaving] = useState(false);
  const workshop = user?.workshop;

  const handleSubmit = async (values: {
    name: string;
    desc?: string;
    address?: string;
    image?: string;
  }) => {
    setSaving(true);
    try {
      const ws = workshop
        ? await authApi.updateWorkshop(values)
        : await authApi.createWorkshop(values);
      updateUser({ workshop: ws });
      Toast.show({ content: "保存成功" });
    } catch (e) {
      Toast.show({ content: e instanceof Error ? e.message : "保存失败" });
    } finally {
      setSaving(false);
    }
  };

  return (
    <div>
      <PageHeader title="工坊设置" onBack={() => navigate({ to: "/" })} />
      <div className="p-4">
        <Form
          initialValues={{
            name: workshop?.name || "",
            desc: workshop?.desc || "",
            address: workshop?.address || "",
            image: workshop?.image || "",
          }}
          onFinish={handleSubmit}
          footer={
            <Button block type="submit" color="primary" loading={saving}>
              保存
            </Button>
          }
        >
          <Form.Item name="name" label="工坊名称" rules={[{ required: true, message: "请输入工坊名称" }]}>
            <Input placeholder="请输入工坊名称" clearable />
          </Form.Item>
          <Form.Item name="address" label="工坊地址">
            <Input placeholder="请输入工坊地址" clearable />
          </Form.Item>
          <Form.Item name="desc" label="工坊简介">
            <TextArea placeholder="请输入工坊简介" rows={3} />
          </Form.Item>
          <Form.Item name="image" label="工坊图片">
            <ImageUploader maxCount={1} />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}
