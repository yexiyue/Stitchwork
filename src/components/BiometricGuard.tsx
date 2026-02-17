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
 * 包裹敏感页面内容，每次进入页面都触发生物识别验证。
 */
export function BiometricGuard({
  children,
  reason = "请验证身份以继续",
  onCancel,
}: BiometricGuardProps) {
  const { isChecking, authenticate } = useBiometric();
  const [status, setStatus] = useState<"pending" | "authenticating" | "success" | "failed">("pending");

  useEffect(() => {
    if (isChecking) return;
    if (status !== "pending") return;

    const doAuth = async () => {
      setStatus("authenticating");
      const success = await authenticate(reason);
      setStatus(success ? "success" : "failed");
      if (!success && onCancel) {
        onCancel();
      }
    };

    doAuth();
  }, [isChecking, status, authenticate, reason, onCancel]);

  if (isChecking || status === "pending" || status === "authenticating") {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-4 text-gray-500">
        <Fingerprint size={48} className="text-blue-500" />
        <SpinLoading color="primary" />
        <p className="text-sm">正在验证身份...</p>
      </div>
    );
  }

  if (status === "failed") {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-4 text-gray-500">
        <Fingerprint size={48} className="text-gray-400" />
        <p className="text-sm">身份验证未通过</p>
        <button
          className="px-4 py-2 text-sm text-blue-500 border border-blue-500 rounded-lg"
          onClick={() => setStatus("pending")}
        >
          重新验证
        </button>
      </div>
    );
  }

  return <>{children}</>;
}
