import { useEffect } from "react";
import { useBiometricStore } from "@/stores/biometric";

/**
 * 生物识别超时处理 Hook
 *
 * 监听应用可见性变化，当应用从后台切换回来时，
 * 检查是否超过了设定的超时时间（5分钟），如果超时则清除验证状态。
 */
export function useBiometricTimeout() {
  const { setBackground, checkAndClearIfExpired } = useBiometricStore();

  useEffect(() => {
    const handleVisibilityChange = () => {
      if (document.visibilityState === "hidden") {
        // 应用进入后台，记录时间
        setBackground();
      } else if (document.visibilityState === "visible") {
        // 应用回到前台，检查是否超时
        checkAndClearIfExpired();
      }
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [setBackground, checkAndClearIfExpired]);
}
