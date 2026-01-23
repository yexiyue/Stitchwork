/**
 * ThreadHistoryAdapter 实现
 * 用于加载和保存消息历史
 *
 * AI SDK runtime 使用 withFormat 模式调用 history adapter:
 * historyAdapter.withFormat(formatAdapter).append(item)
 * historyAdapter.withFormat(formatAdapter).load()
 */
import type {
  ThreadHistoryAdapter,
  ExportedMessageRepository,
  ThreadMessage,
} from "@assistant-ui/react";
import { useAssistantApi } from "@assistant-ui/react";
import { chatApi, type ChatMessage } from "@/api/chat";
import { useMemo } from "react";

interface MessageRepositoryItem {
  message: ThreadMessage;
  parentId: string | null;
}

interface MessageFormatItem<TMessage> {
  message: TMessage;
  parentId: string | null;
}

interface MessageFormatRepository<TMessage> {
  headId?: string | null;
  messages: MessageFormatItem<TMessage>[];
}

interface MessageFormatAdapter<TMessage, TStorageFormat> {
  format: string;
  encode(item: MessageFormatItem<TMessage>): TStorageFormat;
  decode(entry: {
    id: string;
    parent_id: string | null;
    format: string;
    content: TStorageFormat;
  }): MessageFormatItem<TMessage>;
  getId(message: TMessage): string;
}

interface GenericThreadHistoryAdapter<TMessage> {
  load(): Promise<MessageFormatRepository<TMessage>>;
  append(item: MessageFormatItem<TMessage>): Promise<void>;
}

/**
 * 创建消息历史适配器的 Hook
 */
export function useHistoryAdapter(): ThreadHistoryAdapter {
  const api = useAssistantApi();

  return useMemo<ThreadHistoryAdapter>(() => {
    const getRemoteId = () => api.threadListItem().getState().remoteId;

    const initializeAndGetRemoteId = async () => {
      const { remoteId } = await api.threadListItem().initialize();
      return remoteId;
    };

    return {
      // 直接调用模式（用于 LocalRuntime）
      async load(): Promise<ExportedMessageRepository> {
        const remoteId = getRemoteId();
        if (!remoteId) return { messages: [] };

        const messages = await chatApi.listMessages(remoteId);
        const exportedMessages = messages.map(
          (m: ChatMessage) => m.content as MessageRepositoryItem,
        );

        return {
          headId: exportedMessages.at(-1)?.message.id ?? null,
          messages: exportedMessages,
        };
      },

      async append(item: MessageRepositoryItem): Promise<void> {
        const remoteId = await initializeAndGetRemoteId();
        if (remoteId) {
          await chatApi.addMessage(remoteId, {
            id: item.message.id,
            parentId: item.parentId,
            format: "thread-message",
            content: item,
          });
        }
      },

      // withFormat 模式（用于 AI SDK runtime）
      withFormat<TMessage, TStorageFormat>(
        formatAdapter: MessageFormatAdapter<TMessage, TStorageFormat>,
      ): GenericThreadHistoryAdapter<TMessage> {
        return {
          async load(): Promise<MessageFormatRepository<TMessage>> {
            const remoteId = getRemoteId();
            if (!remoteId) return { messages: [] };

            const messages = await chatApi.listMessages(remoteId);

            // 直接从数据库字段读取，不再需要嵌套解析
            const decodedMessages = messages
              .filter((m) => m.format === formatAdapter.format)
              .map((m: ChatMessage) =>
                formatAdapter.decode({
                  id: m.id,
                  parent_id: m.parentId,
                  format: m.format,
                  content: m.content as TStorageFormat,
                }),
              );

            return {
              headId:
                decodedMessages.length > 0
                  ? formatAdapter.getId(decodedMessages.at(-1)!.message)
                  : null,
              messages: decodedMessages,
            };
          },

          async append(item: MessageFormatItem<TMessage>): Promise<void> {
            const remoteId = await initializeAndGetRemoteId();
            if (!remoteId) return;

            const encoded = formatAdapter.encode(item);
            await chatApi.addMessage(remoteId, {
              id: formatAdapter.getId(item.message),
              parentId: item.parentId,
              format: formatAdapter.format,
              content: encoded,
            });
          },
        };
      },
    };
  }, [api]);
}
