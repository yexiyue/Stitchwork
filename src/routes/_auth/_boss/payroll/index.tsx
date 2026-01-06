import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Dropdown, Image } from "antd-mobile";
import { Plus, ImageIcon, Calendar, Users } from "lucide-react";
import type { Payroll, Staff } from "@/types";
import { RelativeTime, VirtualList, DateRangeButton } from "@/components";
import { payrollApi } from "@/api";
import { useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import { useStaffList, useInfiniteList, useToggleFilter, useDateRange } from "@/hooks";

export const Route = createFileRoute("/_auth/_boss/payroll/")({
  component: PayrollPage,
});

function PayrollPage() {
  const navigate = useNavigate();
  const dropdownRef = useRef<DropdownRef>(null);

  const userFilter = useToggleFilter<string>();

  const {
    startDate,
    endDate,
    dateParams,
    calendarVisible,
    setCalendarVisible,
    handleCalendarConfirm,
    hasDateFilter,
  } = useDateRange({ nullable: true });

  const { data: staffList } = useStaffList();

  const { list, isFetching, hasMore, loadMore, refresh } =
    useInfiniteList<Payroll>(
      [
        "payrolls",
        userFilter.selected[0] ?? "",
        startDate?.toISOString(),
        endDate?.toISOString(),
      ],
      (params) =>
        payrollApi.list({
          ...params,
          userId: userFilter.selected[0] || undefined,
          startDate: dateParams.startDate,
          endDate: dateParams.endDate,
        })
    );

  const staffOptions = (staffList?.list ?? []).map((s: Staff) => ({
    label: s.displayName || s.username,
    value: s.id,
  }));

  const staffMap = Object.fromEntries(
    (staffList?.list ?? []).map((s: Staff) => [s.id, s.displayName || s.username])
  );

  const hasFilters = userFilter.hasSelected || hasDateFilter;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* 顶部栏 */}
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between">
          <h1 className="text-xl">工资管理</h1>
          <div className="flex items-center gap-3">
            <Plus
              size={20}
              className="text-blue-500"
              onClick={() => navigate({ to: "/payroll/new" })}
            />
          </div>
        </div>
      </div>

      {/* 筛选栏 */}
      <div className="pb-2 flex items-center justify-end pr-2">
        <Dropdown ref={dropdownRef}>
          <Dropdown.Item
            key="user"
            title={
              <Users
                size={16}
                className={
                  userFilter.hasSelected ? "text-blue-500" : "text-gray-500"
                }
              />
            }
          >
            <div className="p-2 max-h-64 overflow-y-auto">
              <div
                className={`p-3 rounded ${
                  !userFilter.hasSelected ? "bg-blue-50 text-blue-500" : ""
                }`}
                onClick={() => {
                  userFilter.clear();
                  dropdownRef.current?.close();
                }}
              >
                全部员工
              </div>
              {staffOptions.map((opt) => (
                <div
                  key={opt.value}
                  className={`p-3 rounded ${
                    userFilter.isSelected(opt.value)
                      ? "bg-blue-50 text-blue-500"
                      : ""
                  }`}
                  onClick={() => {
                    userFilter.setSelected([opt.value]);
                    dropdownRef.current?.close();
                  }}
                >
                  {opt.label}
                </div>
              ))}
            </div>
          </Dropdown.Item>
          <Dropdown.Item
            key="date"
            title={
              <Calendar
                size={16}
                className={hasDateFilter ? "text-blue-500" : "text-gray-500"}
              />
            }
            onClick={() => setCalendarVisible(true)}
          />
        </Dropdown>
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
          searchEmpty={!!hasFilters && !list.length}
          estimateSize={100}
          renderItem={(payroll) => (
            <div
              className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3 cursor-pointer active:bg-gray-50"
              onClick={() =>
                navigate({ to: "/payroll/$id", params: { id: payroll.id } })
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
                  <span className="font-medium truncate">
                    {staffMap[payroll.userId] || "未知员工"}
                  </span>
                  <span className="text-lg font-medium text-orange-500">
                    ¥{payroll.amount}
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
  );
}
