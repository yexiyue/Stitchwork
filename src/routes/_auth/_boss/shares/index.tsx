import { createFileRoute, useNavigate } from "@tanstack/react-router";
import {
  List,
  SwipeAction,
  Dialog,
  Toast,
  NavBar,
  Switch,
  Popup,
  Button,
} from "antd-mobile";
import { Plus, ChevronLeft, Link2, ExternalLink } from "lucide-react";
import { useShares, useUpdateShare, useDeleteShare } from "@/hooks";
import type { Share } from "@/types";
import { RelativeTime } from "@/components";
import { useState } from "react";
import { QRCodeSVG } from "qrcode.react";

export const Route = createFileRoute("/_auth/_boss/shares/")({
  component: SharesPage,
});

function SharesPage() {
  const navigate = useNavigate();
  const { data: shares, isLoading } = useShares();
  const updateMutation = useUpdateShare();
  const deleteMutation = useDeleteShare();

  const [qrPopupVisible, setQrPopupVisible] = useState(false);
  const [selectedShare, setSelectedShare] = useState<Share | null>(null);

  const getShareUrl = (token: string) => {
    const base = window.location.origin;
    return `${base}/share/${token}`;
  };

  const handleCopyLink = async (share: Share) => {
    const url = getShareUrl(share.token);
    try {
      await navigator.clipboard.writeText(url);
      Toast.show({ content: "链接已复制" });
    } catch {
      Toast.show({ content: "复制失败" });
    }
  };

  const handleShowQR = (share: Share) => {
    setSelectedShare(share);
    setQrPopupVisible(true);
  };

  const handleToggleActive = async (share: Share) => {
    try {
      await updateMutation.mutateAsync({
        id: share.id,
        data: { isActive: !share.isActive },
      });
      Toast.show({ content: share.isActive ? "已停用" : "已启用" });
    } catch (e) {
      Toast.show({ content: "操作失败" });
    }
  };

  const handleDelete = (share: Share) => {
    Dialog.confirm({
      content: `确定删除「${share.title}」？`,
      confirmText: "删除",
      cancelText: "取消",
      onConfirm: async () => {
        try {
          await deleteMutation.mutateAsync(share.id);
          Toast.show({ content: "删除成功" });
        } catch (e) {
          Dialog.alert({
            content: e instanceof Error ? e.message : "删除失败",
            confirmText: "确定",
          });
        }
      },
    });
  };

  const handlePreview = (share: Share) => {
    window.open(getShareUrl(share.token), "_blank");
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <NavBar
        onBack={() => navigate({ to: "/" })}
        backIcon={<ChevronLeft size={24} />}
        right={
          <div className="flex w-full justify-end">
            <Plus
              size={20}
              className="text-blue-500"
              onClick={() => navigate({ to: "/shares/new" })}
            />
          </div>
        }
      >
        招工分享
      </NavBar>

      <div className="flex-1 overflow-auto">
        {isLoading ? (
          <div className="p-4 text-center text-gray-400">加载中...</div>
        ) : !shares?.length ? (
          <div className="p-8 text-center">
            <div className="text-gray-400 mb-4">暂无招工分享</div>
            <Button
              color="primary"
              onClick={() => navigate({ to: "/shares/new" })}
            >
              创建分享
            </Button>
          </div>
        ) : (
          <List>
            {shares.map((share) => (
              <SwipeAction
                key={share.id}
                rightActions={[
                  {
                    key: "edit",
                    text: "编辑",
                    color: "primary",
                    onClick: () =>
                      navigate({
                        to: "/shares/$id",
                        params: { id: share.id },
                      }),
                  },
                  {
                    key: "delete",
                    text: "删除",
                    color: "danger",
                    onClick: () => handleDelete(share),
                  },
                ]}
              >
                <List.Item
                  onClick={() => handleShowQR(share)}
                  description={
                    <div className="flex items-center gap-2 text-xs text-gray-400 mt-1">
                      <span>{share.processIds.length} 个工序</span>
                      <span>·</span>
                      <RelativeTime date={share.createdAt} />
                    </div>
                  }
                  extra={
                    <div
                      className="flex items-center gap-2"
                      onClick={(e) => e.stopPropagation()}
                    >
                      <Switch
                        checked={share.isActive}
                        onChange={() => handleToggleActive(share)}
                        style={{ "--height": "24px", "--width": "40px" }}
                      />
                    </div>
                  }
                  arrow={false}
                >
                  <div className="flex items-center gap-2">
                    <span
                      className={
                        share.isActive ? "" : "text-gray-400 line-through"
                      }
                    >
                      {share.title}
                    </span>
                    {!share.isActive && (
                      <span className="text-xs bg-gray-100 text-gray-500 px-1.5 py-0.5 rounded">
                        已停用
                      </span>
                    )}
                  </div>
                </List.Item>
              </SwipeAction>
            ))}
          </List>
        )}
      </div>

      {/* 二维码弹窗 */}
      <Popup
        visible={qrPopupVisible}
        onMaskClick={() => setQrPopupVisible(false)}
        bodyStyle={{ borderTopLeftRadius: 16, borderTopRightRadius: 16 }}
      >
        {selectedShare && (
          <div className="p-6 text-center">
            <h3 className="text-lg font-medium mb-4">{selectedShare.title}</h3>
            <div className="flex justify-center mb-4">
              <QRCodeSVG value={getShareUrl(selectedShare.token)} size={200} />
            </div>
            <p className="text-sm text-gray-500 mb-4">扫码查看招工信息</p>
            <div className="flex gap-3">
              <Button block onClick={() => handleCopyLink(selectedShare)}>
                <div className="flex items-center justify-center gap-2">
                  <Link2 size={16} />
                  复制链接
                </div>
              </Button>
              <Button
                block
                color="primary"
                onClick={() => handlePreview(selectedShare)}
              >
                <div className="flex items-center justify-center gap-2">
                  <ExternalLink size={16} />
                  预览
                </div>
              </Button>
            </div>
          </div>
        )}
      </Popup>
    </div>
  );
}
