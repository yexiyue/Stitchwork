import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useQueryClient } from "@tanstack/react-query";
import { Toast } from "antd-mobile";
import { useAuthStore } from "@/stores/auth";

/** 通知 payload 类型 */
interface NotificationPayload {
  type: "record_submitted" | "record_approved" | "record_rejected" | "payroll_received";
  title: string;
  body: string;
}

/**
 * 通知监听 Hook
 *
 * 功能：
 * 1. 登录后自动连接 SSE
 * 2. 监听 Rust 层发来的通知事件
 * 3. 显示 Toast 提示
 * 4. 刷新相关数据
 * 5. 登出时断开连接
 */
export function useNotify() {
  const token = useAuthStore((s) => s.token);
  const queryClient = useQueryClient();

  useEffect(() => {
    if (!token) return;

    let unlisten: UnlistenFn | null = null;

    const setup = async () => {
      try {
        // 连接 SSE
        const apiUrl = import.meta.env.VITE_API_URL || "http://localhost:3000";
        await invoke("connect_sse", { apiUrl, token });

        // 监听通知事件
        unlisten = await listen<NotificationPayload>("notification", (event) => {
          const payload = event.payload;

          // 显示 Toast
          Toast.show({
            icon: "success",
            content: payload.title,
            duration: 3000,
          });

          // 刷新相关数据
          switch (payload.type) {
            case "record_submitted":
              // 老板收到新计件待审核
              queryClient.invalidateQueries({ queryKey: ["piece-records"] });
              queryClient.invalidateQueries({ queryKey: ["home-overview"] });
              break;
            case "record_approved":
            case "record_rejected":
              // 员工计件状态更新
              queryClient.invalidateQueries({ queryKey: ["piece-records"] });
              queryClient.invalidateQueries({ queryKey: ["my-records"] });
              queryClient.invalidateQueries({ queryKey: ["home-overview"] });
              break;
            case "payroll_received":
              // 员工收到工资
              queryClient.invalidateQueries({ queryKey: ["payrolls"] });
              queryClient.invalidateQueries({ queryKey: ["home-overview"] });
              break;
          }
        });
      } catch (error) {
        console.error("Failed to setup notifications:", error);
      }
    };

    setup();

    return () => {
      // 清理：断开 SSE 并取消监听
      invoke("disconnect_sse").catch(console.error);
      unlisten?.();
    };
  }, [token, queryClient]);
}
