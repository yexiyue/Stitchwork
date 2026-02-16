import { cn } from "@/lib/utils";

interface SectionLabelProps {
  children: React.ReactNode;
  className?: string;
  delay?: number;
}

/**
 * 统一的区域标签样式组件
 * 用于 StatsDisplay footer 区域的小标题
 */
export function SectionLabel({
  children,
  className,
  delay = 0,
}: SectionLabelProps) {
  return (
    <span
      className={cn(
        "ml-2 text-muted-foreground relative",
        "text-[10px] sm:text-xs font-medium tracking-wider uppercase",
        "opacity-80",
        "animate-in fade-in slide-in-from-bottom-1 duration-500",
        "ease-[cubic-bezier(0.16,1,0.3,1)] fill-mode-both",
        className
      )}
      style={delay > 0 ? { animationDelay: `${delay}ms` } : undefined}
    >
      {children}
    </span>
  );
}
