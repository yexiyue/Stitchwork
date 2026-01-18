import { createFileRoute, redirect, useNavigate } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import {
  AssistantRuntimeProvider,
  WebSpeechSynthesisAdapter,
} from "@assistant-ui/react";
import {
  useChatRuntime,
  AssistantChatTransport,
} from "@assistant-ui/react-ai-sdk";
import { useAuthStore } from "@/stores/auth";
import { Thread } from "@/components/thread";
import { StatsToolUi } from "@/components/tools";

export const Route = createFileRoute("/chat")({
  beforeLoad: () => {
    const token = useAuthStore.getState().token;
    if (!token) {
      throw redirect({ to: "/login" });
    }
  },
  component: ChatPage,
});

function ChatPage() {
  const navigate = useNavigate();
  const token = useAuthStore((state) => state.token);

  const runtime = useChatRuntime({
    transport: new AssistantChatTransport({
      api: `${import.meta.env.VITE_API_URL}/api/chat`,
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }),
    adapters: {
      speech: new WebSpeechSynthesisAdapter(),
    },
  });

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <div className="flex h-full flex-col bg-background">
        {/* 顶部导航栏 */}
        <div className="flex h-12 items-center border-b px-2">
          <button
            onClick={() => navigate({ to: "/" })}
            className="flex size-10 items-center justify-center rounded-full hover:bg-muted"
          >
            <ArrowLeft size={20} />
          </button>
          <span className="ml-2 font-medium">AI 助手</span>
        </div>
        {/* 聊天内容 */}
        <div className="flex-1 overflow-hidden">
          <Thread />
        </div>
      </div>
      <StatsToolUi />
    </AssistantRuntimeProvider>
  );
}
