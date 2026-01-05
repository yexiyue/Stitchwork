import { useState, useMemo } from "react";
import dayjs from "dayjs";

interface UseDateRangeOptions {
  /** 默认开始日期 */
  defaultStart?: Date;
  /** 默认结束日期 */
  defaultEnd?: Date;
  /** 默认使用本月范围 */
  defaultToMonth?: boolean;
  /** 允许空值 */
  nullable?: boolean;
}

interface UseDateRangeReturn {
  startDate: Date | null;
  endDate: Date | null;
  setStartDate: (date: Date | null) => void;
  setEndDate: (date: Date | null) => void;
  /** 格式化的日期参数，用于 API 调用 */
  dateParams: {
    startDate?: string;
    endDate?: string;
  };
  /** 日历选择器可见状态 */
  calendarVisible: boolean;
  setCalendarVisible: (visible: boolean) => void;
  /** 处理日历选择确认 */
  handleCalendarConfirm: (val: Date[] | null) => void;
  /** 清除日期范围 */
  clearDateRange: () => void;
  /** 是否有日期筛选 */
  hasDateFilter: boolean;
}

/**
 * 日期范围状态管理 Hook
 */
export function useDateRange(options: UseDateRangeOptions = {}): UseDateRangeReturn {
  const {
    defaultStart,
    defaultEnd,
    defaultToMonth = false,
    nullable = false,
  } = options;

  const getDefaultStart = () => {
    if (defaultStart) return defaultStart;
    if (defaultToMonth) return dayjs().startOf("month").toDate();
    return nullable ? null : dayjs().startOf("month").toDate();
  };

  const getDefaultEnd = () => {
    if (defaultEnd) return defaultEnd;
    if (defaultToMonth) return dayjs().endOf("month").toDate();
    return nullable ? null : dayjs().endOf("month").toDate();
  };

  const [startDate, setStartDate] = useState<Date | null>(getDefaultStart);
  const [endDate, setEndDate] = useState<Date | null>(getDefaultEnd);
  const [calendarVisible, setCalendarVisible] = useState(false);

  const dateParams = useMemo(() => ({
    startDate: startDate ? dayjs(startDate).format("YYYY-MM-DD") : undefined,
    endDate: endDate ? dayjs(endDate).format("YYYY-MM-DD") : undefined,
  }), [startDate, endDate]);

  const handleCalendarConfirm = (val: Date[] | null) => {
    if (val && val.length === 2 && val[0] && val[1]) {
      setStartDate(val[0]);
      setEndDate(val[1]);
    } else if (nullable) {
      setStartDate(null);
      setEndDate(null);
    }
    setCalendarVisible(false);
  };

  const clearDateRange = () => {
    setStartDate(null);
    setEndDate(null);
  };

  const hasDateFilter = startDate !== null;

  return {
    startDate,
    endDate,
    setStartDate,
    setEndDate,
    dateParams,
    calendarVisible,
    setCalendarVisible,
    handleCalendarConfirm,
    clearDateRange,
    hasDateFilter,
  };
}
