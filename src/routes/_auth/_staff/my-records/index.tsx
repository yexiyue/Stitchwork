import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Tag, Dropdown, Button } from "antd-mobile";
import { ChevronRight, Filter, Plus } from "lucide-react";
import type { PieceRecord, PieceRecordStatus } from "@/types";
import { RelativeTime, VirtualList } from "@/components";
import { pieceRecordApi, statsApi } from "@/api";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { useState, useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import dayjs from "dayjs";

export const Route = createFileRoute("/_auth/_staff/my-records/")({
  component: StaffRecordsPage,
});

const PAGE_SIZE = 20;

const STATUS_MAP: Record<PieceRecordStatus, { label: string; color: string }> =
  {
    pending: { label: "待审核", color: "#faad14" },
    approved: { label: "已通过", color: "#52c41a" },
    rejected: { label: "已拒绝", color: "#ff4d4f" },
  };

const STATUS_OPTIONS = [
  { key: "pending", title: "待审核" },
  { key: "approved", title: "已通过" },
  { key: "rejected", title: "已拒绝" },
];

function StaffRecordsPage() {
  const navigate = useNavigate();
  const [statusFilter, setStatusFilter] = useState<string[]>([]);
  const dropdownRef = useRef<DropdownRef>(null);

  // 本月统计
  const { data: statsData } = useQuery({
    queryKey: [
      "my-stats",
      dayjs().startOf("month").format("YYYY-MM-DD"),
      dayjs().endOf("month").format("YYYY-MM-DD"),
    ],
    queryFn: () =>
      statsApi.workerProduction({
        startDate: dayjs().startOf("month").format("YYYY-MM-DD"),
        endDate: dayjs().endOf("month").format("YYYY-MM-DD"),
      }),
  });

  const myStats = statsData?.list?.[0];

  const { data, isFetching, fetchNextPage, hasNextPage, refetch } =
    useInfiniteQuery({
      queryKey: ["piece-records", statusFilter],
      queryFn: ({ pageParam = 1 }) =>
        pieceRecordApi.list({
          page: pageParam,
          pageSize: PAGE_SIZE,
          status: statusFilter.length ? statusFilter : undefined,
        }),
      initialPageParam: 1,
      getNextPageParam: (lastPage, allPages) =>
        allPages.flatMap((p) => p.list).length < lastPage.total
          ? allPages.length + 1
          : undefined,
    });

  const list = data?.pages.flatMap((p) => p.list) ?? [];

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="p-4 pb-2">
        {/* 本月统计卡片 */}
        {myStats && (
          <div
            className="bg-linear-to-r from-blue-500 to-blue-600 rounded-lg p-4 mb-3 text-white cursor-pointer active:opacity-90"
            onClick={() => navigate({ to: "/my-records/stats" })}
          >
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm opacity-80">本月计件</span>
              <ChevronRight size={18} className="opacity-60" />
            </div>
            <div className="flex justify-around">
              <div className="text-center">
                <div className="text-2xl font-bold">{myStats.totalQuantity}</div>
                <div className="text-xs opacity-80">件</div>
              </div>
              <div className="w-px bg-white/30" />
              <div className="text-center">
                <div className="text-2xl font-bold">¥{parseFloat(myStats.totalAmount).toFixed(0)}</div>
                <div className="text-xs opacity-80">金额</div>
              </div>
            </div>
          </div>
        )}

        <div className="flex items-center justify-between mb-3">
          <h1 className="text-xl">我的计件</h1>
          <div className="flex items-center gap-2">
            <div
              className="flex items-center text-blue-500"
              onClick={() => navigate({ to: "/my-records/new" })}
            >
              <Plus size={20} />
              <span>录入</span>
            </div>
            <Dropdown ref={dropdownRef}>
              <Dropdown.Item key="filter" title={<Filter size={20} />}>
                <div className="p-2">
                  <div className="text-gray-500 text-sm mb-2">状态</div>
                  {STATUS_OPTIONS.map((opt) => (
                    <div
                      key={opt.key}
                      className={`p-3 rounded flex items-center justify-between ${
                        statusFilter.includes(opt.key)
                          ? "bg-blue-50 text-blue-500"
                          : ""
                      }`}
                      onClick={() =>
                        setStatusFilter((prev) =>
                          prev.includes(opt.key)
                            ? prev.filter((s) => s !== opt.key)
                            : [...prev, opt.key]
                        )
                      }
                    >
                      {opt.title}
                      {statusFilter.includes(opt.key) && <span>✓</span>}
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
          searchEmpty={!!statusFilter.length && !list.length}
          estimateSize={98}
          renderItem={(record: PieceRecord) => (
            <div
              className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3"
              onClick={() =>
                navigate({ to: "/my-records/$id", params: { id: record.id } })
              }
            >
              {record.orderImage ? (
                <img
                  src={record.orderImage}
                  alt="订单图片"
                  className="w-14 h-14 rounded object-cover shrink-0"
                />
              ) : (
                <div className="w-14 h-14 rounded bg-gray-100 flex items-center justify-center flex-shrink-0">
                  <span className="text-gray-400 text-xs">无图</span>
                </div>
              )}
              <div className="flex-1 min-w-0">
                <div className="flex justify-between items-start mb-1">
                  <span className="font-medium truncate">
                    {record.orderName || "未知订单"} - {record.processName || "未知工序"}
                  </span>
                  <Tag
                    color={STATUS_MAP[record.status].color}
                    fill="outline"
                    style={{ "--border-radius": "4px" }}
                    className="shrink-0 ml-2"
                  >
                    {STATUS_MAP[record.status].label}
                  </Tag>
                </div>
                <div className="text-sm text-gray-600">
                  数量: {record.quantity} · 金额: ¥{record.amount}
                </div>
                <div className="text-xs text-gray-400 mt-1">
                  <RelativeTime date={record.recordedAt} />
                  {record.recordedBy === "boss" && " · 老板代录"}
                </div>
              </div>
            </div>
          )}
        />
      </div>
    </div>
  );
}
