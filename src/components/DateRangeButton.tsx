import { Button, CalendarPicker } from "antd-mobile";
import { Calendar } from "lucide-react";
import dayjs from "dayjs";

interface DateRangeButtonProps {
  startDate: Date | null;
  endDate: Date | null;
  visible: boolean;
  onVisibleChange: (visible: boolean) => void;
  onConfirm: (dates: Date[] | null) => void;
  /** 是否显示图标 */
  showIcon?: boolean;
  /** 日期格式 */
  format?: string;
}

/**
 * 日期范围选择按钮 + CalendarPicker
 */
export function DateRangeButton({
  startDate,
  endDate,
  visible,
  onVisibleChange,
  onConfirm,
  showIcon = true,
  format = "YYYY-MM-DD",
}: DateRangeButtonProps) {
  const formatDate = (date: Date | null) => (date ? dayjs(date).format(format) : "");

  return (
    <>
      {showIcon && (
        <div className="flex items-center gap-2">
          <Calendar size={18} className="text-gray-400" />
          <Button size="small" fill="outline" onClick={() => onVisibleChange(true)}>
            {startDate && endDate
              ? `${formatDate(startDate)} 至 ${formatDate(endDate)}`
              : "选择日期范围"}
          </Button>
        </div>
      )}

      <CalendarPicker
        visible={visible}
        selectionMode="range"
        defaultValue={startDate && endDate ? [startDate, endDate] : undefined}
        onClose={() => onVisibleChange(false)}
        onConfirm={(val) => {
          onConfirm(val as Date[] | null);
        }}
        title="选择日期范围"
      />
    </>
  );
}
