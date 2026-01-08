import { createFileRoute } from "@tanstack/react-router";
import { Dialog, SwipeAction, Toast, Button, Tag } from "antd-mobile";
import { Plus, Copy } from "lucide-react";
import { useMutation } from "@tanstack/react-query";
import type { RegisterCode } from "@/types";
import { adminApi } from "@/api";
import { RelativeTime, VirtualList } from "@/components";
import { QRCodeSVG } from "qrcode.react";
import { useInfiniteList } from "@/hooks";
import { copyToClipboard } from "@/utils/clipboard";

export const Route = createFileRoute("/_auth/admin/register-codes")({
  component: RegisterCodesPage,
});

function RegisterCodesPage() {
  const { list, isFetching, hasMore, loadMore, refresh, invalidate } =
    useInfiniteList<RegisterCode>(
      ["admin", "register-codes"],
      adminApi.listRegisterCodes
    );

  const createMutation = useMutation({
    mutationFn: adminApi.createRegisterCode,
    onSuccess: () => {
      invalidate();
      Toast.show({ content: "创建成功" });
    },
  });

  const disableMutation = useMutation({
    mutationFn: adminApi.disableRegisterCode,
    onSuccess: () => {
      invalidate();
      Toast.show({ content: "已禁用" });
    },
  });

  const showCodeQR = (code: string) => {
    const url = `stitchwork://register?code=${code}`;
    Dialog.alert({
      title: "注册码二维码",
      content: (
        <div className="flex flex-col items-center py-4">
          <QRCodeSVG value={url} size={180} />
          <p className="mt-3 text-lg font-mono font-bold">{code}</p>
          <Button
            size="small"
            className="mt-2"
            onClick={() => {
              copyToClipboard(code);
              Toast.show({ content: "已复制" });
            }}
          >
            <div className="flex items-center justify-center">
              <Copy size={16} className="mr-1" />
              复制
            </div>
          </Button>
        </div>
      ),
      confirmText: "关闭",
    });
  };

  const handleCreate = async () => {
    try {
      const result = await createMutation.mutateAsync();
      const url = `stitchwork://register?code=${result.code}`;
      Dialog.alert({
        title: "注册码已创建",
        content: (
          <div className="flex flex-col items-center py-4">
            <QRCodeSVG value={url} size={180} />
            <p className="mt-3 text-lg font-mono font-bold">{result.code}</p>
            <Button
              size="small"
              className="mt-2"
              onClick={() => {
                copyToClipboard(result.code);
                Toast.show({ content: "已复制" });
              }}
            >
              <div className="flex items-center justify-center">
                <Copy size={16} className="mr-1" />
                复制
              </div>
            </Button>
          </div>
        ),
        confirmText: "关闭",
      });
    } catch (e) {
      Dialog.alert({
        content: e instanceof Error ? e.message : "创建失败",
        confirmText: "确定",
      });
    }
  };

  const handleDisable = (code: RegisterCode) => {
    Dialog.confirm({
      content: `确定禁用注册码「${code.code}」？`,
      confirmText: "禁用",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await disableMutation.mutateAsync(code.id);
        } catch (e) {
          Dialog.alert({
            content: e instanceof Error ? e.message : "禁用失败",
            confirmText: "确定",
          });
        }
      },
    });
  };

  const handleCopy = (code: string) => {
    copyToClipboard(code);
    Toast.show({ content: "已复制" });
  };

  const showUserInfo = (code: RegisterCode) => {
    Dialog.alert({
      title: "使用者信息",
      content: (
        <div className="py-2 space-y-2">
          <div className="flex justify-between">
            <span className="text-gray-500">注册码</span>
            <span className="font-mono">{code.code}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-500">使用者</span>
            <span>{code.usedByUsername || "未知"}</span>
          </div>
          {code.usedAt && (
            <div className="flex justify-between">
              <span className="text-gray-500">使用时间</span>
              <span><RelativeTime date={code.usedAt} /></span>
            </div>
          )}
        </div>
      ),
      confirmText: "关闭",
    });
  };

  const getStatusTag = (code: RegisterCode) => {
    if (code.usedBy) return { color: "default" as const, text: "已使用" };
    if (!code.isActive) return { color: "danger" as const, text: "已禁用" };
    return { color: "success" as const, text: "可用" };
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="p-4 pb-2">
        <div className="flex items-center justify-between">
          <h1 className="text-xl">注册码管理</h1>
          <Plus
            size={20}
            className="text-blue-500"
            onClick={handleCreate}
          />
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <VirtualList
          data={list}
          loading={isFetching}
          hasMore={hasMore}
          onLoadMore={loadMore}
          onRefresh={refresh}
          keyExtractor={(code) => code.id}
          emptyText="暂无注册码"
          estimateSize={80}
          renderItem={(code) => {
            const status = getStatusTag(code);
            const isClickable = !code.usedBy && code.isActive;
            return (
              <SwipeAction
                rightActions={
                  isClickable
                    ? [
                        {
                          key: "disable",
                          text: "禁用",
                          color: "danger",
                          onClick: () => handleDisable(code),
                        },
                      ]
                    : []
                }
              >
                <div
                  className="bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm cursor-pointer active:bg-gray-50"
                  onClick={() => {
                    if (code.usedBy) {
                      showUserInfo(code);
                    } else if (code.isActive) {
                      showCodeQR(code.code);
                    }
                  }}
                >
                  <div className="flex justify-between items-start">
                    <div className="font-mono text-base font-medium">{code.code}</div>
                    <Tag color={status.color} fill="outline" style={{ "--border-radius": "4px" }}>
                      {status.text}
                    </Tag>
                  </div>
                  <div className="flex justify-between items-end mt-2">
                    <div className="text-xs text-gray-400">
                      <RelativeTime date={code.createdAt} />
                    </div>
                    {code.usedBy ? (
                      <div className="text-xs text-gray-500">
                        使用者: {code.usedByUsername || "未知"}
                      </div>
                    ) : isClickable ? (
                      <Copy
                        size={16}
                        className="text-blue-500"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCopy(code.code);
                        }}
                      />
                    ) : null}
                  </div>
                </div>
              </SwipeAction>
            );
          }}
        />
      </div>
    </div>
  );
}
