import { useInfiniteQuery, useQueryClient } from "@tanstack/react-query";
import { useMemo, useCallback } from "react";

interface PageResult<T> {
  list: T[];
  total: number;
}

interface UseInfiniteListOptions {
  /** 每页数量 */
  pageSize?: number;
  /** 是否启用查询 */
  enabled?: boolean;
}

interface UseInfiniteListReturn<T> {
  /** 扁平化的列表数据 */
  list: T[];
  /** 是否正在加载 */
  isLoading: boolean;
  /** 是否正在获取数据 */
  isFetching: boolean;
  /** 是否有更多数据 */
  hasMore: boolean;
  /** 加载更多 */
  loadMore: () => Promise<unknown>;
  /** 刷新数据 */
  refresh: () => Promise<unknown>;
  /** 使缓存失效 */
  invalidate: () => void;
  /** 总数 */
  total: number;
}

/**
 * 无限列表查询 Hook
 * 封装 useInfiniteQuery 的通用模式
 */
export function useInfiniteList<T, P extends Record<string, unknown> = Record<string, unknown>>(
  /** 查询键 */
  queryKey: unknown[],
  /** 获取数据的函数 */
  fetchFn: (params: P & { page: number; pageSize: number }) => Promise<PageResult<T>>,
  /** 额外的查询参数 */
  params?: P,
  /** 选项 */
  options?: UseInfiniteListOptions
): UseInfiniteListReturn<T> {
  const { pageSize = 20, enabled = true } = options ?? {};
  const queryClient = useQueryClient();

  const {
    data,
    isLoading,
    isFetching,
    fetchNextPage,
    hasNextPage,
    refetch,
  } = useInfiniteQuery({
    queryKey,
    queryFn: ({ pageParam = 1 }) =>
      fetchFn({
        ...(params as P),
        page: pageParam,
        pageSize,
      }),
    initialPageParam: 1,
    getNextPageParam: (lastPage, allPages) =>
      allPages.flatMap((p) => p.list).length < lastPage.total
        ? allPages.length + 1
        : undefined,
    enabled,
  });

  const list = useMemo(
    () => data?.pages.flatMap((p) => p.list) ?? [],
    [data]
  );

  const total = data?.pages[0]?.total ?? 0;

  const loadMore = useCallback(() => fetchNextPage(), [fetchNextPage]);
  const refresh = useCallback(() => refetch(), [refetch]);
  const invalidate = useCallback(
    () => queryClient.invalidateQueries({ queryKey }),
    [queryClient, queryKey]
  );

  return {
    list,
    isLoading,
    isFetching,
    hasMore: !!hasNextPage,
    loadMore,
    refresh,
    invalidate,
    total,
  };
}
