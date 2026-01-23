/**
 * RemoteThreadListAdapter 实现
 * 用于管理会话列表的 CRUD 操作
 */
import type { unstable_RemoteThreadListAdapter as RemoteThreadListAdapter } from "@assistant-ui/react";
import { chatApi } from "@/api";

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
   * 生成标题（暂不实现，返回空流）
   */
  async generateTitle(_remoteId: string, _messages: readonly unknown[]) {
    // TODO: 实现标题生成 API
    return new ReadableStream({
      start(controller) {
        controller.close();
      },
    }) as never;
  },
});
