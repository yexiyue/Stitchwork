import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, Dialog, SwipeAction, Toast, SearchBar } from "antd-mobile";
import { Plus, Search } from "lucide-react";
import {
  useDeleteCustomer,
  useInfiniteList,
  useDebouncedSearch,
} from "@/hooks";
import type { Customer } from "@/types";
import { Avatar, RelativeTime, VirtualList, PageHeader } from "@/components";
import { customerApi } from "@/api";
import dayjs from "dayjs";
import { useQueryClient } from "@tanstack/react-query";
import { useState } from "react";

export const Route = createFileRoute("/_auth/_boss/customers/")({
  component: CustomersPage,
});

function showCustomerDetail(customer: Customer) {
  Dialog.alert({
    title: customer.name,
    content: (
      <div className="text-left space-y-2">
        <p>
          <span className="text-gray-500">电话：</span>
          {customer.phone || "未填写"}
        </p>
        <p>
          <span className="text-gray-500">备注：</span>
          {customer.description || "无"}
        </p>
        <p>
          <span className="text-gray-500">添加时间：</span>
          {dayjs(customer.createdAt).format("YYYY-MM-DD HH:mm")}
        </p>
      </div>
    ),
    confirmText: "关闭",
  });
}

function CustomersPage() {
  const navigate = useNavigate();
  const deleteMutation = useDeleteCustomer();
  const queryClient = useQueryClient();
  const [showSearch, setShowSearch] = useState(false);

  // 搜索
  const { search, debouncedSearch, setSearch, searchInputRef } =
    useDebouncedSearch();

  // 无限列表
  const { list, isFetching, hasMore, loadMore, refresh } =
    useInfiniteList<Customer>(["customers", debouncedSearch], (params) =>
      customerApi.list({
        ...params,
        search: debouncedSearch || undefined,
      })
    );

  const handleDelete = (customer: Customer) => {
    Dialog.confirm({
      content: `确定删除客户「${customer.name}」？`,
      confirmText: "删除",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await deleteMutation.mutateAsync(customer.id);
          Toast.show({ content: "删除成功" });
          queryClient.invalidateQueries({ queryKey: ["customers"] });
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
      <PageHeader
        title="客户管理"
        right={
          <div className="flex items-center gap-2">
            <Search
              size={20}
              className="text-gray-500"
              onClick={() => setShowSearch(!showSearch)}
            />
            <Plus
              size={20}
              className="text-blue-500"
              onClick={() => navigate({ to: "/customers/new" })}
            />
          </div>
        }
      />

      {showSearch && (
        <div className="px-4 pb-2">
          <SearchBar
            ref={searchInputRef}
            placeholder="搜索客户名称"
            value={search}
            onChange={setSearch}
          />
        </div>
      )}

      <div className="flex flex-1 overflow-hidden">
        <VirtualList
          data={list}
          loading={isFetching}
          hasMore={hasMore}
          onLoadMore={loadMore}
          onRefresh={refresh}
          keyExtractor={(c) => c.id}
          emptyText="暂无客户"
          searchEmpty={!!debouncedSearch && !list.length}
          estimateSize={72}
          renderItem={(customer) => (
            <SwipeAction
              rightActions={[
                {
                  key: "edit",
                  text: "编辑",
                  color: "primary",
                  onClick: () =>
                    navigate({
                      to: "/customers/$id",
                      params: { id: customer.id },
                    }),
                },
                {
                  key: "delete",
                  text: "删除",
                  color: "danger",
                  onClick: () => handleDelete(customer),
                },
              ]}
            >
              <List.Item
                prefix={<Avatar name={customer.name} size="md" />}
                description={
                  <span className="text-gray-400 text-xs">
                    <RelativeTime date={customer.createdAt} />
                    {customer.phone && ` · ${customer.phone}`}
                  </span>
                }
                onClick={() => showCustomerDetail(customer)}
                arrow={false}
              >
                {customer.name}
              </List.Item>
            </SwipeAction>
          )}
        />
      </div>
    </div>
  );
}
