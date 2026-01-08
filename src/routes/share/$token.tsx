import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { shareApi, getFileUrl } from "@/api";
import { Phone, MapPin, Download, Loader2 } from "lucide-react";
import { QRCodeSVG } from "qrcode.react";
import { useRef, useState } from "react";
import { Toast } from "antd-mobile";
import { OssImage } from "@/components";
import { motion, useScroll, useTransform } from "motion/react";

export const Route = createFileRoute("/share/$token")({
  component: SharePage,
});

const HEADER_MAX = 200;
const HEADER_MIN = 56;

function SharePage() {
  const { token } = Route.useParams();
  const contentRef = useRef<HTMLDivElement>(null);
  const scrollRef = useRef<HTMLDivElement>(null);
  const [exporting, setExporting] = useState(false);

  const { scrollY } = useScroll({ container: scrollRef });
  const headerHeight = useTransform(
    scrollY,
    [0, HEADER_MAX - HEADER_MIN],
    [HEADER_MAX, HEADER_MIN]
  );
  const headerOpacity = useTransform(scrollY, [0, 80], [1, 0]);
  const titleBarOpacity = useTransform(scrollY, [60, 120], [0, 1]);

  const { data, isLoading, error } = useQuery({
    queryKey: ["public-share", token],
    queryFn: () => shareApi.getPublic(token),
  });

  const handleExport = async () => {
    if (!contentRef.current) return;

    setExporting(true);
    try {
      const { domToPng } = await import("modern-screenshot");
      const dataUrl = await domToPng(contentRef.current, {
        scale: 2,
        backgroundColor: "#f5f5f5",
      });

      const link = document.createElement("a");
      link.download = `${data?.title || "招工信息"}.png`;
      link.href = dataUrl;
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

  const shareUrl = typeof window !== "undefined" ? window.location.href : "";
  const bgImage = data?.workshopImage;

  return (
    <div className="h-full bg-gray-100 flex flex-col">
      {/* 可折叠头部 */}
      <motion.div
        className="relative overflow-hidden shrink-0"
        style={{ height: data ? headerHeight : HEADER_MIN }}
      >
        {/* 背景图 */}
        {bgImage ? (
          <img
            src={getFileUrl(bgImage)}
            alt=""
            className="absolute inset-0 w-full h-full object-cover"
          />
        ) : (
          <div className="absolute inset-0 bg-gradient-to-r from-blue-500 to-blue-600" />
        )}
        <div className="absolute inset-0 bg-black/40" />

        {/* 展开状态内容 */}
        {data && (
          <motion.div
            className="absolute inset-0 flex flex-col justify-end p-4 text-white"
            style={{ opacity: headerOpacity }}
          >
            <div className="flex items-center gap-3">
              {data.avatar ? (
                <img
                  src={getFileUrl(data.avatar)}
                  alt=""
                  className="w-14 h-14 rounded-full object-cover border-2 border-white/30"
                />
              ) : (
                <div className="w-14 h-14 rounded-full bg-white/20 flex items-center justify-center text-xl font-bold">
                  {data.workshopName?.[0] || "招"}
                </div>
              )}
              <div className="flex-1 min-w-0">
                <h1 className="text-xl font-bold truncate">{data.title}</h1>
                {data.workshopName && (
                  <div className="text-white/80 text-sm">
                    {data.workshopName}
                  </div>
                )}
              </div>
            </div>
            {data.workshopDesc && (
              <div className="mt-2 text-white/90 text-sm line-clamp-2">
                {data.workshopDesc}
              </div>
            )}
          </motion.div>
        )}

        {/* 收起状态标题栏 */}
        {data && (
          <motion.div
            className="absolute inset-0 flex items-center justify-center text-white"
            style={{ opacity: titleBarOpacity }}
          >
            <div className="font-bold text-lg truncate px-4">{data.title}</div>
          </motion.div>
        )}
      </motion.div>

      {/* 滚动内容 */}
      <div ref={scrollRef} className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="flex items-center justify-center py-20">
            <Loader2 className="w-8 h-8 animate-spin text-blue-500" />
          </div>
        ) : error || !data ? (
          <div className="flex items-center justify-center py-20">
            <div className="text-center text-gray-500">
              <div className="text-6xl mb-4">404</div>
              <div>分享不存在或已停用</div>
            </div>
          </div>
        ) : (
          <>
            <div ref={contentRef} className="bg-gray-100 pb-4">
              {/* 联系信息 */}
              {(data.bossPhone || data.workshopAddress || data.description) && (
                <div className="bg-white mx-4 mt-4 rounded-lg p-4 shadow-sm">
                  {data.description && (
                    <div className="text-gray-600 text-sm mb-3 pb-3 border-b border-gray-100">
                      {data.description}
                    </div>
                  )}
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
                        <div className="text-gray-700">
                          {data.workshopAddress}
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              )}

              {/* 工序列表 */}
              <div className="mx-4 mt-4">
                <h2 className="text-lg font-medium mb-3">招工岗位</h2>
                <div className="space-y-3">
                  {data.processes.map((process) => (
                    <div
                      key={process.id}
                      className="bg-white rounded-lg p-3 shadow-sm"
                    >
                      <div className="flex gap-3">
                        {process.orderImages.length > 0 && (
                          <OssImage
                            src={process.orderImages[0]}
                            className="w-16 h-16 rounded object-cover shrink-0"
                          />
                        )}
                        <div className="flex-1 min-w-0">
                          <div className="flex justify-between items-start">
                            <div className="font-medium text-gray-800 truncate">
                              {process.name}
                            </div>
                            <div className="text-orange-500 font-bold shrink-0 ml-2">
                              ¥{process.piecePrice}
                            </div>
                          </div>
                          <div className="text-xs text-gray-400 truncate mt-0.5">
                            {process.orderProductName}
                          </div>
                          <div className="flex justify-between items-end mt-1 gap-2">
                            <div className="text-xs text-gray-500 line-clamp-2">
                              {process.description || ""}
                            </div>
                            <div className="text-xs text-blue-600 shrink-0">
                              剩余{process.remainingQuantity}件
                            </div>
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
                    <div className="text-xs text-gray-400 mt-1">
                      长按保存图片分享
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div className="h-24"></div>
          </>
        )}
      </div>

      {/* 导出按钮 */}
      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white border-t border-gray-200">
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
    </div>
  );
}
