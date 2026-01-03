import { createFileRoute } from "@tanstack/react-router";
import { List } from "antd-mobile";

export const Route = createFileRoute("/_auth/_staff/record")({
  component: RecordPage,
});

function RecordPage() {
  return (
    <div className="p-4">
      <h1 className="mb-4 text-xl font-bold">记件录入</h1>
      <p className="text-gray-500">选择工序录入记件</p>
      <List header="选择工序">
        <List.Item>暂无可记件的工序</List.Item>
      </List>
    </div>
  );
}
