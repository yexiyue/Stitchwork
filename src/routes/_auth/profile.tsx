import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, Button } from "antd-mobile";
import { Users, UserPlus, Settings, LogOut } from "lucide-react";
import { useAuthStore, selectIsBoss } from "@/stores/auth";

export const Route = createFileRoute("/_auth/profile")({
  component: ProfilePage,
});

function ProfilePage() {
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);
  const logout = useAuthStore((s) => s.logout);
  const isBoss = useAuthStore(selectIsBoss);

  const handleLogout = () => {
    logout();
    navigate({ to: "/login" });
  };

  return (
    <div className="p-4">
      <div className="mb-4">
        <h1 className="text-xl font-bold">{user?.displayName || user?.username}</h1>
        <p className="text-gray-500">{isBoss ? "老板" : "员工"}</p>
      </div>

      {isBoss && (
        <List header="管理">
          <List.Item prefix={<Users size={20} />} onClick={() => navigate({ to: "/customers" })}>
            客户管理
          </List.Item>
          <List.Item prefix={<UserPlus size={20} />} onClick={() => {}}>
            员工管理
          </List.Item>
        </List>
      )}

      {!isBoss && (
        <List header="我的">
          <List.Item onClick={() => {}}>记件记录</List.Item>
        </List>
      )}

      <List header="设置" className="mt-4">
        <List.Item prefix={<Settings size={20} />} onClick={() => {}}>
          设置
        </List.Item>
      </List>

      <div className="mt-6">
        <Button block color="default" onClick={handleLogout}>
          <span className="flex items-center justify-center gap-2">
            <LogOut size={18} />
            退出登录
          </span>
        </Button>
      </div>
    </div>
  );
}
