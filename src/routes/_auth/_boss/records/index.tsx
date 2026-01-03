import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Dialog, Toast, Tag, Dropdown, SwipeAction, Button } from "antd-mobile";
import { Filter, Plus } from "lucide-react";
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
  const [statusFilter, setStatusFilter] = useState("");
  const [userFilter, setUserFilter] = useState("");
  const [orderFilter, setOrderFilter] = useState("");
  const dropdownRef = useRef<DropdownRef>(null);
  const queryClient = useQueryClient();

  const { data: staffList } = useStaffList();
  const { data: ordersData } = useQuery({
    queryKey: ["orders-all"],
    queryFn: () => orderApi.list({ pageSize: 1000 }),
  });

  const { data, isFetching, fetchNextPage, hasNextPage, refetch } =
    useInfiniteQuery({
      queryKey: ["piece-records", statusFilter, userFilter, orderFilter],
      queryFn: ({ pageParam = 1 }) =>
        pieceRecordApi.list({
          page: pageParam,
          pageSize: PAGE_SIZE,
          status: statusFilter || undefined,
          userId: userFilter || undefined,
          orderId: orderFilter || undefined,
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

  const hasFilters = statusFilter || userFilter || orderFilter;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between mb-3">
          <h1 className="text-xl">计件管理</h1>
          <div className="flex items-center gap-2">
            <div
              className="flex items-center text-blue-500"
              onClick={() => navigate({ to: "/records/new" })}
            >
              <Plus size={20} />
              <span>新增</span>
            </div>
            <Dropdown ref={dropdownRef}>
              <Dropdown.Item key="filter" title={<Filter size={20} />}>
                <div className="p-2">
                  <div className="text-gray-500 text-sm mb-2">状态</div>
                  {STATUS_OPTIONS.map((opt) => (
                    <div
                      key={opt.key}
                      className={`p-3 rounded ${
                        statusFilter === opt.key
                          ? "bg-blue-50 text-blue-500"
                          : ""
                      }`}
                      onClick={() => setStatusFilter(opt.key)}
                    >
                      {opt.title}
                    </div>
                  ))}
                  <div className="text-gray-500 text-sm mb-2 mt-3">员工</div>
                  <div
                    className={`p-3 rounded ${
                      !userFilter ? "bg-blue-50 text-blue-500" : ""
                    }`}
                    onClick={() => setUserFilter("")}
                  >
                    全部员工
                  </div>
                  {staffOptions.map((opt) => (
                    <div
                      key={opt.value}
                      className={`p-3 rounded ${
                        userFilter === opt.value
                          ? "bg-blue-50 text-blue-500"
                          : ""
                      }`}
                      onClick={() => setUserFilter(opt.value)}
                    >
                      {opt.label}
                    </div>
                  ))}
                  <div className="text-gray-500 text-sm mb-2 mt-3">订单</div>
                  <div
                    className={`p-3 rounded ${
                      !orderFilter ? "bg-blue-50 text-blue-500" : ""
                    }`}
                    onClick={() => setOrderFilter("")}
                  >
                    全部订单
                  </div>
                  {orderOptions.map((opt) => (
                    <div
                      key={opt.value}
                      className={`p-3 rounded ${
                        orderFilter === opt.value
                          ? "bg-blue-50 text-blue-500"
                          : ""
                      }`}
                      onClick={() => setOrderFilter(opt.value)}
                    >
                      {opt.label}
                    </div>
                  ))}
                  <Button
                    block
                    color="primary"
                    className="mt-3"
                    onClick={() => dropdownRef.current?.close()}
                  >
                    确定
                  </Button>
                </div>
              </Dropdown.Item>
            </Dropdown>
          </div>
        </div>
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
          estimateSize={100}
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
              <div className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm">
                <div className="flex justify-between items-start mb-2">
                  <span className="font-medium">
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
                <div className="text-sm text-gray-600 mb-1">
                  {record.orderName && <span>{record.orderName} · </span>}
                  {record.processName || `工序ID: ${record.processId}`}
                </div>
                <div className="text-sm text-gray-500">
                  数量: {record.quantity} · 金额: ¥{record.amount}
                </div>
                <div className="text-xs text-gray-400 mt-1">
                  <RelativeTime date={record.recordedAt} />
                  {record.recordedBy === "boss" && " · 老板代录"}
                </div>
              </div>
            </SwipeAction>
          )}
        />
      </div>
    </div>
  );
}
