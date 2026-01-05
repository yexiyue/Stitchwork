import { useState, useRef } from "react";
import { useDebounceFn } from "ahooks";
import type { SearchBarRef } from "antd-mobile/es/components/search-bar";

interface UseDebouncedSearchOptions {
  /** 防抖等待时间（毫秒） */
  wait?: number;
}

interface UseDebouncedSearchReturn {
  /** 输入框当前值 */
  search: string;
  /** 防抖后的搜索值 */
  debouncedSearch: string;
  /** 设置搜索值（会自动触发防抖更新） */
  setSearch: (value: string) => void;
  /** 搜索输入框 ref */
  searchInputRef: React.RefObject<SearchBarRef | null>;
  /** 清除搜索 */
  clearSearch: () => void;
  /** 是否有搜索内容 */
  hasSearch: boolean;
}

/**
 * 防抖搜索状态管理 Hook
 */
export function useDebouncedSearch(options: UseDebouncedSearchOptions = {}): UseDebouncedSearchReturn {
  const { wait = 300 } = options;

  const [search, setSearchValue] = useState("");
  const [debouncedSearch, setDebouncedSearch] = useState("");
  const searchInputRef = useRef<SearchBarRef | null>(null);

  const { run: updateDebouncedSearch } = useDebounceFn(
    (val: string) => setDebouncedSearch(val),
    { wait }
  );

  const setSearch = (value: string) => {
    setSearchValue(value);
    updateDebouncedSearch(value);
  };

  const clearSearch = () => {
    setSearchValue("");
    setDebouncedSearch("");
  };

  const hasSearch = debouncedSearch.length > 0;

  return {
    search,
    debouncedSearch,
    setSearch,
    searchInputRef,
    clearSearch,
    hasSearch,
  };
}
