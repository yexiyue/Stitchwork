import { createRootRoute, Outlet } from "@tanstack/react-router";
import { ConfigProvider } from "antd-mobile";
import zhCN from "antd-mobile/es/locales/zh-CN";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@/lib/query-client";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  return (
    <QueryClientProvider client={queryClient}>
      <ConfigProvider locale={zhCN}>
        <div className="h-full w-full">
          <Outlet />
        </div>
      </ConfigProvider>
    </QueryClientProvider>
  );
}
