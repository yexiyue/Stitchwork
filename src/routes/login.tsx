import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Form, Input, Button, Card, Dialog, Toast } from "antd-mobile";
import { ScanLine } from "lucide-react";
import { useAuthStore } from "@/stores/auth";
import { useState, useEffect } from "react";

export const Route = createFileRoute("/login")({
  component: LoginPage,
});

function LoginPage() {
  const navigate = useNavigate();
  const login = useAuthStore((s) => s.login);
  const isAuthenticated = useAuthStore((s) => !!s.token);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (isAuthenticated) {
      navigate({ to: "/" });
    }
  }, [isAuthenticated, navigate]);

  const onFinish = async (values: { username: string; password: string }) => {
    setLoading(true);
    try {
      await login(values.username, values.password);
      navigate({ to: "/" });
    } catch (e) {
      Dialog.alert({
        content:
          e instanceof Error ? e.message : "登录失败，请检查用户名和密码",
        confirmText: "确定",
      });
    } finally {
      setLoading(false);
    }
  };

  // 扫描邀请码二维码
  const handleScanQR = async () => {
    try {
      const { scan, Format } = await import(
        "@tauri-apps/plugin-barcode-scanner"
      );
      const result = await scan({ formats: [Format.QRCode] });
      if (result?.content) {
        // 解析 stitchwork://register-staff?code=xxx
        const url = new URL(result.content);
        const path = url.pathname.replace(/^\//, "");
        if (path === "register-staff") {
          const code = url.searchParams.get("code");
          if (code) {
            navigate({ to: "/register-staff", search: { code } });
            return;
          }
        }
        Toast.show({ content: "无效的邀请码", icon: "fail" });
      }
    } catch (e) {
      // 非移动端或用户取消
      if (e instanceof Error && !e.message.includes("cancel")) {
        Toast.show({ content: "扫码功能仅支持移动端", icon: "fail" });
      }
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 px-4">
      <Card className="w-full h-1/2 max-w-sm">
        <h1 className="text-center text-2xl font-bold mb-6">登录</h1>
        <Form
          onFinish={onFinish}
          footer={
            <Button
              block
              type="submit"
              color="primary"
              size="large"
              loading={loading}
            >
              登录
            </Button>
          }
        >
          <Form.Item
            name="username"
            label="用户名"
            rules={[{ required: true, message: "请输入用户名" }]}
          >
            <Input placeholder="请输入用户名" clearable />
          </Form.Item>
          <Form.Item
            name="password"
            label="密码"
            rules={[{ required: true, message: "请输入密码" }]}
          >
            <Input type="password" placeholder="请输入密码" clearable />
          </Form.Item>
        </Form>
        <div className="text-center mt-4">
          <span className="text-gray-500">没有账号？</span>
          <a
            className="text-blue-500 ml-1"
            onClick={() => navigate({ to: "/register" })}
          >
            立即注册
          </a>
        </div>
        <div className="mt-6 pt-4 border-t border-gray-200">
          <Button
            block
            fill="none"
            onClick={handleScanQR}
            className="text-gray-600"
          >
            <div className="flex items-center justify-center gap-2">
              <ScanLine size={20} />
              <span>扫描邀请码加入工坊</span>
            </div>
          </Button>
        </div>
      </Card>
    </div>
  );
}
