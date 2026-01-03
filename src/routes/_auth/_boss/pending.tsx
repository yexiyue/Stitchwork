import { createFileRoute } from "@tanstack/react-router";
import { List } from "antd-mobile";

export const Route = createFileRoute("/_auth/_boss/pending")({
  component: PendingPage,
});

function PendingPage() {
  return (
    <div className="p-4">
      <h1 className="mb-4 text-xl font-bold">待审核记件</h1>
      <p className="text-gray-500">审核员工提交的记件记录</p>
      <List header="待审核列表">
        <List.Item>暂无待审核记录</List.Item>
      </List>
    </div>
  );
}
