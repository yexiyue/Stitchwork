import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { shareApi, getFileUrl } from "@/api";
import { Phone, MapPin, Download, Loader2 } from "lucide-react";
import { QRCodeSVG } from "qrcode.react";
import { useRef, useState } from "react";
import { Toast } from "antd-mobile";

export const Route = createFileRoute("/share/$token")({
  component: SharePage,
});

function SharePage() {
  const { token } = Route.useParams();
  const contentRef = useRef<HTMLDivElement>(null);
  const [exporting, setExporting] = useState(false);

  const { data, isLoading, error } = useQuery({
    queryKey: ["public-share", token],
    queryFn: () => shareApi.getPublic(token),
  });

  const handleExport = async () => {
    if (!contentRef.current) return;

    setExporting(true);
    try {
      const html2canvas = (await import("html2canvas")).default;
      const canvas = await html2canvas(contentRef.current, {
        scale: 2,
        useCORS: true,
        backgroundColor: "#f5f5f5",
      });

      const link = document.createElement("a");
      link.download = `${data?.title || "招工信息"}.png`;
      link.href = canvas.toDataURL("image/png");
      link.click();

      Toast.show({ content: "图片已保存" });
    } catch (e) {
      Toast.show({ content: "导出失败，请重试" });
      console.error(e);
    } finally {
      setExporting(false);
    }
  };

  const handleCall = () => {
    if (data?.bossPhone) {
      window.location.href = `tel:${data.bossPhone}`;
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <Loader2 className="w-8 h-8 animate-spin text-blue-500" />
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="text-center text-gray-500">
          <div className="text-6xl mb-4">404</div>
          <div>分享不存在或已停用</div>
        </div>
      </div>
    );
  }

  const shareUrl = window.location.href;

  return (
    <div className="min-h-screen bg-gray-100">
      {/* 可导出内容区域 */}
      <div ref={contentRef} className="bg-gray-100 pb-4">
        {/* 头部 */}
        <div className="bg-gradient-to-r from-blue-500 to-blue-600 text-white p-6">
          <div className="flex items-center gap-4">
            {data.avatar ? (
              <img
                src={getFileUrl(data.avatar)}
                alt="头像"
                className="w-16 h-16 rounded-full object-cover border-2 border-white/30"
              />
            ) : (
              <div className="w-16 h-16 rounded-full bg-white/20 flex items-center justify-center text-2xl font-bold">
                {data.workshopName?.[0] || "招"}
              </div>
            )}
            <div>
              <h1 className="text-xl font-bold">{data.title}</h1>
              {data.workshopName && (
                <div className="text-white/80 text-sm mt-1">
                  {data.workshopName}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* 联系信息 */}
        {(data.bossPhone || data.workshopAddress) && (
          <div className="bg-white mx-4 mt-4 rounded-lg p-4 shadow-sm">
            {data.bossPhone && (
              <div
                className="flex items-center gap-3 cursor-pointer active:bg-gray-50 -mx-2 px-2 py-2 rounded"
                onClick={handleCall}
              >
                <div className="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center">
                  <Phone className="w-5 h-5 text-green-600" />
                </div>
                <div>
                  <div className="text-sm text-gray-500">联系电话</div>
                  <div className="text-blue-600 font-medium">
                    {data.bossPhone}
                  </div>
                </div>
              </div>
            )}
            {data.workshopAddress && (
              <div className="flex items-center gap-3 mt-3">
                <div className="w-10 h-10 rounded-full bg-orange-100 flex items-center justify-center">
                  <MapPin className="w-5 h-5 text-orange-600" />
                </div>
                <div>
                  <div className="text-sm text-gray-500">工坊地址</div>
                  <div className="text-gray-700">{data.workshopAddress}</div>
                </div>
              </div>
            )}
          </div>
        )}

        {/* 工序列表 */}
        <div className="mx-4 mt-4">
          <h2 className="text-lg font-medium mb-3">招工岗位</h2>
          <div className="grid grid-cols-2 gap-3">
            {data.processes.map((process) => (
              <div
                key={process.id}
                className="bg-white rounded-lg p-4 shadow-sm"
              >
                <div className="font-medium text-gray-800 truncate">
                  {process.name}
                </div>
                <div className="text-xs text-gray-400 truncate mt-0.5">
                  {process.orderProductName}
                </div>
                <div className="flex justify-between items-end mt-3">
                  <div>
                    <div className="text-xs text-gray-500">单价</div>
                    <div className="text-orange-500 font-bold">
                      ¥{process.piecePrice}
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-xs text-gray-500">剩余</div>
                    <div className="text-blue-600 font-medium">
                      {process.remainingQuantity}件
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
          {data.processes.length === 0 && (
            <div className="bg-white rounded-lg p-8 text-center text-gray-400">
              暂无招工岗位
            </div>
          )}
        </div>

        {/* 二维码 */}
        <div className="bg-white mx-4 mt-4 rounded-lg p-4 shadow-sm">
          <div className="flex items-center justify-center gap-4">
            <QRCodeSVG value={shareUrl} size={80} />
            <div className="text-gray-500 text-sm">
              <div>扫码查看详情</div>
              <div className="text-xs text-gray-400 mt-1">长按保存图片分享</div>
            </div>
          </div>
        </div>
      </div>

      {/* 导出按钮 - 固定在底部 */}
      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white border-t">
        <button
          className="w-full bg-blue-500 text-white py-3 rounded-lg font-medium flex items-center justify-center gap-2 active:bg-blue-600 disabled:opacity-50"
          onClick={handleExport}
          disabled={exporting}
        >
          {exporting ? (
            <Loader2 className="w-5 h-5 animate-spin" />
          ) : (
            <Download className="w-5 h-5" />
          )}
          {exporting ? "正在生成..." : "保存为图片"}
        </button>
      </div>

      {/* 底部占位 */}
      <div className="h-20" />
    </div>
  );
}
