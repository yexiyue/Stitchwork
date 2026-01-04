import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Dialog, Toast, Tag, Dropdown, SwipeAction, Image, CalendarPicker } from "antd-mobile";
import { Filter, Plus, ImageIcon, BarChart2, Calendar, Users, Package } from "lucide-react";
import type { PieceRecord, PieceRecordStatus, Staff, Order } from "@/types";
import { RelativeTime, VirtualList } from "@/components";
import { pieceRecordApi, orderApi } from "@/api";
import {
  useInfiniteQuery,
  useQueryClient,
  useMutation,
  useQuery,
} from "@tanstack/react-query";
import { useState, useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import { useStaffList } from "@/hooks";
import dayjs from "dayjs";

export const Route = createFileRoute("/_auth/_boss/records/")({
  component: RecordsPage,
});

const PAGE_SIZE = 20;

const STATUS_MAP: Record<PieceRecordStatus, { label: string; color: string }> =
  {
    pending: { label: "待审核", color: "#faad14" },
    approved: { label: "已通过", color: "#52c41a" },
    rejected: { label: "已拒绝", color: "#ff4d4f" },
  };

const STATUS_OPTIONS = [
  { key: "", title: "全部状态" },
  { key: "pending", title: "待审核" },
  { key: "approved", title: "已通过" },
  { key: "rejected", title: "已拒绝" },
];

function RecordsPage() {
  const navigate = useNavigate();
  const [statusFilter, setStatusFilter] = useState<string[]>([]);
  const [userFilter, setUserFilter] = useState("");
  const [orderFilter, setOrderFilter] = useState("");
  const [startDate, setStartDate] = useState<Date | null>(null);
  const [endDate, setEndDate] = useState<Date | null>(null);
  const [calendarVisible, setCalendarVisible] = useState(false);
  const dropdownRef = useRef<DropdownRef>(null);
  const queryClient = useQueryClient();

  const { data: staffList } = useStaffList();
  const { data: ordersData } = useQuery({
    queryKey: ["orders-all"],
    queryFn: () => orderApi.list({ pageSize: 1000 }),
  });

  const { data, isFetching, fetchNextPage, hasNextPage, refetch } =
    useInfiniteQuery({
      queryKey: ["piece-records", statusFilter, userFilter, orderFilter, startDate?.toISOString(), endDate?.toISOString()],
      queryFn: ({ pageParam = 1 }) =>
        pieceRecordApi.list({
          page: pageParam,
          pageSize: PAGE_SIZE,
          status: statusFilter.length ? statusFilter : undefined,
          userId: userFilter || undefined,
          orderId: orderFilter || undefined,
          startDate: startDate ? dayjs(startDate).format("YYYY-MM-DD") : undefined,
          endDate: endDate ? dayjs(endDate).format("YYYY-MM-DD") : undefined,
        }),
      initialPageParam: 1,
      getNextPageParam: (lastPage, allPages) =>
        allPages.flatMap((p) => p.list).length < lastPage.total
          ? allPages.length + 1
          : undefined,
    });

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

  const list = data?.pages.flatMap((p) => p.list) ?? [];

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

  const hasFilters = statusFilter || userFilter || orderFilter || startDate;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* 顶部栏：标题 + 操作按钮 */}
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between">
          <h1 className="text-xl">计件管理</h1>
          <div className="flex items-center gap-3">
            <div
              className="flex items-center text-blue-500"
              onClick={() => navigate({ to: "/records/stats" })}
            >
              <BarChart2 size={20} />
            </div>
            <div
              className="flex items-center text-blue-500"
              onClick={() => navigate({ to: "/records/new" })}
            >
              <Plus size={20} />
            </div>
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
                className={statusFilter.length ? "text-blue-500" : "text-gray-500"}
              />
            }
          >
            <div className="p-2">
              <div
                className={`p-3 rounded ${!statusFilter.length ? "bg-blue-50 text-blue-500" : ""}`}
                onClick={() => {
                  setStatusFilter([]);
                  dropdownRef.current?.close();
                }}
              >
                全部状态
              </div>
              {STATUS_OPTIONS.filter(opt => opt.key).map((opt) => (
                <div
                  key={opt.key}
                  className={`p-3 rounded flex items-center justify-between ${
                    statusFilter.includes(opt.key) ? "bg-blue-50 text-blue-500" : ""
                  }`}
                  onClick={() => {
                    setStatusFilter(prev =>
                      prev.includes(opt.key)
                        ? prev.filter(s => s !== opt.key)
                        : [...prev, opt.key]
                    );
                  }}
                >
                  {opt.title}
                  {statusFilter.includes(opt.key) && <span>✓</span>}
                </div>
              ))}
            </div>
          </Dropdown.Item>
          <Dropdown.Item
            key="user"
            title={
              <Users
                size={16}
                className={userFilter ? "text-blue-500" : "text-gray-500"}
              />
            }
          >
            <div className="p-2 max-h-64 overflow-y-auto">
              <div
                className={`p-3 rounded ${
                  !userFilter ? "bg-blue-50 text-blue-500" : ""
                }`}
                onClick={() => {
                  setUserFilter("");
                  dropdownRef.current?.close();
                }}
              >
                全部员工
              </div>
              {staffOptions.map((opt) => (
                <div
                  key={opt.value}
                  className={`p-3 rounded ${
                    userFilter === opt.value ? "bg-blue-50 text-blue-500" : ""
                  }`}
                  onClick={() => {
                    setUserFilter(opt.value);
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
                className={orderFilter ? "text-blue-500" : "text-gray-500"}
              />
            }
          >
            <div className="p-2 max-h-64 overflow-y-auto">
              <div
                className={`p-3 rounded ${
                  !orderFilter ? "bg-blue-50 text-blue-500" : ""
                }`}
                onClick={() => {
                  setOrderFilter("");
                  dropdownRef.current?.close();
                }}
              >
                全部订单
              </div>
              {orderOptions.map((opt) => (
                <div
                  key={opt.value}
                  className={`p-3 rounded ${
                    orderFilter === opt.value ? "bg-blue-50 text-blue-500" : ""
                  }`}
                  onClick={() => {
                    setOrderFilter(opt.value);
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
                className={startDate ? "text-blue-500" : "text-gray-500"}
              />
            }
            onClick={() => setCalendarVisible(true)}
          />
        </Dropdown>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <VirtualList
          data={list}
          loading={isFetching}
          hasMore={!!hasNextPage}
          onLoadMore={fetchNextPage}
          onRefresh={refetch}
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
                onClick={() => navigate({ to: "/records/$id", params: { id: record.id } })}
              >
                {/* 订单图片 */}
                <div className="shrink-0 w-14 h-14 rounded-lg overflow-hidden bg-gray-100 flex items-center justify-center">
                  {record.orderImage ? (
                    <Image
                      src={record.orderImage}
                      width={56}
                      height={56}
                      fit="cover"
                    />
                  ) : (
                    <ImageIcon size={24} className="text-gray-400" />
                  )}
                </div>
                {/* 信息 */}
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between items-start mb-1">
                    <span className="font-medium truncate">
                      {record.userName || "未知员工"}
                    </span>
                    <Tag
                      color={STATUS_MAP[record.status].color}
                      fill="outline"
                      style={{ "--border-radius": "4px" }}
                    >
                      {STATUS_MAP[record.status].label}
                    </Tag>
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
                    {record.recordedBy === "boss" && " · 老板代录"}
                  </div>
                </div>
              </div>
            </SwipeAction>
          )}
        />
      </div>

      {/* 日期范围选择器 */}
      <CalendarPicker
        visible={calendarVisible}
        selectionMode="range"
        onClose={() => {
          setCalendarVisible(false);
          dropdownRef.current?.close();
        }}
        onConfirm={(val) => {
          if (val && val.length === 2) {
            setStartDate(val[0]);
            setEndDate(val[1]);
          } else {
            setStartDate(null);
            setEndDate(null);
          }
          setCalendarVisible(false);
          dropdownRef.current?.close();
        }}
        title="选择日期范围"
      />
    </div>
  );
}
