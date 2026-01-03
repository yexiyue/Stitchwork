import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Card, Badge } from "antd-mobile";
import { Home, ClipboardCheck, FileEdit } from "lucide-react";
import { useAuthStore, selectIsBoss } from "@/stores/auth";

export const Route = createFileRoute("/_auth/")({
  component: IndexPage,
});

function IndexPage() {
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);
  const isBoss = useAuthStore(selectIsBoss);

  return (
    <div className="p-4">
      <h1 className="flex items-center gap-2 text-xl font-bold">
        <Home size={24} />
        StitchWork
      </h1>
      <p className="mt-2">欢迎, {user?.displayName || user?.username}</p>
      <p className="text-gray-500">服装加工流程管理系统</p>

      <div className="mt-6 grid grid-cols-2 gap-4">
        {isBoss && (
          <Card className="cursor-pointer" onClick={() => navigate({ to: "/pending" })}>
            <div className="flex flex-col items-center gap-2 py-2">
              <Badge content="0">
                <ClipboardCheck size={32} className="text-orange-500" />
              </Badge>
              <span>待审核</span>
            </div>
          </Card>
        )}

        <Card className="cursor-pointer" onClick={() => navigate({ to: isBoss ? "/orders" : "/record" })}>
          <div className="flex flex-col items-center gap-2 py-2">
            <FileEdit size={32} className="text-blue-500" />
            <span>{isBoss ? "快捷记件" : "录入记件"}</span>
          </div>
        </Card>
      </div>
    </div>
  );
}
