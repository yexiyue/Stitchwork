import { createFileRoute } from "@tanstack/react-router";
import { Dialog, Tag } from "antd-mobile";
import type { UserListItem } from "@/types";
import { adminApi } from "@/api";
import { Avatar, RelativeTime, VirtualList } from "@/components";
import { useInfiniteList } from "@/hooks";

export const Route = createFileRoute("/_auth/admin/users")({
  component: UsersPage,
});

function UsersPage() {
  const { list, isFetching, hasMore, loadMore, refresh } =
    useInfiniteList<UserListItem>(["admin", "users"], adminApi.listUsers);

  const getRoleText = (user: UserListItem) => {
    if (user.isSuperAdmin) return "超管";
    return user.role === "boss" ? "老板" : "员工";
  };

  const getRoleColor = (user: UserListItem): "primary" | "success" | "default" => {
    if (user.isSuperAdmin) return "primary";
    return user.role === "boss" ? "success" : "default";
  };

  const showUserInfo = (user: UserListItem) => {
    Dialog.alert({
      title: "用户信息",
      content: (
        <div className="flex flex-col items-center py-4">
          <Avatar
            name={user.displayName || user.username}
            src={user.avatar}
            size="lg"
          />
          <div className="mt-3 text-lg font-medium">
            {user.displayName || user.username}
          </div>
          <Tag color={getRoleColor(user)} className="mt-1">
            {getRoleText(user)}
          </Tag>
          <div className="w-full mt-4 space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-500">用户名</span>
              <span>{user.username}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">手机号</span>
              <span>{user.phone}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">注册时间</span>
              <span><RelativeTime date={user.createdAt} /></span>
            </div>
          </div>
        </div>
      ),
      confirmText: "关闭",
    });
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="p-4 pb-2">
        <h1 className="text-xl">用户管理</h1>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <VirtualList
          data={list}
          loading={isFetching}
          hasMore={hasMore}
          onLoadMore={loadMore}
          onRefresh={refresh}
          keyExtractor={(user) => user.id}
          emptyText="暂无用户"
          estimateSize={80}
          renderItem={(user) => (
            <div
              className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm cursor-pointer active:bg-gray-50"
              onClick={() => showUserInfo(user)}
            >
              <div className="flex items-center gap-3">
                <Avatar
                  name={user.displayName || user.username}
                  src={user.avatar}
                  size="md"
                />
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between items-start">
                    <span className="font-medium truncate">
                      {user.displayName || user.username}
                    </span>
                    <Tag color={getRoleColor(user)} fill="outline" style={{ "--border-radius": "4px" }}>
                      {getRoleText(user)}
                    </Tag>
                  </div>
                  <div className="flex justify-between items-end mt-1">
                    <span className="text-xs text-gray-500">{user.phone}</span>
                    <span className="text-xs text-gray-400">
                      <RelativeTime date={user.createdAt} />
                    </span>
                  </div>
                </div>
              </div>
            </div>
          )}
        />
      </div>
    </div>
  );
}
