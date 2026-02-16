import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  List,
  Dialog,
  Toast,
  SearchBar,
  SwipeAction,
  NavBar,
  Button,
} from "antd-mobile";
import { Search, UserPlus, X, ChevronLeft, Copy } from "lucide-react";
import { Avatar, VirtualList, RelativeTime } from "@/components";
import { authApi } from "@/api";
import { useState } from "react";
import { copyToClipboard } from "@/utils/clipboard";
import { QRCodeSVG } from "qrcode.react";
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

  const handleInvite = async () => {
    try {
      const data = await authApi.generateInviteCode();
      // 使用 custom scheme deep link，扫码直接打开应用
      const url = `stitchwork://register?code=${data.code}`;
      Dialog.alert({
        title: "邀请员工",
        content: (
          <div className="flex flex-col items-center py-4">
            <QRCodeSVG value={url} size={180} />
            <p className="mt-3 text-lg font-mono font-bold">{data.code}</p>
            <Button
              size="small"
              className="mt-2"
              onClick={() => {
                copyToClipboard(data.code);
                Toast.show({ content: "已复制" });
              }}
            >
              <div className="flex items-center justify-center">
                <Copy size={16} className="mr-1" />
                复制
              </div>
            </Button>
            <p className="mt-2 text-xs text-gray-400">
              有效期至: {new Date(data.expiresAt).toLocaleString()}
            </p>
          </div>
        ),
        confirmText: "关闭",
      });
    } catch (e) {
      Toast.show({
        content: e instanceof Error ? e.message : "生成邀请码失败",
      });
    }
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
                  onClick={handleInvite}
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
