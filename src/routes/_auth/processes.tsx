import { createFileRoute } from "@tanstack/react-router";
import { List } from "antd-mobile";

export const Route = createFileRoute("/_auth/processes")({
  component: ProcessesPage,
});

function ProcessesPage() {
  return (
    <div className="p-4">
      <h1 className="mb-4 text-xl font-bold">工序列表</h1>
      <p className="text-gray-500">查看工坊所有工序</p>
      <List header="工序列表">
        <List.Item>暂无工序数据</List.Item>
      </List>
    </div>
  );
}
