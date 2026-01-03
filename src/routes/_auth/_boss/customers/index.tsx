import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  List,
  Button,
  Dialog,
  SwipeAction,
  Toast,
  SearchBar,
} from "antd-mobile";
import { Plus, Search } from "lucide-react";
import { useDeleteCustomer } from "@/hooks";
import type { Customer } from "@/types";
import { Avatar, RelativeTime, VirtualList } from "@/components";
import { customerApi } from "@/api";
import dayjs from "dayjs";
import { useInfiniteQuery, useQueryClient } from "@tanstack/react-query";
import { useDebounceFn } from "ahooks";
import { useState, useRef } from "react";
import { motion, AnimatePresence } from "motion/react";
import type { SearchBarRef } from "antd-mobile/es/components/search-bar";

export const Route = createFileRoute("/_auth/_boss/customers/")({
  component: CustomersPage,
});

const PAGE_SIZE = 20;

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
  const [search, setSearch] = useState("");
  const [debouncedSearch, setDebouncedSearch] = useState("");
  const [showSearch, setShowSearch] = useState(false);
  const searchInputRef = useRef<SearchBarRef>(null);

  const { run: updateSearch } = useDebounceFn(
    (val: string) => setDebouncedSearch(val),
    { wait: 300 }
  );

  const queryClient = useQueryClient();
  const { data, isFetching, fetchNextPage, hasNextPage, refetch } =
    useInfiniteQuery({
      queryKey: ["customers", debouncedSearch],
      queryFn: ({ pageParam = 1 }) =>
        customerApi.list({
          page: pageParam,
          pageSize: PAGE_SIZE,
          search: debouncedSearch,
        }),
      initialPageParam: 1,
      getNextPageParam: (lastPage, allPages) =>
        allPages.flatMap((p) => p.list).length < lastPage.total
          ? allPages.length + 1
          : undefined,
    });

  const list = data?.pages.flatMap((p) => p.list) ?? [];

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
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between mb-3">
          <AnimatePresence initial={false}>
            {!showSearch && (
              <motion.h1
                key="title"
                className="text-xl  whitespace-nowrap"
                initial={{ width: 0, opacity: 0 }}
                animate={{ width: "auto", opacity: 1 }}
                exit={{ width: 0, opacity: 0 }}
                transition={{ duration: 0.2 }}
              >
                客户管理
              </motion.h1>
            )}
          </AnimatePresence>
          <div className="flex items-center gap-2 flex-1 justify-end">
            <AnimatePresence initial={false}>
              {showSearch && (
                <motion.div
                  key="search"
                  className="flex-1 overflow-hidden"
                  initial={{ width: 0 }}
                  animate={{ width: "100%" }}
                  exit={{ width: 0 }}
                  transition={{ duration: 0.2 }}
                >
                  <SearchBar
                    ref={searchInputRef}
                    placeholder="搜索客户名称"
                    value={search}
                    onChange={(val) => {
                      setSearch(val);
                      updateSearch(val);
                    }}
                    onCancel={() => {
                      setShowSearch(false);
                      setSearch("");
                      updateSearch("");
                    }}
                    showCancelButton
                    autoFocus
                  />
                </motion.div>
              )}
            </AnimatePresence>
            {!showSearch && (
              <>
                <Button
                  size="small"
                  fill="none"
                  onClick={() => {
                    setShowSearch(true);
                    setTimeout(() => searchInputRef.current?.focus(), 100);
                  }}
                >
                  <Search size={20} />
                </Button>
                <Button
                  size="small"
                  color="primary"
                  onClick={() => navigate({ to: "/customers/new" })}
                >
                  <div className="flex items-center">
                    <Plus size={16} className="mr-1" />
                    新增
                  </div>
                </Button>
              </>
            )}
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
          keyExtractor={(c) => c.id}
          emptyText="暂无客户"
          searchEmpty={!!debouncedSearch && !list.length}
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
                prefix={
                  <div className="flex h-full flex-col justify-center pr-2">
                    <Avatar name={customer.name} />
                  </div>
                }
                description={
                  <div className="flex justify-between items-center">
                    <span className="line-clamp-1">
                      {customer.phone && <span>{customer.phone}</span>}
                      {customer.phone && customer.description && " · "}
                      {customer.description}
                    </span>
                    <span className="text-xs text-gray-400 shrink-0 ml-2">
                      <RelativeTime date={customer.createdAt} />
                    </span>
                  </div>
                }
                onClick={() => showCustomerDetail(customer)}
                clickable
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
