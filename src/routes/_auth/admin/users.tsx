import { createFileRoute } from "@tanstack/react-router";
import { List, Tag } from "antd-mobile";
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
          estimateSize={72}
          renderItem={(user) => (
            <List.Item
              className="border-b border-gray-100"
              prefix={
                <div className="flex h-full items-center mr-2">
                  <Avatar
                    name={user.displayName || user.username}
                    src={user.avatar}
                    size="md"
                  />
                </div>
              }
              description={user.phone}
              extra={
                <div className="flex flex-col items-end gap-1">
                  <Tag color={getRoleColor(user)}>{getRoleText(user)}</Tag>
                  <span className="text-xs text-gray-400">
                    <RelativeTime date={user.createdAt} />
                  </span>
                </div>
              }
              arrow={false}
            >
              {user.displayName || user.username}
            </List.Item>
          )}
        />
      </div>
    </div>
  );
}
