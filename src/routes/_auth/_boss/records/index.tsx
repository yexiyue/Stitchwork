import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Dialog, Toast, Dropdown, SwipeAction, Image } from "antd-mobile";
import {
  Filter,
  Plus,
  ImageIcon,
  BarChart3,
  Calendar,
  Users,
  Package,
} from "lucide-react";
import type { PieceRecord, Staff, Order } from "@/types";
import {
  RelativeTime,
  VirtualList,
  StatusTag,
  DateRangeButton,
} from "@/components";
import { pieceRecordApi, orderApi } from "@/api";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import {
  useStaffList,
  useInfiniteList,
  useToggleFilter,
  useDateRange,
} from "@/hooks";
import { RECORD_STATUS_OPTIONS } from "@/constants";

export const Route = createFileRoute("/_auth/_boss/records/")({
  component: RecordsPage,
  validateSearch: (search: Record<string, unknown>): { status?: string } => ({
    status: (search.status as string) || undefined,
  }),
});

function RecordsPage() {
  const navigate = useNavigate();
  const { status: initialStatus } = Route.useSearch();
  const dropdownRef = useRef<DropdownRef>(null);
  const queryClient = useQueryClient();

  // 筛选状态
  const statusFilter = useToggleFilter<string>(
    initialStatus ? [initialStatus] : []
  );
  const userFilter = useToggleFilter<string>();
  const orderFilter = useToggleFilter<string>();

  // 日期筛选
  const {
    startDate,
    endDate,
    dateParams,
    calendarVisible,
    setCalendarVisible,
    handleCalendarConfirm,
    hasDateFilter,
  } = useDateRange({ nullable: true });

  // 员工和订单数据
  const { data: staffList } = useStaffList();
  const { data: ordersData } = useQuery({
    queryKey: ["orders-all"],
    queryFn: () => orderApi.list({ pageSize: 1000 }),
  });

  // 无限列表
  const { list, isFetching, hasMore, loadMore, refresh } =
    useInfiniteList<PieceRecord>(
      [
        "piece-records",
        statusFilter.selected,
        userFilter.selected[0] ?? "",
        orderFilter.selected[0] ?? "",
        startDate?.toISOString(),
        endDate?.toISOString(),
      ],
      (params) =>
        pieceRecordApi.list({
          ...params,
          status: statusFilter.hasSelected ? statusFilter.selected : undefined,
          userId: userFilter.selected[0] || undefined,
          orderId: orderFilter.selected[0] || undefined,
          startDate: dateParams.startDate,
          endDate: dateParams.endDate,
        })
    );

  // 审批操作
  const approveMutation = useMutation({
    mutationFn: pieceRecordApi.approve,
    onSuccess: () => {
      Toast.show({ content: "已通过" });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
    },
  });

  const rejectMutation = useMutation({
    mutationFn: pieceRecordApi.reject,
    onSuccess: () => {
      Toast.show({ content: "已拒绝" });
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
    },
  });

  const handleApprove = (record: PieceRecord) => {
    Dialog.confirm({
      content: "确定通过该计件记录？",
      confirmText: "通过",
      cancelText: "取消",
      onConfirm: () => approveMutation.mutate(record.id),
    });
  };

  const handleReject = (record: PieceRecord) => {
    Dialog.confirm({
      content: "确定拒绝该计件记录？",
      confirmText: "拒绝",
      cancelText: "取消",
      onConfirm: () => rejectMutation.mutate(record.id),
    });
  };

  const staffOptions = (staffList?.list ?? []).map((s: Staff) => ({
    label: s.displayName || s.username,
    value: s.id,
  }));

  const orderOptions = (ordersData?.list ?? []).map((o: Order) => ({
    label: o.productName,
    value: o.id,
  }));

  const hasFilters =
    statusFilter.hasSelected ||
    userFilter.hasSelected ||
    orderFilter.hasSelected ||
    hasDateFilter;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* 顶部栏 */}
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between">
          <h1 className="text-xl">计件管理</h1>
          <div className="flex items-center gap-3">
            <BarChart3
              size={20}
              className="text-gray-500"
              onClick={() => navigate({ to: "/records/stats" })}
            />
            <Plus
              size={20}
              className="text-blue-500"
              onClick={() => navigate({ to: "/records/new" })}
            />
          </div>
        </div>
      </div>

      {/* 筛选栏 */}
      <div className="pb-2 flex items-center justify-end pr-2">
        <Dropdown ref={dropdownRef}>
          <Dropdown.Item
            key="status"
            title={
              <Filter
                size={16}
                className={
                  statusFilter.hasSelected ? "text-blue-500" : "text-gray-500"
                }
              />
            }
          >
            <div className="p-2">
              <div
                className={`p-3 rounded ${
                  !statusFilter.hasSelected ? "bg-blue-50 text-blue-500" : ""
                }`}
                onClick={() => {
                  statusFilter.clear();
                  dropdownRef.current?.close();
                }}
              >
                全部状态
              </div>
              {RECORD_STATUS_OPTIONS.filter((opt) => opt.key).map((opt) => (
                <div
                  key={opt.key}
                  className={`p-3 rounded flex items-center justify-between ${
                    statusFilter.isSelected(opt.key)
                      ? "bg-blue-50 text-blue-500"
                      : ""
                  }`}
                  onClick={() => statusFilter.toggle(opt.key)}
                >
                  {opt.title}
                  {statusFilter.isSelected(opt.key) && <span>✓</span>}
                </div>
              ))}
            </div>
          </Dropdown.Item>
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
            key="order"
            title={
              <Package
                size={16}
                className={
                  orderFilter.hasSelected ? "text-blue-500" : "text-gray-500"
                }
              />
            }
          >
            <div className="p-2 max-h-64 overflow-y-auto">
              <div
                className={`p-3 rounded ${
                  !orderFilter.hasSelected ? "bg-blue-50 text-blue-500" : ""
                }`}
                onClick={() => {
                  orderFilter.clear();
                  dropdownRef.current?.close();
                }}
              >
                全部订单
              </div>
              {orderOptions.map((opt) => (
                <div
                  key={opt.value}
                  className={`p-3 rounded ${
                    orderFilter.isSelected(opt.value)
                      ? "bg-blue-50 text-blue-500"
                      : ""
                  }`}
                  onClick={() => {
                    orderFilter.setSelected([opt.value]);
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
          emptyText="暂无计件记录"
          searchEmpty={!!hasFilters && !list.length}
          estimateSize={114}
          renderItem={(record) => (
            <SwipeAction
              rightActions={
                record.status === "pending"
                  ? [
                      {
                        key: "approve",
                        text: "通过",
                        color: "primary",
                        onClick: () => handleApprove(record),
                      },
                      {
                        key: "reject",
                        text: "拒绝",
                        color: "danger",
                        onClick: () => handleReject(record),
                      },
                    ]
                  : []
              }
            >
              <div
                className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3 cursor-pointer active:bg-gray-50"
                onClick={() =>
                  navigate({ to: "/records/$id", params: { id: record.id } })
                }
              >
                <div className="shrink-0 h-20 aspect-square rounded-lg overflow-hidden bg-gray-100 flex items-center justify-center">
                  {record.orderImage ? (
                    <Image
                      src={record.orderImage}
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
                      {record.userName || "未知员工"}
                    </span>
                    <StatusTag status={record.status} type="record" />
                  </div>
                  <div className="text-sm text-gray-600 truncate">
                    {record.orderName && <span>{record.orderName} · </span>}
                    {record.processName || `工序ID: ${record.processId}`}
                  </div>
                  <div className="text-sm text-gray-500">
                    数量: {record.quantity} · 金额: ¥{record.amount}
                  </div>
                  <div className="text-xs text-gray-400 mt-0.5">
                    <RelativeTime date={record.recordedAt} />
                    {record.recordedBy === "byBoss" && " · 老板代录"}
                  </div>
                </div>
              </div>
            </SwipeAction>
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
