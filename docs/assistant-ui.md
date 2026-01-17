# Assistant-UI 使用教程

Assistant-UI 是一个基于 React 的 AI 聊天界面库，提供类似 ChatGPT 的用户体验。本教程基于项目实际使用情况，帮助你快速上手。

## 目录

- [概述](#概述)
- [安装与配置](#安装与配置)
- [核心概念](#核心概念)
- [组件架构](#组件架构)
- [Thread 组件详解](#thread-组件详解)
- [Markdown 渲染](#markdown-渲染)
- [附件处理](#附件处理)
- [工具调用展示](#工具调用展示)
- [Runtime 集成](#runtime-集成)
- [样式定制](#样式定制)
- [最佳实践](#最佳实践)

## 概述

### 什么是 Assistant-UI

Assistant-UI 是一个开源的 TypeScript/React 库，核心设计理念是**基于 Primitives（原语）的组合式架构**：

- **Primitives**: 底层无样式的构建块，提供功能但不强制 UI
- **组合优于配置**: 通过组合小组件构建复杂界面
- **状态管理内置**: 自带 Runtime 处理 AI 后端通信

### 与其他库的区别

| 特性 | Assistant-UI | 传统组件库 |
|------|-------------|-----------|
| 架构 | Primitives 组合 | Props 配置 |
| 样式 | 完全自定义 | 主题覆盖 |
| 灵活性 | 高 | 中 |
| 学习曲线 | 中等 | 低 |

## 安装与配置

### 1. 安装依赖

```bash
# 核心库
pnpm add @assistant-ui/react

# Markdown 渲染支持
pnpm add @assistant-ui/react-markdown remark-gfm

# UI 基础组件 (Radix UI)
pnpm add @radix-ui/react-avatar @radix-ui/react-dialog @radix-ui/react-tooltip @radix-ui/react-slot

# 工具库
pnpm add class-variance-authority clsx tailwind-merge lucide-react
```

### 2. shadcn/ui 配置

创建 `components.json`:

```json
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "new-york",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "src/index.css",
    "baseColor": "neutral",
    "cssVariables": true
  },
  "iconLibrary": "lucide",
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui"
  }
}
```

### 3. 工具函数

创建 `src/lib/utils.ts`:

```typescript
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

## 核心概念

### Primitives（原语）

Assistant-UI 的核心是一组 Primitives，它们是无样式的功能组件：

```typescript
import {
  ThreadPrimitive,      // 聊天线程
  MessagePrimitive,     // 消息
  ComposerPrimitive,    // 输入框
  ActionBarPrimitive,   // 操作栏
  BranchPickerPrimitive,// 分支选择器
  AttachmentPrimitive,  // 附件
} from "@assistant-ui/react";
```

### Runtime（运行时）

Runtime 负责状态管理和 AI 后端通信：

```typescript
import { AssistantRuntimeProvider } from "@assistant-ui/react";
import { useChatRuntime } from "@assistant-ui/react-ai-sdk";

function App() {
  const runtime = useChatRuntime({ api: "/api/chat" });

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <Thread />
    </AssistantRuntimeProvider>
  );
}
```

### 状态访问

使用 hooks 访问当前状态：

```typescript
import { useAssistantState, useAssistantApi } from "@assistant-ui/react";

// 访问状态
const isRunning = useAssistantState(({ thread }) => thread.isRunning);

// 访问 API
const api = useAssistantApi();
api.composer.send();
```

### 条件渲染

`AssistantIf` 根据状态条件渲染：

```typescript
import { AssistantIf } from "@assistant-ui/react";

// 线程为空时显示欢迎界面
<AssistantIf condition={({ thread }) => thread.isEmpty}>
  <ThreadWelcome />
</AssistantIf>

// AI 运行中显示停止按钮
<AssistantIf condition={({ thread }) => thread.isRunning}>
  <StopButton />
</AssistantIf>
```

## 组件架构

项目中的 assistant-ui 组件结构：

```
src/components/
├── thread.tsx              # 主聊天组件
├── markdown-text.tsx       # AI 响应 Markdown 渲染
├── attachment.tsx          # 文件附件处理
├── tool-fallback.tsx       # 工具调用结果展示
├── tooltip-icon-button.tsx # 带提示的图标按钮
└── ui/                     # 基础 UI 组件
    ├── avatar.tsx
    ├── button.tsx
    ├── dialog.tsx
    └── tooltip.tsx
```

## Thread 组件详解

Thread 是聊天界面的主容器，由多个子组件组成：

### 基本结构

```typescript
export const Thread: FC = () => {
  return (
    <ThreadPrimitive.Root
      className="aui-thread-root"
      style={{ ["--thread-max-width" as string]: "44rem" }}
    >
      <ThreadPrimitive.Viewport turnAnchor="top">
        {/* 欢迎界面 - 线程为空时显示 */}
        <AssistantIf condition={({ thread }) => thread.isEmpty}>
          <ThreadWelcome />
        </AssistantIf>

        {/* 消息列表 */}
        <ThreadPrimitive.Messages
          components={{
            UserMessage,      // 用户消息
            EditComposer,     // 编辑模式
            AssistantMessage, // AI 消息
          }}
        />

        {/* 底部输入区 */}
        <ThreadPrimitive.ViewportFooter>
          <ThreadScrollToBottom />
          <Composer />
        </ThreadPrimitive.ViewportFooter>
      </ThreadPrimitive.Viewport>
    </ThreadPrimitive.Root>
  );
};
```

### ThreadWelcome - 欢迎界面

```typescript
const ThreadWelcome: FC = () => {
  return (
    <div className="aui-thread-welcome-root">
      <h1 className="animate-in fade-in slide-in-from-bottom-1">
        Hello there!
      </h1>
      <p>How can I help you today?</p>
      <ThreadSuggestions />
    </div>
  );
};
```

### ThreadSuggestions - 建议提示

使用 `ThreadPrimitive.Suggestion` 创建点击即发送的建议：

```typescript
const SUGGESTIONS = [
  { title: "What's the weather", prompt: "What's the weather in SF?" },
  { title: "Explain React hooks", prompt: "Explain useState and useEffect" },
];

const ThreadSuggestions: FC = () => {
  return (
    <div className="grid @md:grid-cols-2 gap-2">
      {SUGGESTIONS.map((s) => (
        <ThreadPrimitive.Suggestion prompt={s.prompt} send asChild>
          <Button variant="ghost">{s.title}</Button>
        </ThreadPrimitive.Suggestion>
      ))}
    </div>
  );
};
```

### Composer - 消息输入

```typescript
const Composer: FC = () => {
  return (
    <ComposerPrimitive.Root>
      {/* 拖拽上传区域 */}
      <ComposerPrimitive.AttachmentDropzone>
        <ComposerAttachments />
        <ComposerPrimitive.Input
          placeholder="Send a message..."
          autoFocus
        />
        <ComposerAction />
      </ComposerPrimitive.AttachmentDropzone>
    </ComposerPrimitive.Root>
  );
};

const ComposerAction: FC = () => {
  return (
    <div className="flex items-center justify-between">
      <ComposerAddAttachment />

      {/* 非运行时显示发送按钮 */}
      <AssistantIf condition={({ thread }) => !thread.isRunning}>
        <ComposerPrimitive.Send asChild>
          <Button>Send</Button>
        </ComposerPrimitive.Send>
      </AssistantIf>

      {/* 运行时显示取消按钮 */}
      <AssistantIf condition={({ thread }) => thread.isRunning}>
        <ComposerPrimitive.Cancel asChild>
          <Button>Stop</Button>
        </ComposerPrimitive.Cancel>
      </AssistantIf>
    </div>
  );
};
```

### UserMessage - 用户消息

```typescript
const UserMessage: FC = () => {
  return (
    <MessagePrimitive.Root data-role="user">
      <UserMessageAttachments />
      <div className="aui-user-message-content">
        <MessagePrimitive.Parts />
      </div>
      <UserActionBar />
      <BranchPicker />
    </MessagePrimitive.Root>
  );
};
```

### AssistantMessage - AI 消息

```typescript
const AssistantMessage: FC = () => {
  return (
    <MessagePrimitive.Root data-role="assistant">
      <div className="aui-assistant-message-content">
        {/* 消息内容：文本用 MarkdownText，工具用 ToolFallback */}
        <MessagePrimitive.Parts
          components={{
            Text: MarkdownText,
            tools: { Fallback: ToolFallback },
          }}
        />
        <MessageError />
      </div>
      <BranchPicker />
      <AssistantActionBar />
    </MessagePrimitive.Root>
  );
};
```

### AssistantActionBar - AI 操作栏

```typescript
const AssistantActionBar: FC = () => {
  return (
    <ActionBarPrimitive.Root
      hideWhenRunning        // 运行时隐藏
      autohide="not-last"    // 非最后一条自动隐藏
      autohideFloat="single-branch"
    >
      {/* 复制按钮 */}
      <ActionBarPrimitive.Copy asChild>
        <TooltipIconButton tooltip="Copy">
          <AssistantIf condition={({ message }) => message.isCopied}>
            <CheckIcon />
          </AssistantIf>
          <AssistantIf condition={({ message }) => !message.isCopied}>
            <CopyIcon />
          </AssistantIf>
        </TooltipIconButton>
      </ActionBarPrimitive.Copy>

      {/* 重新生成 */}
      <ActionBarPrimitive.Reload asChild>
        <TooltipIconButton tooltip="Refresh">
          <RefreshCwIcon />
        </TooltipIconButton>
      </ActionBarPrimitive.Reload>

      {/* 更多操作菜单 */}
      <ActionBarMorePrimitive.Root>
        <ActionBarMorePrimitive.Trigger asChild>
          <TooltipIconButton tooltip="More">
            <MoreHorizontalIcon />
          </TooltipIconButton>
        </ActionBarMorePrimitive.Trigger>
        <ActionBarMorePrimitive.Content>
          <ActionBarPrimitive.ExportMarkdown asChild>
            <ActionBarMorePrimitive.Item>
              <DownloadIcon /> Export as Markdown
            </ActionBarMorePrimitive.Item>
          </ActionBarPrimitive.ExportMarkdown>
        </ActionBarMorePrimitive.Content>
      </ActionBarMorePrimitive.Root>
    </ActionBarPrimitive.Root>
  );
};
```

### BranchPicker - 分支选择器

用于在消息的不同版本间切换：

```typescript
const BranchPicker: FC = () => {
  return (
    <BranchPickerPrimitive.Root hideWhenSingleBranch>
      <BranchPickerPrimitive.Previous asChild>
        <TooltipIconButton tooltip="Previous">
          <ChevronLeftIcon />
        </TooltipIconButton>
      </BranchPickerPrimitive.Previous>

      <span>
        <BranchPickerPrimitive.Number /> / <BranchPickerPrimitive.Count />
      </span>

      <BranchPickerPrimitive.Next asChild>
        <TooltipIconButton tooltip="Next">
          <ChevronRightIcon />
        </TooltipIconButton>
      </BranchPickerPrimitive.Next>
    </BranchPickerPrimitive.Root>
  );
};
```

### EditComposer - 编辑模式

用户可以编辑已发送的消息：

```typescript
const EditComposer: FC = () => {
  return (
    <MessagePrimitive.Root>
      <ComposerPrimitive.Root>
        <ComposerPrimitive.Input autoFocus />
        <div className="flex gap-2">
          <ComposerPrimitive.Cancel asChild>
            <Button variant="ghost">Cancel</Button>
          </ComposerPrimitive.Cancel>
          <ComposerPrimitive.Send asChild>
            <Button>Update</Button>
          </ComposerPrimitive.Send>
        </div>
      </ComposerPrimitive.Root>
    </MessagePrimitive.Root>
  );
};
```

## Markdown 渲染

使用 `@assistant-ui/react-markdown` 渲染 AI 响应：

### 基本配置

```typescript
import "@assistant-ui/react-markdown/styles/dot.css";
import {
  MarkdownTextPrimitive,
  unstable_memoizeMarkdownComponents as memoizeMarkdownComponents,
  useIsMarkdownCodeBlock,
} from "@assistant-ui/react-markdown";
import remarkGfm from "remark-gfm";

const MarkdownText = memo(() => {
  return (
    <MarkdownTextPrimitive
      remarkPlugins={[remarkGfm]}  // GitHub 风格 Markdown
      className="aui-md"
      components={defaultComponents}
    />
  );
});
```

### 自定义组件

使用 `memoizeMarkdownComponents` 优化性能：

```typescript
const defaultComponents = memoizeMarkdownComponents({
  // 标题
  h1: ({ className, ...props }) => (
    <h1 className={cn("aui-md-h1 text-4xl font-extrabold", className)} {...props} />
  ),
  h2: ({ className, ...props }) => (
    <h2 className={cn("aui-md-h2 text-3xl font-semibold", className)} {...props} />
  ),

  // 段落
  p: ({ className, ...props }) => (
    <p className={cn("aui-md-p leading-7", className)} {...props} />
  ),

  // 代码块
  pre: ({ className, ...props }) => (
    <pre className={cn("aui-md-pre bg-black text-white p-4 rounded-b-lg", className)} {...props} />
  ),

  // 内联代码 - 使用 useIsMarkdownCodeBlock 区分
  code: function Code({ className, ...props }) {
    const isCodeBlock = useIsMarkdownCodeBlock();
    return (
      <code
        className={cn(
          !isCodeBlock && "aui-md-inline-code rounded border bg-muted",
          className,
        )}
        {...props}
      />
    );
  },

  // 代码块头部（显示语言 + 复制按钮）
  CodeHeader,

  // 表格
  table: ({ className, ...props }) => (
    <table className={cn("aui-md-table w-full", className)} {...props} />
  ),
  th: ({ className, ...props }) => (
    <th className={cn("aui-md-th bg-muted px-4 py-2", className)} {...props} />
  ),
  td: ({ className, ...props }) => (
    <td className={cn("aui-md-td border px-4 py-2", className)} {...props} />
  ),
});
```

### CodeHeader - 代码块头部

```typescript
const CodeHeader: FC<CodeHeaderProps> = ({ language, code }) => {
  const { isCopied, copyToClipboard } = useCopyToClipboard();

  return (
    <div className="flex items-center justify-between bg-muted px-4 py-2 rounded-t-lg">
      <span className="text-sm">{language}</span>
      <TooltipIconButton tooltip="Copy" onClick={() => copyToClipboard(code)}>
        {isCopied ? <CheckIcon /> : <CopyIcon />}
      </TooltipIconButton>
    </div>
  );
};
```

## 附件处理

### 核心 Hooks

```typescript
// 为 File 对象创建 Object URL
const useFileSrc = (file: File | undefined) => {
  const [src, setSrc] = useState<string | undefined>();

  useEffect(() => {
    if (!file) {
      setSrc(undefined);
      return;
    }
    const objectUrl = URL.createObjectURL(file);
    setSrc(objectUrl);
    return () => URL.revokeObjectURL(objectUrl);
  }, [file]);

  return src;
};

// 从 assistant 状态获取附件图片源
const useAttachmentSrc = () => {
  const { file, src } = useAssistantState(
    useShallow(({ attachment }) => {
      if (attachment.type !== "image") return {};
      if (attachment.file) return { file: attachment.file };
      const src = attachment.content?.filter((c) => c.type === "image")[0]?.image;
      return { src };
    }),
  );
  return useFileSrc(file) ?? src;
};
```

### 附件组件

```typescript
// 附件缩略图
const AttachmentThumb: FC = () => {
  const isImage = useAssistantState(({ attachment }) => attachment.type === "image");
  const src = useAttachmentSrc();

  return (
    <Avatar className="h-full w-full rounded-none">
      <AvatarImage src={src} alt="Attachment preview" />
      <AvatarFallback delayMs={isImage ? 200 : 0}>
        <FileText className="size-8 text-muted-foreground" />
      </AvatarFallback>
    </Avatar>
  );
};

// 附件 UI
const AttachmentUI: FC = () => {
  const api = useAssistantApi();
  const isComposer = api.attachment.source === "composer";

  return (
    <Tooltip>
      <AttachmentPrimitive.Root>
        <AttachmentPreviewDialog>
          <TooltipTrigger asChild>
            <div className="aui-attachment-tile">
              <AttachmentThumb />
            </div>
          </TooltipTrigger>
        </AttachmentPreviewDialog>
        {isComposer && <AttachmentRemove />}
      </AttachmentPrimitive.Root>
      <TooltipContent>
        <AttachmentPrimitive.Name />
      </TooltipContent>
    </Tooltip>
  );
};
```

### 导出组件

```typescript
// 用户消息附件
export const UserMessageAttachments: FC = () => (
  <div className="flex gap-2">
    <MessagePrimitive.Attachments components={{ Attachment: AttachmentUI }} />
  </div>
);

// 输入框附件
export const ComposerAttachments: FC = () => (
  <div className="flex gap-2 empty:hidden">
    <ComposerPrimitive.Attachments components={{ Attachment: AttachmentUI }} />
  </div>
);

// 添加附件按钮
export const ComposerAddAttachment: FC = () => (
  <ComposerPrimitive.AddAttachment asChild>
    <TooltipIconButton tooltip="Add Attachment">
      <PlusIcon />
    </TooltipIconButton>
  </ComposerPrimitive.AddAttachment>
);
```

## 工具调用展示

`ToolFallback` 组件用于展示 AI 工具调用的结果：

```typescript
import type { ToolCallMessagePartComponent } from "@assistant-ui/react";

export const ToolFallback: ToolCallMessagePartComponent = ({
  toolName,   // 工具名称
  argsText,   // 参数 JSON 字符串
  result,     // 执行结果
  status,     // 状态：running | complete | incomplete
}) => {
  const [isCollapsed, setIsCollapsed] = useState(true);

  const isCancelled = status?.type === "incomplete" && status.reason === "cancelled";

  return (
    <div className={cn(
      "aui-tool-fallback-root rounded-lg border py-3",
      isCancelled && "bg-muted/30"
    )}>
      {/* 头部：图标 + 工具名 + 展开按钮 */}
      <div className="flex items-center gap-2 px-4">
        {isCancelled ? <XCircleIcon /> : <CheckIcon />}
        <p>
          {isCancelled ? "Cancelled tool: " : "Used tool: "}
          <b>{toolName}</b>
        </p>
        <Button onClick={() => setIsCollapsed(!isCollapsed)}>
          {isCollapsed ? <ChevronUpIcon /> : <ChevronDownIcon />}
        </Button>
      </div>

      {/* 详情内容 */}
      {!isCollapsed && (
        <div className="border-t pt-2">
          {/* 取消原因 */}
          {isCancelled && status.error && (
            <div className="px-4">
              <p className="font-semibold">Cancelled reason:</p>
              <p>{JSON.stringify(status.error)}</p>
            </div>
          )}

          {/* 参数 */}
          <div className="px-4">
            <pre className="whitespace-pre-wrap">{argsText}</pre>
          </div>

          {/* 结果 */}
          {!isCancelled && result !== undefined && (
            <div className="border-t border-dashed px-4 pt-2">
              <p className="font-semibold">Result:</p>
              <pre className="whitespace-pre-wrap">
                {typeof result === "string" ? result : JSON.stringify(result, null, 2)}
              </pre>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
```

## Runtime 集成

### 使用 AI SDK (Vercel)

```typescript
import { AssistantRuntimeProvider } from "@assistant-ui/react";
import { useChatRuntime } from "@assistant-ui/react-ai-sdk";

export function ChatPage() {
  const runtime = useChatRuntime({
    api: "/api/chat",
    // 可选配置
    initialMessages: [],
    onFinish: (message) => console.log("Message completed:", message),
    onError: (error) => console.error("Chat error:", error),
    headers: { "Authorization": "Bearer xxx" },
    body: { model: "gpt-4" },
  });

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <Thread />
    </AssistantRuntimeProvider>
  );
}
```

### 使用 Data Stream Runtime

```typescript
import { useDataStreamRuntime } from "@assistant-ui/react-data-stream";

const runtime = useDataStreamRuntime({
  api: "/api/chat",
  initialMessages: [],
});
```

### 访问 Runtime API

```typescript
import { useAssistantRuntime } from "@assistant-ui/react";

function MyComponent() {
  const runtime = useAssistantRuntime();

  // 发送消息
  runtime.composer.setText("Hello");
  runtime.composer.send();

  // 添加附件
  runtime.composer.addAttachment(file);

  // 取消运行
  runtime.composer.cancel();

  // 获取线程列表
  runtime.getThreads();
}
```

## 样式定制

### CSS 类名规范

Assistant-UI 使用 `aui-` 前缀的 BEM 风格类名：

```css
/* 线程 */
.aui-thread-root { }
.aui-thread-viewport { }
.aui-thread-welcome-root { }

/* 消息 */
.aui-assistant-message-root { }
.aui-user-message-root { }
.aui-user-message-content { }

/* 输入框 */
.aui-composer-root { }
.aui-composer-input { }
.aui-composer-send { }

/* Markdown */
.aui-md { }
.aui-md-h1, .aui-md-h2, .aui-md-p { }
.aui-md-pre, .aui-md-inline-code { }
```

### CSS 变量

```css
/* 设置线程最大宽度 */
.aui-thread-root {
  --thread-max-width: 44rem;
}

/* 使用变量 */
.aui-assistant-message-root {
  max-width: var(--thread-max-width);
}
```

### Tailwind 动画

```typescript
// 消息进入动画
<MessagePrimitive.Root
  className="animate-in fade-in slide-in-from-bottom-1 duration-150"
>

// 建议卡片错开动画
{SUGGESTIONS.map((s, i) => (
  <div
    className="animate-in fade-in slide-in-from-bottom-2 fill-mode-both"
    style={{ animationDelay: `${100 + i * 50}ms` }}
  >
    ...
  </div>
))}
```

### 响应式设计

使用 `@container` 查询：

```typescript
<ThreadPrimitive.Root className="@container">
  {/* @md 是容器宽度断点 */}
  <div className="@md:grid-cols-2 grid-cols-1">
    ...
  </div>
</ThreadPrimitive.Root>
```

## 最佳实践

### 1. 组件 Memoization

```typescript
// Markdown 组件使用 memo
export const MarkdownText = memo(MarkdownTextImpl);

// Markdown 子组件使用 memoizeMarkdownComponents
const components = memoizeMarkdownComponents({
  h1: ...,
  p: ...,
});
```

### 2. 懒加载图片

```typescript
<Image
  src={src}
  alt="Preview"
  className={isLoaded ? "block" : "hidden"}
  onLoadingComplete={() => setIsLoaded(true)}
  priority={false}  // 非优先加载
/>
```

### 3. Object URL 清理

```typescript
useEffect(() => {
  if (!file) return;
  const url = URL.createObjectURL(file);
  return () => URL.revokeObjectURL(url);  // 清理防止内存泄漏
}, [file]);
```

### 4. TypeScript 类型安全

```typescript
// 使用 assistant-ui 提供的类型
import type { ToolCallMessagePartComponent } from "@assistant-ui/react";

export const ToolFallback: ToolCallMessagePartComponent = ({
  toolName,
  argsText,
  result,
  status,
}) => { ... };
```

### 5. 无障碍访问

```typescript
// 屏幕阅读器文本
<span className="sr-only">{tooltip}</span>

// aria-label
<Button aria-label="Send message">
  <ArrowUpIcon />
</Button>

// 对话框标题
<DialogTitle className="sr-only">Image Preview</DialogTitle>
```

### 6. asChild 模式

使用 Radix 的 `asChild` 将 Primitive 行为附加到自定义组件：

```typescript
// 将 Send 行为附加到自定义 Button
<ComposerPrimitive.Send asChild>
  <Button variant="default" size="icon">
    <ArrowUpIcon />
  </Button>
</ComposerPrimitive.Send>

// 将 ScrollToBottom 行为附加到 TooltipIconButton
<ThreadPrimitive.ScrollToBottom asChild>
  <TooltipIconButton tooltip="Scroll to bottom">
    <ArrowDownIcon />
  </TooltipIconButton>
</ThreadPrimitive.ScrollToBottom>
```

## 参考资源

- [Assistant-UI 官方文档](https://assistant-ui.com/docs)
- [GitHub 仓库](https://github.com/assistant-ui/assistant-ui)
- [shadcn/ui](https://ui.shadcn.com)
- [Radix UI](https://www.radix-ui.com)
