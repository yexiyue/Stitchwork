import { createFileRoute } from "@tanstack/react-router";
import { List } from "antd-mobile";

export const Route = createFileRoute("/_auth/_boss/orders")({
  component: OrdersPage,
});

function OrdersPage() {
  return (
    <div className="p-4">
      <h1 className="mb-4 text-xl font-bold">订单管理</h1>
      <p className="text-gray-500">老板可以管理所有订单</p>
      <List header="订单列表">
        <List.Item>暂无订单数据</List.Item>
      </List>
    </div>
  );
}
