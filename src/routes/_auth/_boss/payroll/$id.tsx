import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, ImageViewer } from "antd-mobile";
import { ChevronLeft, ImageIcon } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { payrollApi, getFileUrl } from "@/api";
import { Image, RelativeTime, BiometricGuard } from "@/components";
import { useStaffList } from "@/hooks";
import type { Staff } from "@/types";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_boss/payroll/$id")({
  component: PayrollDetailPage,
});

function PayrollDetailPage() {
  const { id } = Route.useParams();
  const navigate = useNavigate();
  const [imageViewerVisible, setImageViewerVisible] = useState(false);

  const { data: payroll, isLoading } = useQuery({
    queryKey: ["payroll", id],
    queryFn: () => payrollApi.getOne(id),
  });

  const { data: staffList } = useStaffList();
  const staffMap = Object.fromEntries(
    (staffList?.list ?? []).map((s: Staff) => [
      s.id,
      s.displayName || s.username,
    ])
  );

  if (isLoading) {
    return (
      <div className="flex flex-col h-full">
        <NavBar
          onBack={() => navigate({ to: "/payroll" })}
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
          onBack={() => navigate({ to: "/payroll" })}
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
      onCancel={() => navigate({ to: "/payroll" })}
    >
      <div className="flex flex-col h-full bg-gray-50">
        <NavBar
          onBack={() => navigate({ to: "/payroll" })}
          backIcon={<ChevronLeft size={24} />}
        >
          工资详情
        </NavBar>

        <div className="flex-1 overflow-auto">
          {/* 金额卡片 */}
          <div className="bg-gradient-to-r from-orange-400 to-orange-500 text-white p-6 text-center">
            <div className="text-sm opacity-80 mb-2">发放金额</div>
            <div className="text-4xl font-bold">¥{payroll.amount}</div>
          </div>

          {/* 员工信息 */}
          <div className="bg-white mt-2 p-4">
            <div className="text-sm text-gray-500 mb-2">员工</div>
            <div className="font-medium text-lg">
              {staffMap[payroll.userId] || "未知员工"}
            </div>
          </div>

          {/* 发放时间 */}
          <div className="bg-white mt-2 p-4">
            <div className="text-sm text-gray-500 mb-2">发放时间</div>
            <div>
              <RelativeTime date={payroll.paidAt} />
            </div>
          </div>

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
