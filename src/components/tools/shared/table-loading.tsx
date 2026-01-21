import { DataTable } from "@/components/tool-ui/data-table";

interface TableLoadingProps {
  id?: string;
}

// 通用 DataTable 加载状态组件
export function TableLoading({ id = "table-loading" }: TableLoadingProps) {
  return (
    <DataTable
      id={id}
      columns={[]}
      data={[]}
      isLoading={true}
      layout="auto"
    />
  );
}
