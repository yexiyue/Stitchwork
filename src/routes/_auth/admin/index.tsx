import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, NavBar } from "antd-mobile";
import { ChevronLeft, Key, Shield } from "lucide-react";

export const Route = createFileRoute("/_auth/admin/")({
  component: AdminIndexPage,
});

function AdminIndexPage() {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <NavBar
        onBack={() => navigate({ to: "/profile" })}
        backIcon={<ChevronLeft size={24} />}
      >
        管理后台
      </NavBar>

      <div className="flex-1 overflow-auto">
        <List>
          <List.Item
            prefix={<Key size={20} />}
            onClick={() => navigate({ to: "/admin/register-codes" })}
            arrow
          >
            注册码管理
          </List.Item>
          <List.Item
            prefix={<Shield size={20} />}
            onClick={() => navigate({ to: "/admin/users" })}
            arrow
          >
            用户管理
          </List.Item>
        </List>
      </div>
    </div>
  );
}
