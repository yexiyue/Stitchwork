import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, Dialog, SwipeAction, Toast, NavBar, Button } from "antd-mobile";
import { Plus, ChevronLeft, Copy } from "lucide-react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import type { RegisterCode } from "@/types";
import { adminApi } from "@/api";
import { RelativeTime } from "@/components";

export const Route = createFileRoute("/_auth/admin/register-codes")({
  component: RegisterCodesPage,
});

function RegisterCodesPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const { data: codes = [], isLoading } = useQuery({
    queryKey: ["admin", "register-codes"],
    queryFn: adminApi.listRegisterCodes,
  });

  const createMutation = useMutation({
    mutationFn: adminApi.createRegisterCode,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["admin", "register-codes"] });
      Toast.show({ content: "创建成功" });
    },
  });

  const disableMutation = useMutation({
    mutationFn: adminApi.disableRegisterCode,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["admin", "register-codes"] });
      Toast.show({ content: "已禁用" });
    },
  });

  const handleCreate = async () => {
    try {
      const result = await createMutation.mutateAsync();
      Dialog.alert({
        title: "注册码已创建",
        content: (
          <div className="text-center">
            <div className="text-2xl font-mono font-bold my-4">
              {result.code}
            </div>
            <Button
              size="small"
              onClick={() => {
                navigator.clipboard.writeText(result.code);
                Toast.show({ content: "已复制" });
              }}
            >
              <Copy size={16} className="mr-1" />
              复制
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
    navigator.clipboard.writeText(code);
    Toast.show({ content: "已复制" });
  };

  const getStatusText = (code: RegisterCode) => {
    if (code.usedBy) return "已使用";
    if (!code.isActive) return "已禁用";
    return "可用";
  };

  const getStatusColor = (code: RegisterCode) => {
    if (code.usedBy) return "text-gray-400";
    if (!code.isActive) return "text-red-500";
    return "text-green-600";
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <NavBar
        onBack={() => navigate({ to: "/profile" })}
        backIcon={<ChevronLeft size={24} />}
        right={
          <Plus
            size={20}
            className="text-blue-500"
            onClick={handleCreate}
          />
        }
      >
        注册码管理
      </NavBar>

      <div className="flex-1 overflow-auto">
        {isLoading ? (
          <div className="p-4 text-center text-gray-400">加载中...</div>
        ) : codes.length === 0 ? (
          <div className="p-4 text-center text-gray-400">暂无注册码</div>
        ) : (
          <List>
            {codes.map((code) => (
              <SwipeAction
                key={code.id}
                rightActions={
                  code.isActive && !code.usedBy
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
                <List.Item
                  description={
                    code.usedBy
                      ? `使用者: ${code.usedByUsername || "未知"}`
                      : undefined
                  }
                  extra={
                    <div className="flex items-center gap-2">
                      <span className={`text-xs ${getStatusColor(code)}`}>
                        {getStatusText(code)}
                      </span>
                      {!code.usedBy && code.isActive && (
                        <Copy
                          size={16}
                          className="text-gray-400"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCopy(code.code);
                          }}
                        />
                      )}
                    </div>
                  }
                  arrow={false}
                >
                  <div className="font-mono">{code.code}</div>
                  <div className="text-xs text-gray-400">
                    <RelativeTime date={code.createdAt} />
                  </div>
                </List.Item>
              </SwipeAction>
            ))}
          </List>
        )}
      </div>
    </div>
  );
}
