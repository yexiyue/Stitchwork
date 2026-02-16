import { Dialog, SwipeAction, Toast, Button, Tag, NavBar } from "antd-mobile";
import { Plus, Copy, ChevronLeft } from "lucide-react";
import { useMutation } from "@tanstack/react-query";
import type { RegisterCode } from "@/types";
import { registerCodeApi } from "@/api";
import { RelativeTime, VirtualList } from "@/components";
import { QRCodeSVG } from "qrcode.react";
import { useInfiniteList } from "@/hooks";
import { copyToClipboard } from "@/utils/clipboard";

interface RegisterCodeManagerProps {
  queryKey: string[];
  onBack?: () => void;
}

export function RegisterCodeManager({ queryKey, onBack }: RegisterCodeManagerProps) {
  const { list, isFetching, hasMore, loadMore, refresh, invalidate } =
    useInfiniteList<RegisterCode>(queryKey, registerCodeApi.list);

  const createMutation = useMutation({
    mutationFn: registerCodeApi.create,
    onSuccess: () => {
      invalidate();
      Toast.show({ content: "创建成功" });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: registerCodeApi.delete,
    onSuccess: () => {
      invalidate();
      Toast.show({ content: "已删除" });
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

  const handleDelete = (code: RegisterCode) => {
    Dialog.confirm({
      content: `确定删除注册码「${code.code}」？`,
      confirmText: "删除",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await deleteMutation.mutateAsync(code.id);
        } catch (e) {
          Dialog.alert({
            content: e instanceof Error ? e.message : "删除失败",
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
              <span>
                <RelativeTime date={code.usedAt} />
              </span>
            </div>
          )}
        </div>
      ),
      confirmText: "关闭",
    });
  };

  const isAvailable = (code: RegisterCode) => !code.usedBy;

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <NavBar
        onBack={onBack}
        backIcon={onBack ? <ChevronLeft size={24} /> : null}
        right={
          <div className="w-full flex justify-end items-center">
            <Plus size={20} className="text-blue-500" onClick={handleCreate} />
          </div>
        }
      >
        注册码管理
      </NavBar>

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
            const available = isAvailable(code);
            return (
              <SwipeAction
                rightActions={
                  available
                    ? [
                        {
                          key: "delete",
                          text: "删除",
                          color: "danger",
                          onClick: () => handleDelete(code),
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
                    } else {
                      showCodeQR(code.code);
                    }
                  }}
                >
                  <div className="flex justify-between items-start">
                    <div className="font-mono text-base font-medium">
                      {code.code}
                    </div>
                    <Tag
                      color={available ? "success" : "default"}
                      fill="outline"
                      style={{ "--border-radius": "4px" }}
                    >
                      {available ? "可用" : "已使用"}
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
                    ) : (
                      <Copy
                        size={16}
                        className="text-blue-500"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCopy(code.code);
                        }}
                      />
                    )}
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
