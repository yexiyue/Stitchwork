import React from "react";
import ReactDOM, { createRoot } from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { unstableSetRender } from "antd-mobile";
import { onOpenUrl, getCurrent } from "@tauri-apps/plugin-deep-link";
import { routeTree } from "./routeTree.gen";
import "./index.css";

// React 19 compatibility for antd-mobile
unstableSetRender((node, container) => {
  (container as any)._reactRoot ||= createRoot(container);
  const root = (container as any)._reactRoot;
  root.render(node);
  return async () => {
    await new Promise((resolve) => setTimeout(resolve, 0));
    root.unmount();
  };
});

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

// 处理 deep link URL
function handleDeepLink(urls: string[]) {
  for (const urlStr of urls) {
    try {
      const url = new URL(urlStr);
      // 处理 stitchwork://register-staff?code=xxx
      // custom scheme 的 pathname 可能是 "register-staff" 或 "/register-staff"
      const path = url.pathname.replace(/^\//, ""); // 移除前导斜杠
      if (path === "register-staff") {
        const code = url.searchParams.get("code");
        if (code) {
          router.navigate({ to: "/register-staff", search: { code } });
          return;
        }
      }
    } catch {
      // 忽略无效 URL
    }
  }
}

// 初始化 deep link 监听
async function initDeepLink() {
  // 检查应用启动时是否有 deep link
  try {
    const urls = await getCurrent();
    if (urls && urls.length > 0) {
      handleDeepLink(urls);
    }
  } catch {
    // 非 Tauri 环境或无初始 deep link
  }

  // 监听后续 deep link 事件
  try {
    await onOpenUrl(handleDeepLink);
  } catch {
    // 非 Tauri 环境
  }
}

// 启动应用后初始化 deep link
initDeepLink();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
