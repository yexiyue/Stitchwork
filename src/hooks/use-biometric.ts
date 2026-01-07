import { useCallback, useEffect, useState } from "react";
import { useBiometricStore } from "@/stores/biometric";
import { isTauri } from "@/utils/platform";

interface UseBiometricReturn {
  /** 设备是否支持生物识别 */
  isAvailable: boolean;
  /** 当前会话是否已验证 */
  isVerified: boolean;
  /** 正在检查可用性 */
  isChecking: boolean;
  /** 触发生物识别验证 */
  authenticate: (reason?: string) => Promise<boolean>;
}

export function useBiometric(): UseBiometricReturn {
  const [isAvailable, setIsAvailable] = useState(false);
  const [isChecking, setIsChecking] = useState(true);
  const { verified, setVerified } = useBiometricStore();

  // 检查生物识别是否可用
  useEffect(() => {
    const checkAvailability = async () => {
      if (!isTauri()) {
        // 非 Tauri 环境，自动跳过
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
      // 已验证则直接返回
      if (verified) {
        return true;
      }

      // 非 Tauri 环境或不支持生物识别，自动通过
      if (!isTauri() || !isAvailable) {
        setVerified(true);
        return true;
      }

      try {
        const { authenticate: bioAuth } = await import(
          "@tauri-apps/plugin-biometric"
        );
        await bioAuth(reason, {
          allowDeviceCredential: true,
        });
        setVerified(true);
        return true;
      } catch {
        // 用户取消或验证失败
        return false;
      }
    },
    [verified, isAvailable, setVerified]
  );

  return {
    isAvailable,
    isVerified: verified || !isTauri() || (!isChecking && !isAvailable),
    isChecking,
    authenticate,
  };
}
