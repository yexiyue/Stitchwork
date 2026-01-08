import React from "react";
import ReactDOM, { createRoot } from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { unstableSetRender } from "antd-mobile";
import { routeTree } from "./routeTree.gen";
import { isTauri } from "@/utils/platform";
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
      // 处理 stitchwork://register?code=xxx
      // custom scheme 的 pathname 可能是 "register" 或 "/register"
      const path = url.pathname.replace(/^\//, ""); // 移除前导斜杠
      if (path === "register" || path === "register-staff") {
        // 兼容旧的 register-staff deep link
        const code = url.searchParams.get("code");
        if (code) {
          router.navigate({ to: "/register", search: { code } });
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
  if (!isTauri()) return;

  try {
    const { onOpenUrl, getCurrent } = await import(
      "@tauri-apps/plugin-deep-link"
    );

    // 检查应用启动时是否有 deep link
    const urls = await getCurrent();
    if (urls && urls.length > 0) {
      handleDeepLink(urls);
    }

    // 监听后续 deep link 事件
    await onOpenUrl(handleDeepLink);
  } catch {
    // deep link 插件不可用
  }
}

// 启动应用后初始化 deep link
initDeepLink();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
