import { useRef } from "react";
import {
  PullToRefresh,
  InfiniteScroll,
  ErrorBlock,
  DotLoading,
} from "antd-mobile";
import { useVirtualizer } from "@tanstack/react-virtual";

interface VirtualListProps<T> {
  data: T[];
  loading?: boolean;
  hasMore: boolean;
  pageSize?: number;
  onLoadMore: () => Promise<unknown>;
  onRefresh: () => Promise<unknown>;
  renderItem: (item: T, index: number) => React.ReactNode;
  estimateSize?: number;
  emptyText?: string;
  loadingText?: string;
  emptyDescription?: string;
  searchEmpty?: boolean;
  keyExtractor: (item: T) => string;
}

export function VirtualList<T>({
  data,
  loading,
  hasMore,
  pageSize = 20,
  onLoadMore,
  onRefresh,
  renderItem,
  estimateSize = 72,
  emptyText = "暂无数据",
  emptyDescription,
  searchEmpty,
  loadingText = "加载中...",
  keyExtractor,
}: VirtualListProps<T>) {
  const containerRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: data.length,
    getScrollElement: () => containerRef.current,
    estimateSize: () => estimateSize,
    overscan: 5,
  });

  return (
    <div ref={containerRef} className="h-full flex-1 overflow-y-auto px-2">
      <PullToRefresh
        onRefresh={async () => {
          await onRefresh();
        }}
      >
        {loading && !data.length ? (
          <div className="w-full h-full flex items-center justify-center flex-col gap-3">
            <DotLoading color="primary" className="text-2xl" />
            <span className="text-gray-500 text-xl">{loadingText}</span>
          </div>
        ) : !data.length ? (
          <div className="w-full h-full flex items-center justify-center flex-col">
            <ErrorBlock
              status={searchEmpty ? "empty" : "default"}
              title={searchEmpty ? "没有搜索结果" : emptyText}
              description={emptyDescription}
            />
          </div>
        ) : (
          <div
            style={{
              height: virtualizer.getTotalSize(),
              position: "relative",
            }}
          >
            {virtualizer.getVirtualItems().map((virtualRow) => {
              const item = data[virtualRow.index];
              return (
                <div
                  key={keyExtractor(item)}
                  style={{
                    position: "absolute",
                    top: 0,
                    left: 0,
                    width: "100%",
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                >
                  {renderItem(item, virtualRow.index)}
                </div>
              );
            })}
          </div>
        )}
        <InfiniteScroll
          loadMore={async () => {
            await onLoadMore();
          }}
          hasMore={hasMore && data.length >= pageSize}
        />
      </PullToRefresh>
    </div>
  );
}
