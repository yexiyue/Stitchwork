import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable } from "@/components/tool-ui/data-table";
import { TableLoading } from "../shared";
import { getPayrollColumns } from "./columns";
import type { Output, MyPayrollListResponse } from "../types";

// 工资单查询 UI
export const PayrollToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_my_payrolls",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <TableLoading id="my-payrolls-loading" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: MyPayrollListResponse = JSON.parse(result[0].text);

    // 将 MyPayroll[] 转换为 Record<string, string>[] 格式
    const tableData = data.list.map((item) => ({
      id: item.id,
      userId: item.userId,
      bossId: item.bossId,
      amount: item.amount,
      paymentImage: item.paymentImage || "",
      note: item.note || "",
      paidAt: item.paidAt,
    }));

    return (
      <DataTable
        id="my-payrolls"
        columns={getPayrollColumns()}
        data={tableData}
        rowIdKey="id"
        emptyMessage="暂无工资单记录"
        layout="auto"
        maxHeight="400px"
      />
    );
  },
});
