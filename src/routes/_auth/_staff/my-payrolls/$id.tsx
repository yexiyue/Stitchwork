import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, ImageViewer } from "antd-mobile";
import { ChevronLeft, ImageIcon } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { payrollApi, getFileUrl } from "@/api";
import { Image, RelativeTime, BiometricGuard } from "@/components";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_staff/my-payrolls/$id")({
  component: MyPayrollDetailPage,
});

function MyPayrollDetailPage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const [imageViewerVisible, setImageViewerVisible] = useState(false);

  const { data: payroll, isLoading } = useQuery({
    queryKey: ["my-payroll", id],
    queryFn: () => payrollApi.getOne(id),
  });

  if (isLoading) {
    return (
      <div className="flex flex-col h-full">
        <NavBar
          onBack={() => navigate({ to: "/my-payrolls" })}
          backIcon={<ChevronLeft size={24} />}
        >
          工资详情
        </NavBar>
        <div className="flex-1 flex items-center justify-center">加载中...</div>
      </div>
    );
  }

  if (!payroll) {
    return (
      <div className="flex flex-col h-full">
        <NavBar
          onBack={() => navigate({ to: "/my-payrolls" })}
          backIcon={<ChevronLeft size={24} />}
        >
          工资详情
        </NavBar>
        <div className="flex-1 flex items-center justify-center text-gray-500">
          记录不存在
        </div>
      </div>
    );
  }

  return (
    <BiometricGuard
      reason="查看工资详情需要验证身份"
      onCancel={() => navigate({ to: "/my-payrolls" })}
    >
      <div className="flex flex-col h-full bg-gray-50">
        <NavBar
          onBack={() => navigate({ to: "/my-payrolls" })}
          backIcon={<ChevronLeft size={24} />}
        >
          工资详情
        </NavBar>

        <div className="flex-1 overflow-auto">
          {/* 金额卡片 */}
          <div className="bg-linear-to-r from-green-500 to-green-600 text-white p-6 text-center">
            <div className="text-sm opacity-80 mb-2">收到工资</div>
            <div className="text-4xl font-bold">+¥{payroll.amount}</div>
          </div>

          {/* 发放时间 */}
          <div className="bg-white mt-2 p-4">
            <div className="text-sm text-gray-500 mb-2">发放时间</div>
            <div>
              <RelativeTime date={payroll.paidAt} />
            </div>
          </div>

          {/* 计件记录 */}
          {payroll.records.length > 0 && (
            <div className="bg-white mt-2 p-4">
              <div className="text-sm text-gray-500 mb-3">
                结算记录 ({payroll.records.length}条)
              </div>
              <div className="space-y-3">
                {payroll.records.map((r) => (
                  <div key={r.id} className="flex gap-3">
                    {r.orderImages?.[0] ? (
                      <Image
                        src={r.orderImages[0]}
                        width={56}
                        height={56}
                        fit="cover"
                        className="rounded-lg shrink-0"
                      />
                    ) : (
                      <div className="w-14 h-14 bg-gray-100 rounded-lg shrink-0" />
                    )}
                    <div className="flex-1 min-w-0 flex justify-between">
                      <div className="flex flex-col justify-between">
                        <div className="text-sm truncate">
                          {r.orderName || "未知订单"}
                        </div>
                        <div className="text-xs text-gray-400">
                          {r.processName || "未知工序"}
                        </div>
                        <div className="text-xs text-gray-400">
                          <RelativeTime date={r.recordedAt} />
                        </div>
                      </div>
                      <div className="flex flex-col items-end justify-between shrink-0 ml-2">
                        <div className="text-xs text-gray-400">
                          {r.quantity}件 × ¥{r.piecePrice}
                        </div>
                        <div className="text-green-500 font-medium">
                          ¥{r.amount}
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* 支付凭证 */}
          <div className="bg-white mt-2 p-4">
            <div className="text-sm text-gray-500 mb-2">支付凭证</div>
            {payroll.paymentImage ? (
              <div
                className="h-40 w-40 rounded-lg overflow-hidden cursor-pointer"
                onClick={() => setImageViewerVisible(true)}
              >
                <Image
                  src={payroll.paymentImage}
                  width="100%"
                  height="100%"
                  fit="cover"
                />
              </div>
            ) : (
              <div className="h-20 w-20 bg-gray-100 rounded-lg flex items-center justify-center">
                <ImageIcon size={24} className="text-gray-400" />
              </div>
            )}
          </div>

          {/* 备注 */}
          {payroll.note && (
            <div className="bg-white mt-2 p-4">
              <div className="text-sm text-gray-500 mb-2">备注</div>
              <div className="text-gray-700">{payroll.note}</div>
            </div>
          )}
        </div>

        {/* 图片查看器 */}
        {payroll.paymentImage && (
          <ImageViewer
            image={getFileUrl(payroll.paymentImage)}
            visible={imageViewerVisible}
            onClose={() => setImageViewerVisible(false)}
          />
        )}
      </div>
    </BiometricGuard>
  );
}
