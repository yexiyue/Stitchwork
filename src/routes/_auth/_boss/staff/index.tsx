import { createFileRoute } from "@tanstack/react-router";
import { List, Dialog, Toast, SearchBar, SwipeAction } from "antd-mobile";
import { Search } from "lucide-react";
import { Avatar, VirtualList, PageHeader } from "@/components";
import { authApi } from "@/api";
import { useInfiniteQuery } from "@tanstack/react-query";
import { useState, useRef } from "react";
import { useDebounceFn } from "ahooks";
import type { SearchBarRef } from "antd-mobile/es/components/search-bar";
import { QRCodeSVG } from "qrcode.react";

export const Route = createFileRoute("/_auth/_boss/staff/")({
  component: StaffPage,
});

const PAGE_SIZE = 20;

function StaffPage() {
  const [inviteCode, setInviteCode] = useState<{
    code: string;
    expiresAt: number;
  } | null>(null);
  const [search, setSearch] = useState("");
  const [debouncedSearch, setDebouncedSearch] = useState("");
  const [showSearch, setShowSearch] = useState(false);
  const searchInputRef = useRef<SearchBarRef>(null);

  const { run: updateSearch } = useDebounceFn(
    (val: string) => setDebouncedSearch(val),
    { wait: 300 }
  );

  const { data, isFetching, fetchNextPage, hasNextPage, refetch } =
    useInfiniteQuery({
      queryKey: ["staff", debouncedSearch],
      queryFn: ({ pageParam = 1 }) =>
        authApi.getStaffList({
          page: pageParam,
          pageSize: PAGE_SIZE,
          search: debouncedSearch || undefined,
        }),
      initialPageParam: 1,
      getNextPageParam: (lastPage, allPages) =>
        allPages.flatMap((p) => p.list).length < lastPage.total
          ? allPages.length + 1
          : undefined,
    });

  const list = data?.pages.flatMap((p) => p.list) ?? [];

  const handleGenerateInviteCode = async () => {
    try {
      const res = await authApi.generateInviteCode();
      setInviteCode(res);
    } catch (e) {
      Toast.show({ content: e instanceof Error ? e.message : "生成失败" });
    }
  };

  const handleCopyCode = async () => {
    if (!inviteCode) return;
    try {
      await navigator.clipboard.writeText(inviteCode.code);
      Toast.show({ content: "已复制" });
    } catch {
      Toast.show({ content: "复制失败" });
    }
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {showSearch ? (
        <div className="h-11.25 flex items-center px-3">
          <SearchBar
            ref={searchInputRef}
            placeholder="搜索员工"
            value={search}
            onChange={(v) => {
              setSearch(v);
              updateSearch(v);
            }}
            onCancel={() => {
              setShowSearch(false);
              setSearch("");
              updateSearch("");
            }}
            showCancelButton
            autoFocus
            className="flex-1"
          />
        </div>
      ) : (
        <PageHeader title="员工管理" right={
          <div className="flex items-center gap-4">
            <Search
              size={20}
              className="text-gray-500"
              onClick={() => {
                setShowSearch(true);
                setTimeout(() => searchInputRef.current?.focus(), 100);
              }}
            />
            <span className="text-blue-500" onClick={handleGenerateInviteCode}>
              邀请码
            </span>
          </div>
        } />
      )}

      <div className="flex flex-1 overflow-hidden">
        <VirtualList
          data={list}
          loading={isFetching}
          hasMore={!!hasNextPage}
          onLoadMore={fetchNextPage}
          onRefresh={refetch}
          keyExtractor={(s) => s.id}
          emptyText="暂无员工"
          renderItem={(staff) => (
            <SwipeAction
              key={staff.id}
              rightActions={[
                {
                  key: "delete",
                  text: "移除",
                  color: "danger",
                  onClick: async () => {
                    const confirmed = await Dialog.confirm({
                      content: `确定移除员工「${staff.displayName || staff.username}」吗？`,
                    });
                    if (confirmed) {
                      try {
                        await authApi.removeStaff(staff.id);
                        Toast.show({ content: "已移除" });
                        refetch();
                      } catch (e) {
                        Toast.show({
                          content: e instanceof Error ? e.message : "移除失败",
                        });
                      }
                    }
                  },
                },
              ]}
            >
              <List.Item
                prefix={
                  <div className="flex h-full flex-col justify-center pr-2">
                    <Avatar name={staff.displayName || staff.username} src={staff.avatar} />
                  </div>
                }
                description={staff.phone || "未填写手机号"}
              >
                {staff.displayName || staff.username}
              </List.Item>
            </SwipeAction>
          )}
        />
      </div>

      <Dialog
        visible={!!inviteCode}
        title="邀请码"
        content={
          <div className="text-center py-4">
            <div className="flex justify-center mb-4">
              <QRCodeSVG
                value={`stitch://bind?code=${inviteCode?.code}`}
                size={160}
              />
            </div>
            <div className="text-2xl font-mono font-bold tracking-wider mb-2">
              {inviteCode?.code}
            </div>
            <div className="text-gray-500 text-sm">
              有效期至{" "}
              {inviteCode &&
                new Date(inviteCode.expiresAt * 1000).toLocaleString()}
            </div>
          </div>
        }
        actions={[
          { key: "copy", text: "复制", onClick: handleCopyCode },
          { key: "close", text: "关闭", onClick: () => setInviteCode(null) },
        ]}
        onClose={() => setInviteCode(null)}
      />
    </div>
  );
}
