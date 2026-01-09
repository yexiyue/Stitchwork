import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { Toast } from "antd-mobile";
import { useAuthStore } from "@/stores/auth";
import { isTauri } from "@/utils/platform";

const CHANNEL_ID = "stitchwork_high";

/** 通知 payload 类型 */
interface NotificationPayload {
  type:
    | "record_submitted"
    | "record_approved"
    | "record_rejected"
    | "payroll_received"
    | "user_registered"
    | "staff_joined";
  title: string;
  body: string;
}

/**
 * 处理通知 payload，刷新相关数据
 */
function handleNotification(
  payload: NotificationPayload,
  queryClient: ReturnType<typeof useQueryClient>
) {
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
    case "user_registered":
      // 超管收到新用户注册通知
      queryClient.invalidateQueries({ queryKey: ["admin", "users"] });
      break;
    case "staff_joined":
      // 老板收到新员工加入通知
      queryClient.invalidateQueries({ queryKey: ["staff"] });
      queryClient.invalidateQueries({ queryKey: ["home-overview"] });
      break;
  }
}

/**
 * 通知监听 Hook
 *
 * 功能：
 * 1. 登录后自动连接 SSE
 * 2. 显示 Toast 提示
 * 3. 刷新相关数据
 * 4. 登出时断开连接
 *
 * 环境支持：
 * - Tauri: 通过 Rust SSE 客户端 + 本地通知
 * - 浏览器: 通过原生 EventSource + Toast
 */
export function useNotify() {
  const token = useAuthStore((s) => s.token);
  const queryClient = useQueryClient();

  // Tauri 环境
  useEffect(() => {
    if (!token || !isTauri()) return;

    let unlisten: (() => void) | null = null;
    let disconnectOnCleanup = true;

    const setup = async () => {
      try {
        // 动态导入 Tauri 模块
        const { invoke } = await import("@tauri-apps/api/core");
        const { listen } = await import("@tauri-apps/api/event");
        const { platform } = await import("@tauri-apps/plugin-os");
        const {
          isPermissionGranted,
          requestPermission,
          createChannel,
          Importance,
        } = await import("@tauri-apps/plugin-notification");

        // 检查并请求通知权限 (Android 13+ 需要)
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
          const permission = await requestPermission();
          permissionGranted = permission === "granted";
        }

        // 创建高优先级通知渠道 (Android 8+ only)
        let channelId: string | undefined;
        const os = platform();
        if (os === "android") {
          try {
            // 直接创建，已存在会自动忽略
            await createChannel({
              id: CHANNEL_ID,
              name: "重要通知",
              importance: Importance.High,
              vibration: true,
            });
          } catch {
            // 渠道已存在或创建失败，忽略
          }
          channelId = CHANNEL_ID;
        }

        // 连接 SSE，传递 channel_id (仅 Android)
        const apiUrl =
          import.meta.env.VITE_API_URL || "http://47.115.172.218:8080";
        await invoke("connect_sse", { apiUrl, token, channelId });

        // 监听通知事件
        unlisten = await listen<NotificationPayload>(
          "notification",
          (event) => {
            handleNotification(event.payload, queryClient);
          }
        );
      } catch (error) {
        console.error("Failed to setup Tauri notifications:", error);
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

  // 浏览器环境
  useEffect(() => {
    if (!token || isTauri()) return;

    const apiUrl = import.meta.env.VITE_API_URL || "http://47.115.172.218:8080";
    // 浏览器 EventSource 不支持自定义 headers，使用 query parameter
    const url = `${apiUrl}/api/sse/events?token=${encodeURIComponent(token)}`;

    let eventSource: EventSource | null = null;
    let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

    const connect = () => {
      eventSource = new EventSource(url);

      eventSource.onopen = () => {
        console.log("SSE connected (browser)");
      };

      eventSource.onmessage = (event) => {
        try {
          const payload = JSON.parse(event.data) as NotificationPayload;
          handleNotification(payload, queryClient);
        } catch (error) {
          console.error("Failed to parse SSE message:", error);
        }
      };

      eventSource.onerror = () => {
        console.warn("SSE error, reconnecting in 5s...");
        eventSource?.close();
        eventSource = null;
        // 5 秒后重连
        reconnectTimer = setTimeout(connect, 5000);
      };
    };

    connect();

    return () => {
      if (reconnectTimer) {
        clearTimeout(reconnectTimer);
      }
      eventSource?.close();
    };
  }, [token, queryClient]);
}
