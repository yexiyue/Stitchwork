import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Card, Badge } from "antd-mobile";
import { ClipboardCheck, FileEdit, Users, Store } from "lucide-react";
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
      {user?.workshop ? (
        <div className="p-4 bg-gray-50 rounded-lg" onClick={() => isBoss && navigate({ to: "/workshop" })}>
          <div className="flex items-start gap-3">
            {user.workshop.image ? (
              <img src={user.workshop.image} className="w-16 h-16 rounded-lg object-cover" />
            ) : (
              <div className="w-16 h-16 rounded-lg bg-gray-200 flex items-center justify-center">
                <Store size={24} className="text-gray-400" />
              </div>
            )}
            <div className="flex-1 min-w-0">
              <h2 className="font-medium">{user.workshop.name}</h2>
              {user.workshop.address && <p className="text-sm text-gray-500 mt-1">{user.workshop.address}</p>}
              {user.workshop.desc && <p className="text-sm text-gray-400 mt-1 line-clamp-2">{user.workshop.desc}</p>}
            </div>
          </div>
        </div>
      ) : (
        <div className="p-4 bg-gray-50 rounded-lg text-center text-gray-500">
          {isBoss ? (
            <span onClick={() => navigate({ to: "/workshop" })}>点击创建工坊</span>
          ) : (
            <span>暂未加入工坊</span>
          )}
        </div>
      )}

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

        {isBoss && (
          <Card className="cursor-pointer" onClick={() => navigate({ to: "/customers" })}>
            <div className="flex flex-col items-center gap-2 py-2">
              <Users size={32} className="text-green-500" />
              <span>客户管理</span>
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
