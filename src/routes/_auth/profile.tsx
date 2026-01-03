import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, Button, Dialog, Toast, Input, PullToRefresh } from "antd-mobile";
import { Users, UserPlus, LogOut, ScanLine, Camera } from "lucide-react";
import { useAuthStore, selectIsBoss } from "@/stores/auth";
import { authApi } from "@/api";
import { Avatar, useAvatarCropper } from "@/components";
import { uploadImage } from "@/utils/upload";

export const Route = createFileRoute("/_auth/profile")({
  component: ProfilePage,
});

function ProfilePage() {
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);
  const logout = useAuthStore((s) => s.logout);
  const updateUser = useAuthStore((s) => s.updateUser);
  const isBoss = useAuthStore(selectIsBoss);

  const { openFilePicker, FileInput, CropPopup } = useAvatarCropper({
    onConfirm: async (blob) => {
      const fileUrl = await uploadImage(blob, { maxWidthOrHeight: 512 });
      await authApi.updateProfile({ avatar: fileUrl });
      updateUser({ avatar: fileUrl });
      Toast.show({ content: "头像已更新" });
    },
  });

  const handleLogout = () => {
    logout();
    navigate({ to: "/login" });
  };

  const handleBindWorkshop = async () => {
    const code = await new Promise<string | null>((resolve) => {
      Dialog.confirm({
        title: "加入工坊",
        content: (
          <Input
            placeholder="请输入邀请码"
            onChange={(v) => {
              (window as unknown as { __inviteCode: string }).__inviteCode = v;
            }}
          />
        ),
        onConfirm: () =>
          resolve((window as unknown as { __inviteCode: string }).__inviteCode || null),
        onCancel: () => resolve(null),
      });
    });
    if (!code) return;
    try {
      await authApi.bindWorkshop({ inviteCode: code });
      Toast.show({ content: "加入成功" });
      const profile = await authApi.getProfile();
      updateUser(profile);
    } catch (e) {
      Toast.show({ content: e instanceof Error ? e.message : "加入失败" });
    }
  };

  const handleEditName = async () => {
    const name = await new Promise<string | null>((resolve) => {
      let value = user?.displayName || "";
      Dialog.confirm({
        title: "修改昵称",
        content: (
          <Input
            placeholder="请输入昵称"
            defaultValue={value}
            onChange={(v) => (value = v)}
          />
        ),
        onConfirm: () => resolve(value || null),
        onCancel: () => resolve(null),
      });
    });
    if (!name) return;
    try {
      await authApi.updateProfile({ displayName: name });
      updateUser({ displayName: name });
      Toast.show({ content: "修改成功" });
    } catch (e) {
      Toast.show({ content: e instanceof Error ? e.message : "修改失败" });
    }
  };

  return (
    <PullToRefresh onRefresh={async () => {
      const profile = await authApi.getProfile();
      updateUser(profile);
    }}>
      <div className="p-4">
        <div className="flex flex-col items-center mb-6">
          <div className="relative" onClick={openFilePicker}>
            <Avatar
              name={user?.displayName || user?.username || "U"}
              src={user?.avatar}
              size="lg"
            />
            <div className="absolute -bottom-1 -right-1 w-5 h-5 bg-white rounded-full shadow flex items-center justify-center">
              <Camera size={12} className="text-gray-500" />
            </div>
          </div>
          {FileInput}
          <div className="mt-3 text-center" onClick={handleEditName}>
            <h1 className="text-xl font-bold">
              {user?.displayName || user?.username}
            </h1>
            <p className="text-gray-500 text-sm">
              {user?.displayName && user?.username !== user?.displayName && `@${user.username} · `}
              {isBoss ? "老板" : "员工"}
            </p>
          </div>
        </div>

        {isBoss && (
          <List header="管理">
            <List.Item prefix={<Users size={20} />} onClick={() => navigate({ to: "/customers" })}>
              客户管理
            </List.Item>
            <List.Item prefix={<UserPlus size={20} />} onClick={() => navigate({ to: "/staff" })}>
              员工管理
            </List.Item>
          </List>
        )}

        {!isBoss && (
          <List header="我的">
            <List.Item prefix={<ScanLine size={20} />} onClick={handleBindWorkshop}>
              加入工坊
            </List.Item>
            <List.Item onClick={() => {}}>记件记录</List.Item>
          </List>
        )}

        <div className="mt-6">
          <Button block color="default" onClick={handleLogout}>
            <span className="flex items-center justify-center gap-2">
              <LogOut size={18} />
              退出登录
            </span>
          </Button>
        </div>

        {CropPopup}
      </div>
    </PullToRefresh>
  );
}
