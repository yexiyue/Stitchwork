import { ThreadPrimitive } from "@assistant-ui/react";
import { Button } from "@/components/ui/button";
import { useSuggestions } from "@/lib/chat/suggestion-context";
import { Loader2 } from "lucide-react";
import type { FC } from "react";

export const DynamicSuggestions: FC = () => {
  const { suggestions, isLoading, clearSuggestions } = useSuggestions();

  // 没有建议且不在加载中时不显示
  if (suggestions.length === 0 && !isLoading) {
    return null;
  }

  return (
    <div className="flex flex-col gap-2 px-2 py-2">
      {isLoading && (
        <div className="flex items-center gap-1 text-muted-foreground text-sm">
          <Loader2 className="size-3 animate-spin" />
          <span>生成建议中...</span>
        </div>
      )}
      {suggestions.map((suggestion, index) => (
        <ThreadPrimitive.Suggestion
          key={`${suggestion.prompt}-${index}`}
          prompt={suggestion.prompt}
          send
          asChild
        >
          <Button
            variant="outline"
            size="sm"
            className="h-auto rounded-full px-3 py-1.5 text-sm transition-all hover:bg-primary hover:text-primary-foreground"
            onClick={() => clearSuggestions()}
          >
            {suggestion.prompt}
          </Button>
        </ThreadPrimitive.Suggestion>
      ))}
    </div>
  );
};
