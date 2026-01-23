import { useMemo, useRef, useState } from "react";
import { createFileRoute, redirect, useNavigate } from "@tanstack/react-router";
import { ArrowLeft, MessageSquareText } from "lucide-react";
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
import { ThreadList } from "@/components/thread-list";
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

/** History adapter provider - 由 unstable_Provider 渲染 */
function HistoryAdapterProvider({ children }: { children?: React.ReactNode }) {
  console.log(
    "[HistoryAdapterProvider] Rendering - unstable_Provider is working",
  );
  const history = useHistoryAdapter();
  console.log("[HistoryAdapterProvider] history adapter created:", history);
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
  const [threadListOpen, setThreadListOpen] = useState(false);

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
          // 查找已完成的前端工具调用 (type 格式为 "tool-${toolName}")
          const toolPart = lastMsg?.parts.find(
            (part) =>
              part.type === "tool-create_piece_record" &&
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
        // history adapter 通过 unstable_Provider -> RuntimeAdapterProvider 自动注入
      }),
    adapter: threadListAdapter,
  });

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <div className="flex h-full flex-col bg-background">
        {/* 顶部导航栏 */}
        <div className="flex h-12 items-center justify-between border-b px-2">
          <div className="flex items-center">
            <button
              onClick={() => navigate({ to: "/" })}
              className="flex size-10 items-center justify-center rounded-full hover:bg-muted"
            >
              <ArrowLeft size={20} />
            </button>
            <span className="ml-2 font-medium">AI 助手</span>
          </div>
          <button
            onClick={() => setThreadListOpen(true)}
            className="flex size-10 items-center justify-center rounded-full hover:bg-muted"
            aria-label="会话历史"
          >
            <MessageSquareText size={20} />
          </button>
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
      {/* 会话列表侧边栏 */}
      <ThreadList
        open={threadListOpen}
        onClose={() => setThreadListOpen(false)}
      />
    </AssistantRuntimeProvider>
  );
}
