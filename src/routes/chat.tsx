import { useRef } from "react";
import { createFileRoute, redirect, useNavigate } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import { AssistantRuntimeProvider } from "@assistant-ui/react";
import {
  useChatRuntime,
  AssistantChatTransport,
} from "@assistant-ui/react-ai-sdk";
import { useAuthStore } from "@/stores/auth";
import { Thread } from "@/components/thread";
import {
  RecordsToolUi,
  StatsToolUi,
  PayrollToolUi,
  AvailableTasksToolUi,
} from "@/components/tools";
import { CreateRecordTool } from "@/components/tools/crate-record";
import { lastAssistantMessageIsCompleteWithToolCalls } from "ai";

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
  const [token, user] = useAuthStore((state) => [state.token, state.user]);
  const processedToolCalls = useRef(new Set<string>());

  const runtime = useChatRuntime({
    transport: new AssistantChatTransport({
      api: `${import.meta.env.VITE_API_URL}/api/chat`,
      headers: {
        Authorization: `Bearer ${token}`,
      },
      body: {
        sessionId: user?.id,
      },
    }),
    sendAutomaticallyWhen: (options) => {
      if (!lastAssistantMessageIsCompleteWithToolCalls(options)) {
        return false;
      }
      const lastMsg = options.messages.at(-1);
      const toolPart = lastMsg?.parts.find(
        (part) =>
          part.type === "tool-create-record" &&
          part.state === "output-available"
      ) as { toolCallId: string } | undefined;
      if (toolPart) {
        const toolCallId = toolPart.toolCallId;
        if (!processedToolCalls.current.has(toolCallId)) {
          processedToolCalls.current.add(toolCallId);
          return true;
        }
      }
      return false;
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
      <RecordsToolUi />
      <PayrollToolUi />
      <AvailableTasksToolUi />
      <CreateRecordTool />
    </AssistantRuntimeProvider>
  );
}
