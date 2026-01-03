import { createFileRoute } from "@tanstack/react-router";
import { Button } from "antd-mobile";
import { Home } from "lucide-react";

export const Route = createFileRoute("/")({
  component: IndexPage,
});

function IndexPage() {
  return (
    <div style={{ padding: 16 }}>
      <h1 style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <Home size={24} />
        StitchWork
      </h1>
      <p>服装加工流程管理系统</p>
      <Button color="primary">开始使用</Button>
    </div>
  );
}
