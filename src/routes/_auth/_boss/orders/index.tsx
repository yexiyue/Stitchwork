import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Button,
  Dialog,
  SwipeAction,
  Toast,
  SearchBar,
  Tag,
  Dropdown,
  Image,
  ImageViewer,
  CalendarPicker,
  Divider,
} from "antd-mobile";
import { Plus, ImageOff, Calendar, Filter, Users } from "lucide-react";
import { useDeleteOrder, useCustomers } from "@/hooks";
import type { Order, OrderStatus } from "@/types";
import { RelativeTime, VirtualList } from "@/components";
import { orderApi } from "@/api";
import { useInfiniteQuery, useQueryClient } from "@tanstack/react-query";
import { useDebounceFn } from "ahooks";
import { useState, useRef } from "react";
import type { SearchBarRef } from "antd-mobile/es/components/search-bar";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import dayjs from "dayjs";

export const Route = createFileRoute("/_auth/_boss/orders/")({
  component: OrdersPage,
});

const PAGE_SIZE = 20;

const STATUS_MAP: Record<OrderStatus, { label: string; color: string }> = {
  pending: { label: "待处理", color: "#faad14" },
  processing: { label: "进行中", color: "#1890ff" },
  completed: { label: "已完成", color: "#52c41a" },
  delivered: { label: "已交付", color: "#722ed1" },
  cancelled: { label: "已取消", color: "#999" },
};

const STATUS_OPTIONS = [
  { key: "", title: "全部状态" },
  { key: "pending", title: "待处理" },
  { key: "processing", title: "进行中" },
  { key: "completed", title: "已完成" },
  { key: "delivered", title: "已交付" },
  { key: "cancelled", title: "已取消" },
];

function OrdersPage() {
  const navigate = useNavigate();
  const deleteMutation = useDeleteOrder();
  const [search, setSearch] = useState("");
  const [debouncedSearch, setDebouncedSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string[]>([]);
  const [customerFilter, setCustomerFilter] = useState("");
  const [startDate, setStartDate] = useState<Date | null>(null);
  const [endDate, setEndDate] = useState<Date | null>(null);
  const [calendarVisible, setCalendarVisible] = useState(false);
  const searchInputRef = useRef<SearchBarRef>(null);
  const dropdownRef = useRef<DropdownRef>(null);

  const { data: customersData } = useCustomers({ pageSize: 100 });

  const { run: updateSearch } = useDebounceFn(
    (val: string) => setDebouncedSearch(val),
    { wait: 300 }
  );

  const queryClient = useQueryClient();
  const { data, isFetching, fetchNextPage, hasNextPage, refetch } =
    useInfiniteQuery({
      queryKey: [
        "orders",
        debouncedSearch,
        statusFilter,
        customerFilter,
        startDate?.toISOString(),
        endDate?.toISOString(),
      ],
      queryFn: ({ pageParam = 1 }) =>
        orderApi.list({
          page: pageParam,
          pageSize: PAGE_SIZE,
          search: debouncedSearch,
          status: statusFilter.length ? statusFilter : undefined,
          customerId: customerFilter || undefined,
          startDate: startDate
            ? dayjs(startDate).format("YYYY-MM-DD")
            : undefined,
          endDate: endDate ? dayjs(endDate).format("YYYY-MM-DD") : undefined,
        }),
      initialPageParam: 1,
      getNextPageParam: (lastPage, allPages) =>
        allPages.flatMap((p) => p.list).length < lastPage.total
          ? allPages.length + 1
          : undefined,
    });

  const list = data?.pages.flatMap((p) => p.list) ?? [];

  const handleDelete = (order: Order) => {
    Dialog.confirm({
      content: `确定删除订单「${order.productName}」？`,
      confirmText: "删除",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await deleteMutation.mutateAsync(order.id);
          Toast.show({ content: "删除成功" });
          queryClient.invalidateQueries({ queryKey: ["orders"] });
        } catch (e) {
          Dialog.alert({
            content: e instanceof Error ? e.message : "删除失败",
            confirmText: "确定",
          });
        }
      },
    });
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* 顶部栏：标题 + 新增按钮 */}
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between">
          <h1 className="text-xl">订单管理</h1>
          <Button
            size="small"
            color="primary"
            onClick={() => navigate({ to: "/orders/new" })}
          >
            <div className="flex items-center">
              <Plus size={16} className="mr-1" />
              新增
            </div>
          </Button>
        </div>
      </div>

      {/* 筛选栏 */}
      <div className="pb-2 flex items-center">
        <div className="flex-1 ml-4">
          <SearchBar
            ref={searchInputRef}
            placeholder="搜索产品名称"
            value={search}
            onChange={(val) => {
              setSearch(val);
              updateSearch(val);
            }}
          />
        </div>
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
            key="customer"
            title={
              <Users
                size={16}
                className={customerFilter ? "text-blue-500" : "text-gray-500"}
              />
            }
          >
            <div className="p-2 max-h-64 overflow-y-auto">
              <div
                className={`p-3 rounded ${
                  customerFilter === "" ? "bg-blue-50 text-blue-500" : ""
                }`}
                onClick={() => {
                  setCustomerFilter("");
                  dropdownRef.current?.close();
                }}
              >
                全部客户
              </div>
              {customersData?.list.map((c) => (
                <div
                  key={c.id}
                  className={`p-3 rounded ${
                    customerFilter === c.id ? "bg-blue-50 text-blue-500" : ""
                  }`}
                  onClick={() => {
                    setCustomerFilter(c.id);
                    dropdownRef.current?.close();
                  }}
                >
                  {c.name}
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
          keyExtractor={(o) => o.id}
          emptyText="暂无订单"
          searchEmpty={
            !!(
              debouncedSearch ||
              statusFilter ||
              customerFilter ||
              startDate
            ) && !list.length
          }
          estimateSize={108}
          renderItem={(order) => (
            <SwipeAction
              rightActions={[
                {
                  key: "edit",
                  text: "编辑",
                  color: "primary",
                  onClick: () =>
                    navigate({
                      to: "/orders/$id/edit",
                      params: { id: order.id },
                    }),
                },
                {
                  key: "delete",
                  text: "删除",
                  color: "danger",
                  onClick: () => handleDelete(order),
                },
              ]}
            >
              <div
                className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3"
                onClick={() =>
                  navigate({ to: "/orders/$id", params: { id: order.id } })
                }
              >
                {order.images?.length ? (
                  <Image
                    src={order.images[0]}
                    width={72}
                    height={72}
                    fit="cover"
                    className="rounded shrink-0"
                    onClick={(e) => {
                      e.stopPropagation();
                      ImageViewer.Multi.show({ images: order.images! });
                    }}
                  />
                ) : (
                  <div className="w-18 h-18 bg-gray-100 rounded shrink-0 flex items-center justify-center">
                    <ImageOff size={24} className="text-gray-400" />
                  </div>
                )}
                <div className="flex-1 min-w-0 flex flex-col justify-between">
                  <div className="flex justify-between items-start">
                    <span className="font-medium truncate">
                      {order.productName}
                    </span>
                    <Tag
                      color={STATUS_MAP[order.status].color}
                      fill="outline"
                      style={{ "--border-radius": "4px" }}
                    >
                      {STATUS_MAP[order.status].label}
                    </Tag>
                  </div>
                  <div className="text-sm text-gray-500">
                    数量: {order.quantity} · 单价: ¥{order.unitPrice}
                  </div>
                  <div className="text-xs text-gray-400">
                    <RelativeTime date={order.createdAt} />
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
