import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Dropdown, Button } from "antd-mobile";
import { ChevronRight, Filter, Plus, BarChart3, ImageIcon } from "lucide-react";
import type { PieceRecord } from "@/types";
import { RelativeTime, VirtualList, StatusTag, OssImage } from "@/components";
import { pieceRecordApi, statsApi } from "@/api";
import { useQuery } from "@tanstack/react-query";
import { useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import dayjs from "dayjs";
import { useInfiniteList, useToggleFilter, useWorkshopSettings } from "@/hooks";
import { RECORD_STATUS_OPTIONS_NO_ALL } from "@/constants";

export const Route = createFileRoute("/_auth/_staff/my-records/")({
  component: StaffRecordsPage,
});

function StaffRecordsPage() {
  const navigate = useNavigate();
  const dropdownRef = useRef<DropdownRef>(null);
  const { pieceUnit } = useWorkshopSettings();

  // 状态筛选
  const statusFilter = useToggleFilter<string>();

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

  // 无限列表
  const { list, isFetching, hasMore, loadMore, refresh } =
    useInfiniteList<PieceRecord>(
      ["piece-records", statusFilter.selected],
      (params) =>
        pieceRecordApi.list({
          ...params,
          status: statusFilter.hasSelected ? statusFilter.selected : undefined,
        })
    );

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
                <div className="text-2xl font-bold">
                  {myStats.totalQuantity}
                </div>
                <div className="text-xs opacity-80">{pieceUnit}</div>
              </div>
              <div className="w-px bg-white/30" />
              <div className="text-center">
                <div className="text-2xl font-bold">
                  ¥{parseFloat(myStats.totalAmount).toFixed(0)}
                </div>
                <div className="text-xs opacity-80">金额</div>
              </div>
            </div>
          </div>
        )}

        <div className="flex items-center justify-between mb-3">
          <h1 className="text-xl">我的计件</h1>
          <div className="flex items-center gap-3">
            <BarChart3
              size={20}
              className="text-gray-500"
              onClick={() => navigate({ to: "/my-records/stats" })}
            />
            <Dropdown ref={dropdownRef}>
              <Dropdown.Item key="filter" title={<Filter size={20} />}>
                <div className="p-2">
                  <div className="text-gray-500 text-sm mb-2">状态</div>
                  {RECORD_STATUS_OPTIONS_NO_ALL.map((opt) => (
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
            <Plus
              size={20}
              className="text-blue-500"
              onClick={() => navigate({ to: "/my-records/new" })}
            />
          </div>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <VirtualList
          data={list}
          loading={isFetching}
          hasMore={hasMore}
          onLoadMore={loadMore}
          onRefresh={refresh}
          keyExtractor={(r) => r.id}
          emptyText="暂无计件记录"
          searchEmpty={statusFilter.hasSelected && !list.length}
          estimateSize={98}
          renderItem={(record: PieceRecord) => (
            <div
              className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3 border border-gray-50"
              onClick={() =>
                navigate({ to: "/my-records/$id", params: { id: record.id } })
              }
            >
              <div className="shrink-0 h-16 aspect-square rounded-lg overflow-hidden bg-gray-100 flex items-center justify-center">
                {record.orderImage ? (
                  <OssImage
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
                    {record.orderName || "未知订单"} -{" "}
                    {record.processName || "未知工序"}
                  </span>
                  <StatusTag status={record.status} type="record" />
                </div>
                <div className="text-sm text-gray-600">
                  数量: {record.quantity} · 金额: ¥{record.amount}
                </div>
                <div className="text-xs text-gray-400 mt-1">
                  <RelativeTime date={record.recordedAt} />
                  {record.recordedBy === "byBoss" && " · 老板代录"}
                </div>
              </div>
            </div>
          )}
        />
      </div>
    </div>
  );
}
