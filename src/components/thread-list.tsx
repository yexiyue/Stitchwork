/**
 * ThreadList 组件 - 移动端友好的会话列表
 * 使用 assistant-ui primitives + antd-mobile 组件
 */
import { forwardRef, useState, useRef } from "react";
import { Popup, SwipeAction, Dialog, Empty, DotLoading } from "antd-mobile";
import type { SwipeActionRef } from "antd-mobile/es/components/swipe-action";
import {
  ThreadListPrimitive,
  ThreadListItemPrimitive,
  useAssistantState,
} from "@assistant-ui/react";
import {
  MessageSquarePlus,
  MessageSquare,
  Archive,
  ArchiveRestore,
  Trash2,
  X,
} from "lucide-react";
import { cn } from "@/lib/utils";

interface ThreadListProps {
  open: boolean;
  onClose: () => void;
}

export const ThreadList = forwardRef<HTMLDivElement, ThreadListProps>(
  ({ open, onClose }, ref) => {
    const [showArchived, setShowArchived] = useState(false);

    return (
      <Popup
        visible={open}
        onMaskClick={onClose}
        position="left"
        bodyStyle={{
          width: "85vw",
          maxWidth: "320px",
          height: "100vh",
          borderTopRightRadius: "16px",
          borderBottomRightRadius: "16px",
        }}
      >
        <ThreadListPrimitive.Root
          ref={ref}
          className="flex h-full flex-col bg-background"
        >
          {/* 头部 */}
          <div className="flex h-14 shrink-0 items-center justify-between border-b px-4">
            <h2 className="font-semibold text-lg">会话历史</h2>
            <button
              onClick={onClose}
              className="flex size-8 items-center justify-center rounded-full hover:bg-muted"
            >
              <X size={18} />
            </button>
          </div>

          {/* 新建会话按钮 */}
          <div className="shrink-0 border-b p-3">
            <ThreadListPrimitive.New asChild>
              <button
                onClick={onClose}
                className="flex w-full items-center gap-3 rounded-xl bg-primary px-4 py-3 text-primary-foreground transition-colors hover:bg-primary/90"
              >
                <MessageSquarePlus size={20} />
                <span className="font-medium">新建会话</span>
              </button>
            </ThreadListPrimitive.New>
          </div>

          {/* 标签切换 */}
          <div className="flex shrink-0 gap-2 border-b p-3">
            <button
              onClick={() => setShowArchived(false)}
              className={cn(
                "flex-1 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                !showArchived
                  ? "bg-muted text-foreground"
                  : "text-muted-foreground hover:bg-muted/50"
              )}
            >
              活跃会话
            </button>
            <button
              onClick={() => setShowArchived(true)}
              className={cn(
                "flex-1 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                showArchived
                  ? "bg-muted text-foreground"
                  : "text-muted-foreground hover:bg-muted/50"
              )}
            >
              已归档
            </button>
          </div>

          {/* 会话列表 */}
          <div className="flex-1 overflow-y-auto">
            <ThreadListPrimitive.Items
              archived={showArchived}
              components={{
                ThreadListItem: () => (
                  <ThreadListItem
                    onClose={onClose}
                    archived={showArchived}
                  />
                ),
              }}
            />
            <ThreadListEmpty archived={showArchived} />
          </div>
        </ThreadListPrimitive.Root>
      </Popup>
    );
  }
);

ThreadList.displayName = "ThreadList";

/**
 * 单个会话项 - 支持滑动操作
 */
function ThreadListItem({
  onClose,
  archived,
}: {
  onClose: () => void;
  archived: boolean;
}) {
  const swipeRef = useRef<SwipeActionRef>(null);
  const archiveRef = useRef<HTMLButtonElement>(null);
  const unarchiveRef = useRef<HTMLButtonElement>(null);
  const deleteRef = useRef<HTMLButtonElement>(null);

  const rightActions = archived
    ? [
        {
          key: "unarchive",
          text: <ArchiveRestore size={18} />,
          color: "primary" as const,
          onClick: () => {
            unarchiveRef.current?.click();
            swipeRef.current?.close();
          },
        },
        {
          key: "delete",
          text: <Trash2 size={18} />,
          color: "danger" as const,
          onClick: async () => {
            const result = await Dialog.confirm({
              title: "永久删除",
              content: "确定要永久删除这个会话吗？此操作不可撤销。",
            });
            if (result) {
              deleteRef.current?.click();
            }
            swipeRef.current?.close();
          },
        },
      ]
    : [
        {
          key: "archive",
          text: <Archive size={18} />,
          color: "warning" as const,
          onClick: () => {
            archiveRef.current?.click();
            swipeRef.current?.close();
          },
        },
        {
          key: "delete",
          text: <Trash2 size={18} />,
          color: "danger" as const,
          onClick: async () => {
            const result = await Dialog.confirm({
              title: "删除会话",
              content: "确定要删除这个会话吗？此操作不可撤销。",
            });
            if (result) {
              deleteRef.current?.click();
            }
            swipeRef.current?.close();
          },
        },
      ];

  return (
    <SwipeAction ref={swipeRef} rightActions={rightActions}>
      <ThreadListItemPrimitive.Root
        className={cn(
          "group relative border-b transition-colors data-[active=true]:bg-muted/50",
          archived && "opacity-60"
        )}
      >
        <ThreadListItemPrimitive.Trigger
          onClick={onClose}
          className="flex w-full items-center gap-3 px-4 py-3 text-left"
        >
          {archived ? (
            <Archive size={18} className="shrink-0 text-muted-foreground" />
          ) : (
            <MessageSquare
              size={18}
              className="shrink-0 text-muted-foreground group-data-[active=true]:text-primary"
            />
          )}
          <span className="flex-1 truncate text-sm group-data-[active=true]:font-medium">
            <ThreadListItemPrimitive.Title />
          </span>
        </ThreadListItemPrimitive.Trigger>

        {/* 隐藏的操作按钮，供 SwipeAction 调用 */}
        {archived ? (
          <ThreadListItemPrimitive.Unarchive ref={unarchiveRef} className="hidden" />
        ) : (
          <ThreadListItemPrimitive.Archive ref={archiveRef} className="hidden" />
        )}
        <ThreadListItemPrimitive.Delete ref={deleteRef} className="hidden" />
      </ThreadListItemPrimitive.Root>
    </SwipeAction>
  );
}

/**
 * 空状态组件
 */
function ThreadListEmpty({ archived }: { archived: boolean }) {
  // 使用 useAssistantState 避免无限循环，只订阅 threads 数组长度
  const threadCount = useAssistantState((state) => {
    const threadIds = archived ? state.threads.archivedThreadIds : state.threads.threadIds;
    return threadIds?.length ?? -1; // -1 表示加载中
  });

  if (threadCount === -1) {
    return (
      <div className="flex h-32 items-center justify-center">
        <DotLoading color="primary" />
      </div>
    );
  }

  if (threadCount > 0) {
    return null;
  }

  return (
    <Empty
      className="py-12"
      description={archived ? "暂无已归档会话" : "暂无会话，开始新对话吧"}
      imageStyle={{ width: 64 }}
    />
  );
}
