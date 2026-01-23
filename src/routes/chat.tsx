import { useMemo, useRef } from "react";
import { createFileRoute, redirect, useNavigate } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import {
  AssistantRuntimeProvider,
  RuntimeAdapterProvider,
  unstable_useRemoteThreadListRuntime as useRemoteThreadListRuntime,
} from "@assistant-ui/react";
import {
  useChatRuntime,
  AssistantChatTransport,
} from "@assistant-ui/react-ai-sdk";
import { useAuthStore } from "@/stores/auth";
import { Thread } from "@/components/thread";
import {
  // 员工端
  RecordsToolUi,
  StatsToolUi,
  PayrollToolUi,
  AvailableTasksToolUi,
  // 老板端
  OrdersToolUi,
  PieceRecordsToolUi,
  WorkerStatsToolUi,
  OverviewToolUi,
  OrderProgressToolUi,
  UnpaidSummaryToolUi,
} from "@/components/tools";
import { CreateRecordTool } from "@/components/tools/crate-record";
import { lastAssistantMessageIsCompleteWithToolCalls } from "ai";
import { createThreadListAdapter, useHistoryAdapter } from "@/lib/chat";

export const Route = createFileRoute("/chat")({
  beforeLoad: () => {
    const token = useAuthStore.getState().token;
    if (!token) {
      throw redirect({ to: "/login" });
    }
  },
  component: ChatPage,
});

/** History adapter provider */
function HistoryAdapterProvider({ children }: { children?: React.ReactNode }) {
  const history = useHistoryAdapter();
  return (
    <RuntimeAdapterProvider adapters={{ history }}>
      {children}
    </RuntimeAdapterProvider>
  );
}

function ChatPage() {
  const navigate = useNavigate();
  const [token, user] = useAuthStore((state) => [state.token, state.user]);
  const processedToolCalls = useRef(new Set<string>());

  // 创建 thread list adapter
  const threadListAdapter = useMemo(() => {
    const adapter = createThreadListAdapter();
    adapter.unstable_Provider = HistoryAdapterProvider;
    return adapter;
  }, []);

  // 使用 useRemoteThreadListRuntime 包装，支持会话持久化
  const runtime = useRemoteThreadListRuntime({
    runtimeHook: () =>
      useChatRuntime({
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
              part.state === "output-available",
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
      }),
    adapter: threadListAdapter,
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
      {/* 员工端 Tool UI */}
      <StatsToolUi />
      <RecordsToolUi />
      <PayrollToolUi />
      <AvailableTasksToolUi />
      <CreateRecordTool />
      {/* 老板端 Tool UI */}
      <OrdersToolUi />
      <PieceRecordsToolUi />
      <WorkerStatsToolUi />
      <OverviewToolUi />
      <OrderProgressToolUi />
      <UnpaidSummaryToolUi />
    </AssistantRuntimeProvider>
  );
}
