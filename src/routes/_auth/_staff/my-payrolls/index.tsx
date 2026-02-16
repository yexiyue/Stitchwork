import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Dropdown, NavBar } from "antd-mobile";
import { ImageIcon, Calendar, ChevronLeft } from "lucide-react";
import type { Payroll } from "@/types";
import {
  RelativeTime,
  VirtualList,
  DateRangeButton,
  BiometricGuard,
  Image,
} from "@/components";
import { payrollApi } from "@/api";
import { useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import { useInfiniteList, useDateRange } from "@/hooks";

export const Route = createFileRoute("/_auth/_staff/my-payrolls/")({
  component: MyPayrollsPage,
});

function MyPayrollsPage() {
  const navigate = useNavigate();
  const dropdownRef = useRef<DropdownRef>(null);

  const {
    startDate,
    endDate,
    dateParams,
    calendarVisible,
    setCalendarVisible,
    handleCalendarConfirm,
    hasDateFilter,
  } = useDateRange({ nullable: true });

  const { list, isFetching, hasMore, loadMore, refresh } =
    useInfiniteList<Payroll>(
      ["my-payrolls", startDate?.toISOString(), endDate?.toISOString()],
      (params) =>
        payrollApi.list({
          ...params,
          startDate: dateParams.startDate,
          endDate: dateParams.endDate,
        })
    );

  // 计算总金额
  const totalAmount = list.reduce(
    (sum, p) => sum + parseFloat(p.amount || "0"),
    0
  );

  return (
    <BiometricGuard
      reason="查看工资记录需要验证身份"
      onCancel={() => navigate({ to: "/" })}
    >
      <div className="flex flex-col h-full overflow-hidden">
        <NavBar
          onBack={() => navigate({ to: "/profile" })}
          backIcon={<ChevronLeft size={24} />}
          right={
            <div className="flex justify-end">
              <Dropdown ref={dropdownRef}>
                <Dropdown.Item
                  key="date"
                  title={
                    <Calendar
                      size={16}
                      className={
                        hasDateFilter ? "text-blue-500" : "text-gray-500"
                      }
                    />
                  }
                  onClick={() => setCalendarVisible(true)}
                />
              </Dropdown>
            </div>
          }
        >
          工资记录
        </NavBar>

        {/* 顶部统计卡片 */}
        <div className="p-4 pb-2">
          <div className="bg-linear-to-r from-green-500 to-green-600 rounded-lg p-4 text-white">
            <div className="text-sm opacity-80 mb-1">累计收入</div>
            <div className="text-3xl font-bold">¥{totalAmount.toFixed(2)}</div>
            <div className="text-xs opacity-70 mt-1">共 {list.length} 笔</div>
          </div>
        </div>

        {/* 列表 */}
        <div className="flex flex-1 overflow-hidden">
          <VirtualList
            data={list}
            loading={isFetching}
            hasMore={hasMore}
            onLoadMore={loadMore}
            onRefresh={refresh}
            keyExtractor={(r) => r.id}
            emptyText="暂无工资记录"
            searchEmpty={!!hasDateFilter && !list.length}
            estimateSize={94}
            renderItem={(payroll) => (
              <div
                className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3 cursor-pointer active:bg-gray-50"
                onClick={() =>
                  navigate({
                    to: "/my-payrolls/$id",
                    params: { id: payroll.id },
                  })
                }
              >
                <div className="shrink-0 h-16 aspect-square rounded-lg overflow-hidden bg-gray-100 flex items-center justify-center">
                  {payroll.paymentImage ? (
                    <Image
                      src={payroll.paymentImage}
                      width="100%"
                      height="100%"
                      fit="cover"
                    />
                  ) : (
                    <ImageIcon size={24} className="text-gray-400" />
                  )}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between items-start mb-1">
                    <span className="text-lg font-medium text-green-600">
                      +¥{payroll.amount}
                    </span>
                  </div>
                  {payroll.note && (
                    <div className="text-sm text-gray-600 truncate">
                      {payroll.note}
                    </div>
                  )}
                  <div className="text-xs text-gray-400 mt-0.5">
                    <RelativeTime date={payroll.paidAt} />
                  </div>
                </div>
              </div>
            )}
          />
        </div>

        {/* 日期选择器 */}
        <DateRangeButton
          startDate={startDate}
          endDate={endDate}
          visible={calendarVisible}
          onVisibleChange={(v) => {
            setCalendarVisible(v);
            if (!v) dropdownRef.current?.close();
          }}
          onConfirm={(dates) => {
            handleCalendarConfirm(dates);
            dropdownRef.current?.close();
          }}
          showIcon={false}
        />
      </div>
    </BiometricGuard>
  );
}
