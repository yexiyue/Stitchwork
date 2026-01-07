import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Form, Input, Button, Card, Dialog, Toast } from "antd-mobile";
import { authApi } from "@/api";
import { useState } from "react";

export const Route = createFileRoute("/register")({
  component: RegisterPage,
});

interface RegisterFormValues {
  username: string;
  password: string;
  confirmPassword: string;
  phone: string;
  registerCode: string;
}

const phonePattern = /^1\d{10}$/;

function RegisterPage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

  const onFinish = async (values: RegisterFormValues) => {
    if (values.password !== values.confirmPassword) {
      Dialog.alert({ content: "两次密码不一致", confirmText: "确定" });
      return;
    }
    if (!phonePattern.test(values.phone)) {
      Dialog.alert({ content: "手机号格式不正确", confirmText: "确定" });
      return;
    }
    setLoading(true);
    try {
      await authApi.register({
        username: values.username,
        password: values.password,
        phone: values.phone,
        registerCode: values.registerCode,
      });
      Toast.show({ content: "注册成功", icon: "success" });
      navigate({ to: "/login" });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "注册失败，请稍后重试",
        confirmText: "确定",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 px-4">
      <Card className="w-full max-w-sm">
        <h1 className="text-center text-2xl font-bold mb-6">注册</h1>
        <Form
          onFinish={onFinish}
          footer={
            <Button block type="submit" color="primary" size="large" loading={loading}>
              注册
            </Button>
          }
        >
          <Form.Item
            name="registerCode"
            label="注册码"
            rules={[{ required: true, message: "请输入注册码" }]}
          >
            <Input placeholder="请输入注册码" clearable />
          </Form.Item>
          <Form.Item
            name="username"
            label="用户名"
            rules={[{ required: true, message: "请输入用户名" }]}
          >
            <Input placeholder="请输入用户名" clearable />
          </Form.Item>
          <Form.Item
            name="phone"
            label="手机号"
            rules={[{ required: true, message: "请输入手机号" }]}
          >
            <Input placeholder="请输入手机号" type="tel" maxLength={11} clearable />
          </Form.Item>
          <Form.Item
            name="password"
            label="密码"
            rules={[{ required: true, message: "请输入密码" }]}
          >
            <Input type="password" placeholder="请输入密码" clearable />
          </Form.Item>
          <Form.Item
            name="confirmPassword"
            label="确认密码"
            rules={[{ required: true, message: "请再次输入密码" }]}
          >
            <Input type="password" placeholder="请再次输入密码" clearable />
          </Form.Item>
        </Form>
        <div className="text-center mt-4">
          <span className="text-gray-500">已有账号？</span>
          <a className="text-blue-500 ml-1" onClick={() => navigate({ to: "/login" })}>
            立即登录
          </a>
        </div>
      </Card>
    </div>
  );
}
