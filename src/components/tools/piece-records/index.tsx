import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable } from "@/components/tool-ui/data-table";
import { TableLoading } from "../shared";
import { getPieceRecordsColumns } from "./columns";
import type { Output, BossPieceRecordListResponse } from "../types";

// 计件记录列表查询 UI（老板端）
export const PieceRecordsToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "query_piece_records",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <TableLoading id="piece-records-loading" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: BossPieceRecordListResponse = JSON.parse(result[0].text);

    return (
      <DataTable
        id="piece-records"
        columns={getPieceRecordsColumns()}
        data={data.list}
        rowIdKey="id"
        emptyMessage="暂无计件记录"
        layout="auto"
        maxHeight="400px"
      />
    );
  },
});
