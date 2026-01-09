import { createRootRoute, Outlet } from "@tanstack/react-router";
import { ConfigProvider, SafeArea, SpinLoading } from "antd-mobile";
import zhCN from "antd-mobile/es/locales/zh-CN";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@/lib/query-client";
import { storageReady } from "@/stores/auth";
import { useEffect, useState } from "react";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    storageReady.finally(() => setReady(true));
  }, []);

  if (!ready) {
    return (
      <div className="h-screen w-screen flex items-center justify-center bg-white">
        <SpinLoading color="primary" style={{ "--size": "48px" }} />
      </div>
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <ConfigProvider locale={zhCN}>
        <div className="h-full w-full flex flex-col overflow-hidden">
          <SafeArea position="top" />
          <div className="flex-1 h-full overflow-hidden">
            <Outlet />
          </div>
        </div>
        <SafeArea position="bottom" />
      </ConfigProvider>
    </QueryClientProvider>
  );
}
