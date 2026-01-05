import { Empty } from "antd-mobile";

interface EmptyStateProps {
  /** 显示的文本 */
  text?: string;
  /** 是否是搜索无结果 */
  isSearch?: boolean;
}

/**
 * 空状态/无数据显示
 */
export function EmptyState({
  text = "暂无数据",
  isSearch = false,
}: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center py-12">
      <Empty description={isSearch ? "未找到匹配结果" : text} />
    </div>
  );
}
