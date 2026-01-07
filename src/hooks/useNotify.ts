import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { Toast } from "antd-mobile";
import { useAuthStore } from "@/stores/auth";
import { isTauri } from "@/utils/platform";

const CHANNEL_ID = "stitchwork_high";

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
 *
 * 注意：仅在 Tauri 环境下生效，浏览器环境跳过
 */
export function useNotify() {
  const token = useAuthStore((s) => s.token);
  const queryClient = useQueryClient();

  useEffect(() => {
    if (!token || !isTauri()) return;

    let unlisten: (() => void) | null = null;
    let disconnectOnCleanup = true;

    const setup = async () => {
      try {
        // 动态导入 Tauri 模块
        const { invoke } = await import("@tauri-apps/api/core");
        const { listen } = await import("@tauri-apps/api/event");
        const {
          isPermissionGranted,
          requestPermission,
          createChannel,
          channels,
          Importance,
        } = await import("@tauri-apps/plugin-notification");

        // 检查并请求通知权限 (Android 13+ 需要)
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
          const permission = await requestPermission();
          permissionGranted = permission === "granted";
        }

        // 创建高优先级通知渠道 (Android 8+)
        const existingChannels = await channels();
        if (!existingChannels.some((c) => c.id === CHANNEL_ID)) {
          await createChannel({
            id: CHANNEL_ID,
            name: "重要通知",
            importance: Importance.High,
            vibration: true,
          });
        }

        // 连接 SSE，传递 channel_id
        const apiUrl = import.meta.env.VITE_API_URL || "http://localhost:3000";
        await invoke("connect_sse", { apiUrl, token, channelId: CHANNEL_ID });

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
        disconnectOnCleanup = false;
      }
    };

    setup();

    return () => {
      // 清理：断开 SSE 并取消监听
      if (disconnectOnCleanup) {
        import("@tauri-apps/api/core")
          .then(({ invoke }) => invoke("disconnect_sse"))
          .catch(console.error);
      }
      unlisten?.();
    };
  }, [token, queryClient]);
}
