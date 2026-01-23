import { client } from "./client";
import type { ListData, QueryParams } from "@/types";

/** 会话模型 */
export interface ChatThread {
  id: string;
  user_id: string;
  title: string | null;
  archived: boolean;
  created_at: string;
  updated_at: string;
}

/** 消息模型 */
export interface ChatMessage {
  id: string;
  thread_id: string;
  value: unknown;
  created_at: string;
}

/** 更新会话 DTO */
export interface UpdateThreadDto {
  title?: string;
  archived?: boolean;
}

export const chatApi = {
  // 会话管理
  listThreads: (params?: QueryParams) =>
    client.get<ListData<ChatThread>>("/api/threads", params),

  getThread: (id: string) => client.get<ChatThread>(`/api/threads/${id}`),

  createThread: () => client.post<ChatThread>("/api/threads"),

  updateThread: (id: string, data: UpdateThreadDto) =>
    client.put<ChatThread>(`/api/threads/${id}`, data),

  deleteThread: (id: string) => client.delete<void>(`/api/threads/${id}`),

  // 消息管理
  listMessages: (threadId: string) =>
    client.get<ChatMessage[]>(`/api/threads/${threadId}/messages`),

  addMessage: (threadId: string, message: unknown) =>
    client.post<ChatMessage>(`/api/threads/${threadId}/messages`, message),
};
