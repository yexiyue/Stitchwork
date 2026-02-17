import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  List,
  Dialog,
  Toast,
  SearchBar,
  SwipeAction,
  NavBar,
  Input,
} from "antd-mobile";
import { Search, UserPlus, X, ChevronLeft } from "lucide-react";
import { Avatar, VirtualList, RelativeTime } from "@/components";
import type { Staff } from "@/types";
import { authApi } from "@/api";
import { useState } from "react";
import { useInfiniteList, useDebouncedSearch } from "@/hooks";
import { motion, AnimatePresence } from "motion/react";

export const Route = createFileRoute("/_auth/_boss/staff/")({
  component: StaffPage,
});

function StaffPage() {
  const navigate = useNavigate();
  const [showSearch, setShowSearch] = useState(false);

  // 搜索
  const { search, debouncedSearch, setSearch, searchInputRef } =
    useDebouncedSearch();

  // 无限列表
  const { list, isFetching, hasMore, loadMore, refresh } = useInfiniteList(
    ["staff", debouncedSearch],
    (params) =>
      authApi.getStaffList({
        ...params,
        search: debouncedSearch || undefined,
      })
  );

  const showStaffDetail = (staff: Staff) => {
    Dialog.alert({
      content: (
        <div className="py-2">
          <div className="flex flex-col items-center mb-4">
            <Avatar
              name={staff.displayName || staff.username}
              src={staff.avatar}
              size="lg"
            />
            <div className="mt-2 text-lg font-medium">
              {staff.displayName || staff.username}
            </div>
            {staff.displayName && (
              <div className="text-sm text-gray-400">@{staff.username}</div>
            )}
          </div>
          <div className="space-y-2">
            {staff.phone && (
              <div className="flex justify-between">
                <span className="text-gray-500">手机号</span>
                <span>{staff.phone}</span>
              </div>
            )}
            {staff.createdAt && (
              <div className="flex justify-between">
                <span className="text-gray-500">加入时间</span>
                <span>
                  <RelativeTime date={staff.createdAt} />
                </span>
              </div>
            )}
          </div>
        </div>
      ),
      confirmText: "关闭",
    });
  };

  const handleResetPassword = (staffId: string, staffName: string) => {
    let newPassword = "";
    Dialog.confirm({
      title: `重置「${staffName}」的密码`,
      content: (
        <div className="mt-2">
          <Input
            placeholder="请输入新密码"
            type="password"
            onChange={(v) => (newPassword = v)}
          />
        </div>
      ),
      confirmText: "确定",
      cancelText: "取消",
      onConfirm: async () => {
        if (!newPassword.trim()) {
          Toast.show({ content: "请输入新密码" });
          return;
        }
        try {
          await authApi.resetStaffPassword(staffId, newPassword);
          Toast.show({ content: "密码重置成功" });
        } catch (e) {
          Toast.show({
            content: e instanceof Error ? e.message : "重置失败",
          });
        }
      },
    });
  };

  const handleRemoveStaff = (staffId: string, staffName: string) => {
    Dialog.confirm({
      content: `确定移除员工「${staffName}」？`,
      confirmText: "移除",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await authApi.removeStaff(staffId);
          Toast.show({ content: "移除成功" });
          refresh();
        } catch (e) {
          Toast.show({
            content: e instanceof Error ? e.message : "移除失败",
          });
        }
      },
    });
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* 顶部栏 */}
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
                <UserPlus
                  size={20}
                  className="text-blue-500"
                  onClick={() => navigate({ to: "/register-codes" })}
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
                placeholder="搜索员工姓名"
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
              员工管理
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
          keyExtractor={(s) => s.id}
          emptyText="暂无员工"
          searchEmpty={!!debouncedSearch && !list.length}
          estimateSize={54}
          renderItem={(staff) => (
            <SwipeAction
              rightActions={[
                {
                  key: "reset-password",
                  text: "重置密码",
                  color: "primary",
                  onClick: () =>
                    handleResetPassword(
                      staff.id,
                      staff.displayName || staff.username
                    ),
                },
                {
                  key: "remove",
                  text: "移除",
                  color: "danger",
                  onClick: () =>
                    handleRemoveStaff(
                      staff.id,
                      staff.displayName || staff.username
                    ),
                },
              ]}
            >
              <List.Item
                className="border-b border-gray-100"
                onClick={() => showStaffDetail(staff)}
                prefix={
                  <div className="flex h-full items-center mr-2">
                    <Avatar
                      name={staff.displayName || staff.username}
                      src={staff.avatar}
                      size="md"
                    />
                  </div>
                }
                description={staff.username}
                extra={
                  staff.createdAt && (
                    <span className="text-xs text-gray-400">
                      <RelativeTime date={staff.createdAt} />
                    </span>
                  )
                }
              >
                <span className="text-base">
                  {staff.displayName || staff.username}
                </span>
              </List.Item>
            </SwipeAction>
          )}
        />
      </div>
    </div>
  );
}
