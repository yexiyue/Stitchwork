import { createFileRoute, Outlet, redirect, useNavigate, useLocation } from "@tanstack/react-router";
import { TabBar } from "antd-mobile";
import { Home, ClipboardList, FileEdit, User } from "lucide-react";
import { useAuthStore, selectIsBoss } from "@/stores/auth";

export const Route = createFileRoute("/_auth")({
  beforeLoad: () => {
    const token = useAuthStore.getState().token;
    if (!token) {
      throw redirect({ to: "/login" });
    }
  },
  component: AuthLayout,
});

const bossTabs = [
  { key: "/", title: "首页", icon: <Home size={24} /> },
  { key: "/orders", title: "订单", icon: <ClipboardList size={24} /> },
  { key: "/records", title: "计件", icon: <FileEdit size={24} /> },
  { key: "/profile", title: "我的", icon: <User size={24} /> },
];

const staffTabs = [
  { key: "/", title: "首页", icon: <Home size={24} /> },
  { key: "/record", title: "计件", icon: <FileEdit size={24} /> },
  { key: "/profile", title: "我的", icon: <User size={24} /> },
];

function AuthLayout() {
  const navigate = useNavigate();
  const location = useLocation();
  const isBoss = useAuthStore(selectIsBoss);
  const tabs = isBoss ? bossTabs : staffTabs;

  // 子页面映射到对应的 tab
  const subPageMap: Record<string, string> = {
    "/customers": "/profile",
    "/staff": "/profile",
  };
  const mappedPath = Object.entries(subPageMap).find(([prefix]) =>
    location.pathname.startsWith(prefix)
  )?.[1];

  const activeKey = tabs.find((t) =>
    t.key === location.pathname ||
    t.key === mappedPath ||
    (t.key !== "/" && location.pathname.startsWith(t.key))
  )?.key || "/";

  return (
    <div className="flex h-full flex-col">
      <div className="flex-1 overflow-auto">
        <Outlet />
      </div>
      <TabBar activeKey={activeKey} onChange={(key) => navigate({ to: key })}>
        {tabs.map((tab) => (
          <TabBar.Item key={tab.key} icon={tab.icon} title={tab.title} />
        ))}
      </TabBar>
    </div>
  );
}
