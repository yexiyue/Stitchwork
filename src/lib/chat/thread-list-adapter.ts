/**
 * RemoteThreadListAdapter 实现
 * 用于管理会话列表的 CRUD 操作
 */
import type {
  unstable_RemoteThreadListAdapter as RemoteThreadListAdapter,
  ThreadMessage,
} from "@assistant-ui/react";
import { AssistantStream, UIMessageStreamDecoder } from "assistant-stream";
import { chatApi } from "@/api";
import { useAuthStore } from "@/stores/auth";

/** 将 ThreadMessage 转换为后端期望的 UIMessage 格式 */
function toUIMessages(messages: readonly ThreadMessage[]) {
  return messages.map((msg) => ({
    id: msg.id,
    role: msg.role,
    parts: msg.content.map((part) => {
      if (part.type === "text") {
        return { type: "text", text: part.text };
      }
      if (part.type === "tool-call") {
        return {
          type: `tool-${part.toolName}`,
          toolCallId: part.toolCallId,
          toolName: part.toolName,
          state:
            part.result !== undefined ? "output-available" : "input-available",
          input: part.args,
          output: part.result,
        };
      }
      // 其他类型暂不处理
      return { type: "text", text: "" };
    }),
  }));
}

export const createThreadListAdapter = (): RemoteThreadListAdapter => ({
  /**
   * 获取会话列表
   */
  async list() {
    const data = await chatApi.listThreads({ page: 1, pageSize: 100 });
    return {
      threads: data.list.map((t) => ({
        remoteId: t.id,
        externalId: undefined,
        title: t.title ?? undefined,
        status: t.archived ? ("archived" as const) : ("regular" as const),
      })),
    };
  },

  /**
   * 初始化新会话（创建到后端）
   */
  async initialize(_threadId: string) {
    const data = await chatApi.createThread();
    return { remoteId: data.id, externalId: undefined };
  },

  /**
   * 重命名会话
   */
  async rename(remoteId: string, newTitle: string) {
    await chatApi.updateThread(remoteId, { title: newTitle });
  },

  /**
   * 归档会话
   */
  async archive(remoteId: string) {
    await chatApi.updateThread(remoteId, { archived: true });
  },

  /**
   * 取消归档
   */
  async unarchive(remoteId: string) {
    await chatApi.updateThread(remoteId, { archived: false });
  },

  /**
   * 删除会话
   */
  async delete(remoteId: string) {
    await chatApi.deleteThread(remoteId);
  },

  /**
   * 获取单个会话
   */
  async fetch(threadId: string) {
    const data = await chatApi.getThread(threadId);
    return {
      remoteId: data.id,
      externalId: undefined,
      title: data.title ?? undefined,
      status: data.archived ? ("archived" as const) : ("regular" as const),
    };
  },

  /**
   * 生成标题
   */
  async generateTitle(remoteId: string, messages: readonly ThreadMessage[]) {
    const token = useAuthStore.getState().token;
    const response = await fetch(
      `${import.meta.env.VITE_API_URL}/api/chat/generate-title`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          thread_id: remoteId,
          messages: toUIMessages(messages),
        }),
      },
    );

    if (!response.ok) {
      throw new Error(`Failed to generate title: ${response.statusText}`);
    }

    return AssistantStream.fromResponse(response, new UIMessageStreamDecoder());
  },
});
