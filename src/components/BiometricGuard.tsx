import { useEffect, useState, type ReactNode } from "react";
import { useBiometric } from "@/hooks";
import { SpinLoading } from "antd-mobile";
import { Fingerprint } from "lucide-react";

interface BiometricGuardProps {
  /** 受保护的内容 */
  children: ReactNode;
  /** 验证提示文案 */
  reason?: string;
  /** 取消验证时的回调 */
  onCancel?: () => void;
}

/**
 * 生物识别守卫组件
 *
 * 包裹敏感页面内容，首次访问时触发生物识别验证。
 * 验证成功后当前会话内不再重复验证。
 */
export function BiometricGuard({
  children,
  reason = "请验证身份以继续",
  onCancel,
}: BiometricGuardProps) {
  const { isChecking, isVerified, authenticate } = useBiometric();
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const [authAttempted, setAuthAttempted] = useState(false);

  useEffect(() => {
    // 等待检查完成
    if (isChecking) return;
    // 已验证则不需要认证
    if (isVerified) return;
    // 已经尝试过认证
    if (authAttempted) return;

    const doAuth = async () => {
      setIsAuthenticating(true);
      setAuthAttempted(true);
      const success = await authenticate(reason);
      setIsAuthenticating(false);

      if (!success && onCancel) {
        onCancel();
      }
    };

    doAuth();
  }, [isChecking, isVerified, authAttempted, authenticate, reason, onCancel]);

  // 正在检查或认证中
  if (isChecking || isAuthenticating) {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-4 text-gray-500">
        <Fingerprint size={48} className="text-blue-500" />
        <SpinLoading color="primary" />
        <p className="text-sm">正在验证身份...</p>
      </div>
    );
  }

  // 认证失败（已尝试但未验证）
  if (!isVerified && authAttempted) {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-4 text-gray-500">
        <Fingerprint size={48} className="text-gray-400" />
        <p className="text-sm">身份验证未通过</p>
        <button
          className="px-4 py-2 text-sm text-blue-500 border border-blue-500 rounded-lg"
          onClick={() => setAuthAttempted(false)}
        >
          重新验证
        </button>
      </div>
    );
  }

  // 已验证，显示内容
  return <>{children}</>;
}
