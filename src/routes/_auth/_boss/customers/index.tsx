import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  List,
  Dialog,
  SwipeAction,
  Toast,
  SearchBar,
  NavBar,
} from "antd-mobile";
import { Plus, Search, X, ChevronLeft } from "lucide-react";
import {
  useDeleteCustomer,
  useInfiniteList,
  useDebouncedSearch,
} from "@/hooks";
import type { Customer } from "@/types";
import {
  Avatar,
  RelativeTime,
  VirtualList,
  BiometricGuard,
} from "@/components";
import { customerApi } from "@/api";
import dayjs from "dayjs";
import { useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { motion, AnimatePresence } from "motion/react";

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
    <BiometricGuard
      reason="查看客户信息需要验证身份"
      onCancel={() => navigate({ to: "/profile" })}
    >
      <div className="flex flex-col h-full overflow-hidden">
        <NavBar
          onBack={() => navigate({ to: "/profile" })}
          backIcon={<ChevronLeft size={24} />}
          right={
            <AnimatePresence mode="wait">
              {showSearch ? (
                <motion.div
                  key="close"
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                >
                  <X
                    size={20}
                    className="text-gray-500"
                    onClick={() => {
                      setShowSearch(false);
                      setSearch("");
                    }}
                  />
                </motion.div>
              ) : (
                <motion.div
                  key="actions"
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="flex justify-end gap-3"
                >
                  <Search
                    size={20}
                    className="text-gray-500"
                    onClick={() => setShowSearch(true)}
                  />
                  <Plus
                    size={20}
                    className="text-blue-500"
                    onClick={() => navigate({ to: "/customers/new" })}
                  />
                </motion.div>
              )}
            </AnimatePresence>
          }
        >
          <AnimatePresence mode="wait">
            {showSearch ? (
              <motion.div
                key="search"
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 20 }}
                transition={{ duration: 0.15 }}
              >
                <SearchBar
                  ref={searchInputRef}
                  placeholder="搜索客户名称"
                  value={search}
                  onChange={setSearch}
                />
              </motion.div>
            ) : (
              <motion.div
                key="title"
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -20 }}
                transition={{ duration: 0.15 }}
              >
                客户管理
              </motion.div>
            )}
          </AnimatePresence>
        </NavBar>

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
                  prefix={
                    <div className="flex h-full items-center mr-2">
                      <Avatar name={customer.name} size="md" />
                    </div>
                  }
                  description={customer.phone || undefined}
                  extra={
                    <span className="text-xs text-gray-400">
                      <RelativeTime date={customer.createdAt} />
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
    </BiometricGuard>
  );
}
