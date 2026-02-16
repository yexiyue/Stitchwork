import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable } from "@/components/tool-ui/data-table";
import { ToolLoading } from "../shared";
import { getRecordsColumns } from "./columns";
import type { Output, MyRecordsResponse } from "../types";

// 计件记录查询 UI
export const RecordsToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_my_records",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="get_my_records" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: MyRecordsResponse = JSON.parse(result[0].text);

    return (
      <DataTable
        id="my-records"
        columns={getRecordsColumns()}
        data={data.list}
        rowIdKey="id"
        emptyMessage="暂无计件记录"
        layout="auto"
        maxHeight="400px"
      />
    );
  },
});
