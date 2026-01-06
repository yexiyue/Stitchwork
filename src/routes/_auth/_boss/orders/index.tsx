import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  Dialog,
  SwipeAction,
  Toast,
  SearchBar,
  Dropdown,
  Image,
  ImageViewer,
} from "antd-mobile";
import { Plus, ImageOff, Calendar, Filter, Users, BarChart3 } from "lucide-react";
import { useDeleteOrder, useCustomers, useInfiniteList, useDebouncedSearch, useToggleFilter, useDateRange } from "@/hooks";
import type { Order } from "@/types";
import { RelativeTime, VirtualList, StatusTag, DateRangeButton } from "@/components";
import { orderApi } from "@/api";
import { useQueryClient } from "@tanstack/react-query";
import { useRef } from "react";
import type { DropdownRef } from "antd-mobile/es/components/dropdown";
import { ORDER_STATUS_OPTIONS } from "@/constants";

export const Route = createFileRoute("/_auth/_boss/orders/")({
  component: OrdersPage,
});

function OrdersPage() {
  const navigate = useNavigate();
  const deleteMutation = useDeleteOrder();
  const queryClient = useQueryClient();
  const dropdownRef = useRef<DropdownRef>(null);

  // 搜索
  const { search, debouncedSearch, setSearch, searchInputRef } = useDebouncedSearch();

  // 状态筛选
  const statusFilter = useToggleFilter<string>();

  // 客户筛选
  const { data: customersData } = useCustomers({ pageSize: 100 });
  const customerFilter = useToggleFilter<string>();

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

  // 无限列表
  const { list, isFetching, hasMore, loadMore, refresh } = useInfiniteList<Order>(
    [
      "orders",
      debouncedSearch,
      statusFilter.selected,
      customerFilter.selected[0] ?? "",
      startDate?.toISOString(),
      endDate?.toISOString(),
    ],
    (params) =>
      orderApi.list({
        ...params,
        search: debouncedSearch || undefined,
        status: statusFilter.hasSelected ? statusFilter.selected : undefined,
        customerId: customerFilter.selected[0] || undefined,
        startDate: dateParams.startDate,
        endDate: dateParams.endDate,
      })
  );

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

  const hasFilters = debouncedSearch || statusFilter.hasSelected || customerFilter.hasSelected || hasDateFilter;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* 顶部栏 */}
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between">
          <h1 className="text-xl">订单管理</h1>
          <div className="flex items-center gap-3">
            <BarChart3
              size={20}
              className="text-gray-500"
              onClick={() => navigate({ to: "/orders/stats" })}
            />
            <Plus
              size={20}
              className="text-blue-500"
              onClick={() => navigate({ to: "/orders/new" })}
            />
          </div>
        </div>
      </div>

      {/* 筛选栏 */}
      <div className="pb-2 flex items-center">
        <div className="flex-1 ml-4">
          <SearchBar
            ref={searchInputRef}
            placeholder="搜索产品名称"
            value={search}
            onChange={setSearch}
          />
        </div>
        <Dropdown ref={dropdownRef}>
          <Dropdown.Item
            key="status"
            title={
              <Filter
                size={16}
                className={statusFilter.hasSelected ? "text-blue-500" : "text-gray-500"}
              />
            }
          >
            <div className="p-2">
              <div
                className={`p-3 rounded ${!statusFilter.hasSelected ? "bg-blue-50 text-blue-500" : ""}`}
                onClick={() => {
                  statusFilter.clear();
                  dropdownRef.current?.close();
                }}
              >
                全部状态
              </div>
              {ORDER_STATUS_OPTIONS.filter((opt) => opt.key).map((opt) => (
                <div
                  key={opt.key}
                  className={`p-3 rounded flex items-center justify-between ${
                    statusFilter.isSelected(opt.key) ? "bg-blue-50 text-blue-500" : ""
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
            key="customer"
            title={
              <Users
                size={16}
                className={customerFilter.hasSelected ? "text-blue-500" : "text-gray-500"}
              />
            }
          >
            <div className="p-2 max-h-64 overflow-y-auto">
              <div
                className={`p-3 rounded ${!customerFilter.hasSelected ? "bg-blue-50 text-blue-500" : ""}`}
                onClick={() => {
                  customerFilter.clear();
                  dropdownRef.current?.close();
                }}
              >
                全部客户
              </div>
              {customersData?.list.map((c) => (
                <div
                  key={c.id}
                  className={`p-3 rounded ${
                    customerFilter.isSelected(c.id) ? "bg-blue-50 text-blue-500" : ""
                  }`}
                  onClick={() => {
                    customerFilter.setSelected([c.id]);
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
          keyExtractor={(o) => o.id}
          emptyText="暂无订单"
          searchEmpty={!!hasFilters && !list.length}
          estimateSize={108}
          renderItem={(order) => (
            <SwipeAction
              rightActions={[
                {
                  key: "edit",
                  text: "编辑",
                  color: "primary",
                  onClick: () =>
                    navigate({ to: "/orders/$id/edit", params: { id: order.id } }),
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
                onClick={() => navigate({ to: "/orders/$id", params: { id: order.id } })}
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
                    <span className="font-medium truncate">{order.productName}</span>
                    <StatusTag status={order.status} type="order" />
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
