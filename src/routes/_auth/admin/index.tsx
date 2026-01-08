import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List } from "antd-mobile";
import { Key, Shield } from "lucide-react";

export const Route = createFileRoute("/_auth/admin/")({
  component: AdminIndexPage,
});

function AdminIndexPage() {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="p-4 pb-2">
        <h1 className="text-xl">管理后台</h1>
      </div>

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
