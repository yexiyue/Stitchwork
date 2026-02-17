import { useCallback, useEffect, useState } from "react";
import { isTauri } from "@/utils/platform";

interface UseBiometricReturn {
  /** 设备是否支持生物识别 */
  isAvailable: boolean;
  /** 正在检查可用性 */
  isChecking: boolean;
  /** 触发生物识别验证，返回是否成功 */
  authenticate: (reason?: string) => Promise<boolean>;
}

export function useBiometric(): UseBiometricReturn {
  const [isAvailable, setIsAvailable] = useState(false);
  const [isChecking, setIsChecking] = useState(true);

  useEffect(() => {
    const checkAvailability = async () => {
      if (!isTauri()) {
        setIsAvailable(false);
        setIsChecking(false);
        return;
      }

      try {
        const { checkStatus } = await import("@tauri-apps/plugin-biometric");
        const status = await checkStatus();
        setIsAvailable(status.isAvailable);
      } catch {
        setIsAvailable(false);
      } finally {
        setIsChecking(false);
      }
    };

    checkAvailability();
  }, []);

  const authenticate = useCallback(
    async (reason = "请验证身份以继续"): Promise<boolean> => {
      // 非 Tauri 环境或不支持生物识别，自动通过
      if (!isTauri() || !isAvailable) {
        return true;
      }

      try {
        const { authenticate: bioAuth } = await import(
          "@tauri-apps/plugin-biometric"
        );
        await bioAuth(reason, {
          allowDeviceCredential: true,
        });
        return true;
      } catch {
        return false;
      }
    },
    [isAvailable]
  );

  return {
    isAvailable,
    isChecking,
    authenticate,
  };
}
