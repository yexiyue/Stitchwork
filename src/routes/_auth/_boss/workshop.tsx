import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Form, Input, Button, Toast, TextArea } from "antd-mobile";
import { useAuthStore } from "@/stores/auth";
import { authApi } from "@/api";
import { PageHeader, ImageUploader } from "@/components";
import { useWorkshopSettings } from "@/hooks";
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
  const { businessLabel } = useWorkshopSettings();

  const handleSubmit = async (values: {
    name: string;
    desc?: string;
    address?: string;
    image?: string;
    pieceUnit?: string;
    businessLabel?: string;
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
      <PageHeader title={`${businessLabel}设置`} onBack={() => navigate({ to: "/" })} />
      <div className="p-4">
        <Form
          initialValues={{
            name: workshop?.name || "",
            desc: workshop?.desc || "",
            address: workshop?.address || "",
            image: workshop?.image || "",
            pieceUnit: workshop?.pieceUnit || "打",
            businessLabel: workshop?.businessLabel || "工坊",
          }}
          onFinish={handleSubmit}
          footer={
            <Button block type="submit" color="primary" loading={saving}>
              保存
            </Button>
          }
        >
          <Form.Item name="name" label={`${businessLabel}名称`} rules={[{ required: true, message: `请输入${businessLabel}名称` }]}>
            <Input placeholder={`请输入${businessLabel}名称`} clearable />
          </Form.Item>
          <Form.Item name="address" label={`${businessLabel}地址`}>
            <Input placeholder={`请输入${businessLabel}地址`} clearable />
          </Form.Item>
          <Form.Item name="desc" label={`${businessLabel}简介`}>
            <TextArea placeholder={`请输入${businessLabel}简介`} rows={3} />
          </Form.Item>
          <Form.Item name="image" label={`${businessLabel}图片`}>
            <ImageUploader maxCount={1} />
          </Form.Item>
          <Form.Item name="pieceUnit" label="计件单位">
            <Input placeholder="如：打、件、双" clearable />
          </Form.Item>
          <Form.Item name="businessLabel" label="场所名称">
            <Input placeholder="如：工坊、工厂、店铺" clearable />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}
