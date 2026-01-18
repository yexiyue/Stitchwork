import { DataTable } from "@/components/tool-ui/data-table";

// DataTable 加载状态组件
export function DataTableLoading() {
  return (
    <DataTable
      id="my-payrolls-loading"
      columns={[]}
      data={[]}
      isLoading={true}
      layout="auto"
    />
  );
}
