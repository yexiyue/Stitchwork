import { createFileRoute } from "@tanstack/react-router";
import { Button } from "antd-mobile";
import { Home } from "lucide-react";
import { useAuthStore } from "@/stores/auth";

export const Route = createFileRoute("/_auth/")({
  component: IndexPage,
});

function IndexPage() {
  const user = useAuthStore((s) => s.user);
  const logout = useAuthStore((s) => s.logout);

  return (
    <div className="p-4">
      <h1 className="flex items-center gap-2 text-xl font-bold">
        <Home size={24} />
        StitchWork
      </h1>
      <p className="mt-2">欢迎, {user?.displayName || user?.username}</p>
      <p className="text-gray-500">服装加工流程管理系统</p>
      <div className="mt-4 flex gap-2">
        <Button color="primary">开始使用</Button>
        <Button onClick={logout}>退出登录</Button>
      </div>
    </div>
  );
}
