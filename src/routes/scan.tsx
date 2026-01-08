import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useEffect, useRef } from "react";
import { Toast } from "antd-mobile";
import { X, Image } from "lucide-react";
import { scan, Format, cancel } from "@tauri-apps/plugin-barcode-scanner";
import jsQR from "jsqr";

export const Route = createFileRoute("/scan")({
  component: ScanPage,
});

function ScanPage() {
  const navigate = useNavigate();
  const scanning = useRef(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  // 处理扫描结果
  const handleResult = (content: string) => {
    try {
      const url = new URL(content);
      const path = url.pathname.replace(/^\//, "") || url.host;
      // 兼容 register 和旧的 register-staff 路径
      if (path === "register" || path === "register-staff") {
        const code = url.searchParams.get("code");
        if (code) {
          navigate({ to: "/register", search: { code } });
          return true;
        }
      }
    } catch {
      // 不是有效 URL
    }
    Toast.show({ content: "无效的邀请码", icon: "fail" });
    return false;
  };

  useEffect(() => {
    // 设置背景透明
    document.body.style.background = "transparent";

    const startScan = async () => {
      if (scanning.current) return;
      scanning.current = true;

      try {
        // 权限已在登录页面检查，直接开始扫描
        const result = await scan({ formats: [Format.QRCode], windowed: true });

        // 恢复背景
        document.body.style.background = "";

        if (result?.content) {
          if (!handleResult(result.content)) {
            navigate({ to: "/login" });
          }
        } else {
          navigate({ to: "/login" });
        }
      } catch (e) {
        document.body.style.background = "";
        if (e instanceof Error && !e.message.includes("cancel")) {
          Toast.show({ content: "扫码失败", icon: "fail" });
        }
        navigate({ to: "/login" });
      }
    };

    startScan();

    return () => {
      document.body.style.background = "";
      cancel().catch(() => {});
    };
  }, [navigate]);

  const handleCancel = async () => {
    document.body.style.background = "";
    await cancel().catch(() => {});
    navigate({ to: "/login" });
  };

  // 从图片扫描
  const handleImageScan = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    try {
      // 暂停相机扫描
      await cancel().catch(() => {});

      const img = new window.Image();
      img.src = URL.createObjectURL(file);

      img.onload = () => {
        const canvas = document.createElement("canvas");
        canvas.width = img.width;
        canvas.height = img.height;
        const ctx = canvas.getContext("2d");
        if (!ctx) {
          Toast.show({ content: "无法处理图片", icon: "fail" });
          navigate({ to: "/login" });
          return;
        }

        ctx.drawImage(img, 0, 0);
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const code = jsQR(imageData.data, imageData.width, imageData.height);

        URL.revokeObjectURL(img.src);
        document.body.style.background = "";

        if (code?.data) {
          if (!handleResult(code.data)) {
            navigate({ to: "/login" });
          }
        } else {
          Toast.show({ content: "未识别到二维码", icon: "fail" });
          navigate({ to: "/login" });
        }
      };

      img.onerror = () => {
        URL.revokeObjectURL(img.src);
        Toast.show({ content: "图片加载失败", icon: "fail" });
        navigate({ to: "/login" });
      };
    } catch {
      Toast.show({ content: "识别失败", icon: "fail" });
      navigate({ to: "/login" });
    }

    // 清空 input 以便重复选择同一文件
    e.target.value = "";
  };

  return (
    <div className="fixed inset-0 bg-transparent">
      {/* 扫描框 */}
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="relative w-64 h-64">
          {/* 四个角 */}
          <div className="absolute top-0 left-0 w-8 h-8 border-t-4 border-l-4 border-white rounded-tl-lg" />
          <div className="absolute top-0 right-0 w-8 h-8 border-t-4 border-r-4 border-white rounded-tr-lg" />
          <div className="absolute bottom-0 left-0 w-8 h-8 border-b-4 border-l-4 border-white rounded-bl-lg" />
          <div className="absolute bottom-0 right-0 w-8 h-8 border-b-4 border-r-4 border-white rounded-br-lg" />
          {/* 扫描线动画 */}
          <div className="absolute top-0 left-0 right-0 h-0.5 bg-green-500 animate-scan" />
        </div>
      </div>

      {/* 提示文字 */}
      <div className="absolute bottom-32 left-0 right-0 text-center text-white text-sm">
        将二维码放入框内，即可自动扫描
      </div>

      {/* 从相册选择按钮 */}
      <button
        onClick={() => fileInputRef.current?.click()}
        className="absolute bottom-16 left-1/2 -translate-x-1/2 flex items-center gap-2 px-4 py-2 bg-black/50 rounded-full text-white text-sm"
      >
        <Image size={18} />
        从相册选择
      </button>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        className="hidden"
        onChange={handleImageScan}
      />

      {/* 取消按钮 */}
      <button
        onClick={handleCancel}
        className="absolute top-12 right-4 w-10 h-10 flex items-center justify-center bg-black/50 rounded-full"
      >
        <X size={24} className="text-white" />
      </button>
    </div>
  );
}
