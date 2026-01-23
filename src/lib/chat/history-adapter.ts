/**
 * ThreadHistoryAdapter 实现
 * 用于加载和保存消息历史
 */
import type {
  ThreadHistoryAdapter,
  ExportedMessageRepository,
  ThreadMessage,
} from "@assistant-ui/react";
import { useAssistantApi } from "@assistant-ui/react";
import { chatApi, type ChatMessage } from "@/api/chat";
import { useMemo } from "react";

/**
 * 消息仓库项（与 assistant-ui 内部类型匹配）
 */
interface MessageRepositoryItem {
  message: ThreadMessage;
  parentId: string | null;
}

/**
 * 创建消息历史适配器的 Hook
 * 需要在 RuntimeAdapterProvider 内部使用
 */
export function useHistoryAdapter(): ThreadHistoryAdapter {
  const api = useAssistantApi();

  return useMemo<ThreadHistoryAdapter>(
    () => ({
      /**
       * 加载历史消息
       */
      async load(): Promise<ExportedMessageRepository> {
        const { remoteId } = api.threadListItem().getState();
        if (!remoteId) return { messages: [] };

        const messages = await chatApi.listMessages(remoteId);

        // 构建 ExportedMessageRepository 格式
        const exportedMessages = messages.map(
          (m: ChatMessage) => m.value as MessageRepositoryItem
        );
        const headId =
          exportedMessages.length > 0
            ? exportedMessages[exportedMessages.length - 1].message.id
            : null;

        return {
          headId,
          messages: exportedMessages,
        };
      },

      /**
       * 保存新消息
       */
      async append(item: MessageRepositoryItem): Promise<void> {
        const { remoteId } = await api.threadListItem().initialize();
        if (!remoteId) return;

        await chatApi.addMessage(remoteId, item);
      },
    }),
    [api]
  );
}
