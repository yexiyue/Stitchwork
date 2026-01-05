import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, Button, Dialog, Toast, Input, PullToRefresh } from "antd-mobile";
import { Users, UserPlus, LogOut, Camera, Store, Phone, User as UserIcon, AtSign, Lock, ClipboardList } from "lucide-react";
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

  const handleEditPhone = async () => {
    const phone = await new Promise<string | null>((resolve) => {
      let value = user?.phone || "";
      Dialog.confirm({
        title: "修改电话",
        content: (
          <Input
            placeholder="请输入电话号码"
            type="tel"
            defaultValue={value}
            onChange={(v) => (value = v)}
          />
        ),
        onConfirm: () => resolve(value || null),
        onCancel: () => resolve(null),
      });
    });
    if (phone === null) return;
    try {
      await authApi.updateProfile({ phone });
      updateUser({ phone });
      Toast.show({ content: "修改成功" });
    } catch (e) {
      Toast.show({ content: e instanceof Error ? e.message : "修改失败" });
    }
  };

  const handleChangePassword = async () => {
    const result = await new Promise<{ oldPassword: string; newPassword: string } | null>((resolve) => {
      let oldPassword = "";
      let newPassword = "";
      let confirmPassword = "";
      Dialog.confirm({
        title: "修改密码",
        content: (
          <div className="space-y-3">
            <Input
              placeholder="请输入旧密码"
              type="password"
              onChange={(v) => (oldPassword = v)}
            />
            <Input
              placeholder="请输入新密码"
              type="password"
              onChange={(v) => (newPassword = v)}
            />
            <Input
              placeholder="请确认新密码"
              type="password"
              onChange={(v) => (confirmPassword = v)}
            />
          </div>
        ),
        onConfirm: () => {
          if (!oldPassword || !newPassword || !confirmPassword) {
            Toast.show({ content: "请填写完整" });
            return resolve(null);
          }
          if (newPassword !== confirmPassword) {
            Toast.show({ content: "两次密码不一致" });
            return resolve(null);
          }
          resolve({ oldPassword, newPassword });
        },
        onCancel: () => resolve(null),
      });
    });
    if (!result) return;
    try {
      await authApi.changePassword(result);
      Toast.show({ content: "密码修改成功，请重新登录" });
      logout();
      navigate({ to: "/login" });
    } catch (e) {
      Toast.show({ content: e instanceof Error ? e.message : "修改失败" });
    }
  };

  // 根据用户名生成渐变色（预设柔和配色）
  const getGradient = (name: string) => {
    const gradients = [
      "linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%)", // 暖橙
      "linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)", // 薄荷粉
      "linear-gradient(135deg, #d299c2 0%, #fef9d7 100%)", // 紫黄
      "linear-gradient(135deg, #89f7fe 0%, #66a6ff 100%)", // 天蓝
      "linear-gradient(135deg, #fddb92 0%, #d1fdff 100%)", // 柠檬青
      "linear-gradient(135deg, #f6d365 0%, #fda085 100%)", // 阳光橙
      "linear-gradient(135deg, #96fbc4 0%, #f9f586 100%)", // 清新绿
      "linear-gradient(135deg, #fbc2eb 0%, #a6c1ee 100%)", // 粉蓝
    ];
    let hash = 0;
    for (let i = 0; i < name.length; i++) {
      hash = name.charCodeAt(i) + ((hash << 5) - hash);
    }
    return gradients[Math.abs(hash) % gradients.length];
  };

  return (
    <PullToRefresh onRefresh={async () => {
      const profile = await authApi.getProfile();
      updateUser(profile);
    }}>
      <div style={{ background: getGradient(user?.username || "U"), minHeight: "100vh" }}>
        <div className="flex flex-col items-center py-16">
          <div className="relative" onClick={openFilePicker}>
            <Avatar
              name={user?.displayName || user?.username || "U"}
              src={user?.avatar}
              size="xl"
            />
            <div className="absolute -bottom-1 -right-1 w-5 h-5 bg-white rounded-full shadow flex items-center justify-center">
              <Camera size={12} className="text-gray-500" />
            </div>
          </div>
          {FileInput}
        </div>

        <div className="bg-white rounded-t-3xl p-4 min-h-screen">
          <List header="个人信息">
          <List.Item
            prefix={<Camera size={20} />}
            onClick={openFilePicker}
            extra={
              <Avatar
                name={user?.displayName || user?.username || "U"}
                src={user?.avatar}
                size="sm"
              />
            }
          >
            头像
          </List.Item>
          <List.Item
            prefix={<UserIcon size={20} />}
            onClick={handleEditName}
            extra={user?.displayName || "未设置"}
          >
            昵称
          </List.Item>
          <List.Item
            prefix={<Phone size={20} />}
            onClick={handleEditPhone}
            extra={user?.phone || "未设置"}
          >
            电话
          </List.Item>
          <List.Item
            prefix={<AtSign size={20} />}
            extra={user?.username}
          >
            用户名
          </List.Item>
          <List.Item
            prefix={<Lock size={20} />}
            onClick={handleChangePassword}
            extra="修改"
          >
            密码
          </List.Item>
        </List>

        {isBoss && (
          <List header="管理">
            <List.Item prefix={<Users size={20} />} onClick={() => navigate({ to: "/customers" })}>
              客户管理
            </List.Item>
            <List.Item prefix={<UserPlus size={20} />} onClick={() => navigate({ to: "/staff" })}>
              员工管理
            </List.Item>
            <List.Item prefix={<Store size={20} />} onClick={() => navigate({ to: "/workshop" })}>
              工坊设置
            </List.Item>
          </List>
        )}

        {!isBoss && (
          <List header="我的">
            <List.Item prefix={<ClipboardList size={20} />} onClick={() => navigate({ to: "/my-records" })}>记件记录</List.Item>
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
      </div>
    </PullToRefresh>
  );
}
