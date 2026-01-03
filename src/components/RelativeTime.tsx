import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import "dayjs/locale/zh-cn";

dayjs.extend(relativeTime);
dayjs.locale("zh-cn");

export function RelativeTime({ date }: { date: string }) {
  const d = dayjs(date);
  const isRecent = dayjs().diff(d, "day") < 7;
  return (
    <span className="text-gray-400 text-xs">
      {isRecent ? d.fromNow() : d.format("MM-DD")}
    </span>
  );
}
