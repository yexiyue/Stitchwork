import { createFileRoute } from "@tanstack/react-router";
import { List, Dialog, Toast, SearchBar, SwipeAction } from "antd-mobile";
import { Search } from "lucide-react";
import { Avatar, VirtualList, PageHeader } from "@/components";
import { authApi } from "@/api";
import { useState } from "react";
import { QRCodeSVG } from "qrcode.react";
import { useInfiniteList, useDebouncedSearch } from "@/hooks";

export const Route = createFileRoute("/_auth/_boss/staff/")({
  component: StaffPage,
});

function StaffPage() {
  const [showSearch, setShowSearch] = useState(false);

  // 搜索
  const { search, debouncedSearch, setSearch, searchInputRef } = useDebouncedSearch();

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
      const url = `${window.location.origin}/register-staff?code=${data.code}`;
      Dialog.alert({
        title: "邀请员工",
        content: (
          <div className="flex flex-col items-center py-4">
            <QRCodeSVG value={url} size={180} />
            <p className="mt-3 text-sm text-gray-500">
              邀请码: {data.code}
            </p>
            <p className="text-xs text-gray-400">
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
      <PageHeader
        title="员工管理"
        right={
          <div className="flex items-center gap-3">
            <Search
              size={20}
              className="text-gray-500"
              onClick={() => setShowSearch(!showSearch)}
            />
            <span className="text-blue-500 text-sm" onClick={handleInvite}>
              邀请
            </span>
          </div>
        }
      />

      {showSearch && (
        <div className="px-4 pb-2">
          <SearchBar
            ref={searchInputRef}
            placeholder="搜索员工"
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
          keyExtractor={(s) => s.id}
          emptyText="暂无员工"
          searchEmpty={!!debouncedSearch && !list.length}
          estimateSize={72}
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
                prefix={
                  <Avatar
                    name={staff.displayName || staff.username}
                    src={staff.avatar}
                    size="md"
                  />
                }
                description={staff.username}
              >
                {staff.displayName || staff.username}
              </List.Item>
            </SwipeAction>
          )}
        />
      </div>
    </div>
  );
}
