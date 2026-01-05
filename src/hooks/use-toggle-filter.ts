import { useState, useCallback } from "react";

interface UseToggleFilterReturn<T extends string> {
  /** 当前选中的值 */
  selected: T[];
  /** 切换选中状态 */
  toggle: (value: T) => void;
  /** 清除所有选中 */
  clear: () => void;
  /** 检查是否选中 */
  isSelected: (value: T) => boolean;
  /** 是否有选中项 */
  hasSelected: boolean;
  /** 设置选中值 */
  setSelected: (values: T[]) => void;
}

/**
 * 多选筛选状态管理 Hook
 */
export function useToggleFilter<T extends string>(
  initialValue: T[] = []
): UseToggleFilterReturn<T> {
  const [selected, setSelected] = useState<T[]>(initialValue);

  const toggle = useCallback((value: T) => {
    setSelected((prev) =>
      prev.includes(value)
        ? prev.filter((v) => v !== value)
        : [...prev, value]
    );
  }, []);

  const clear = useCallback(() => {
    setSelected([]);
  }, []);

  const isSelected = useCallback(
    (value: T) => selected.includes(value),
    [selected]
  );

  const hasSelected = selected.length > 0;

  return {
    selected,
    toggle,
    clear,
    isSelected,
    hasSelected,
    setSelected,
  };
}
