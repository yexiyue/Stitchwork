use crate::chat::request::ForwardedTools;
use crate::mcp;
use crate::service::auth::Claims;
use anyhow::Result;
use axum::response::sse::Event;
use entity::user::Role;
use futures::Stream;
use rig::agent::{Agent, StreamingPromptHook};
use rig::message::Message;
use rig::providers::anthropic::completion::CompletionModel;
use rig::streaming::StreamingChat;
use rig::{client::CompletionClient, providers::anthropic};
use rmcp::ServiceExt;
use rmcp::service::RunningService;
use sea_orm::DbConn;
use std::convert::Infallible;
use tokio::io::duplex;

// Re-export from rig-ai-sdk for convenience
pub use rig_ai_sdk::{
    AISdkEvent, AISdkStreamBuilder, UIMessage, UIMessagePart, adapt_rig_stream_sse,
    convert_message, convert_messages, extract_prompt_and_history,
};

pub mod frontend_tools;
pub mod knowledge_base;
pub mod request;
pub mod session_manager;

// Re-export business types
pub use request::AISdkChatRequest;

pub struct ChatSession {
    pub agent: Agent<CompletionModel>,
    pub mcp_client: RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
}

impl ChatSession {
    pub async fn new(
        db: &DbConn,
        client: &anthropic::Client,
        claims: Claims,
        forwarded_tools: Option<ForwardedTools>,
    ) -> Result<Self> {
        let (client_io, server_io) = duplex(1024 * 1024); // 1MB buffer for large tool responses
        let role = claims.role.clone();

        let mcp_client = match role {
            Role::Boss => {
                let mcp_server = mcp::boss::BossMcp::new(db.clone(), claims);
                tokio::spawn(async move {
                    match mcp_server.serve(server_io).await {
                        Ok(service) => {
                            if let Err(e) = service.waiting().await {
                                tracing::error!("BossMcp service error: {:?}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("BossMcp serve failed: {:?}", e);
                        }
                    }
                });

                rmcp::model::ClientInfo::default().serve(client_io).await?
            }
            Role::Staff => {
                let mcp_server = mcp::staff::StaffMcp::new(db.clone(), claims);

                tokio::spawn(async move {
                    match mcp_server.serve(server_io).await {
                        Ok(service) => {
                            if let Err(e) = service.waiting().await {
                                tracing::error!("StaffMcp service error: {:?}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("StaffMcp serve failed: {:?}", e);
                        }
                    }
                });

                rmcp::model::ClientInfo::default().serve(client_io).await?
            }
        };

        let tools = mcp_client.list_tools(None).await?;

        let forwarded_tools = match forwarded_tools {
            Some(tools) => tools,
            None => ForwardedTools {
                tools: Default::default(),
            },
        };

        let today = chrono::Local::now().format("%Y年%m月%d日").to_string();

        // 构建前端工具定义字符串，拼接到 system prompt 中

        let preamble = match role {
            Role::Boss => {
                format!(
                    r#"你是 StitchWork 服装加工管理助手（老板端）。

当前日期：{today}

你可以帮助老板：
- 查询和管理订单
- 查看计件记录和审批状态
- 统计员工产量和工资
- 查看订单进度和首页概览

回答时请简洁明了，数据展示时使用表格或列表格式。金额单位为元。
"#
                )
            }
            Role::Staff => {
                format!(
                    r#"你是 StitchWork 服装加工管理助手（员工端）。

当前日期：{today}

你可以帮助员工：
- 查询我的计件记录
- 查看收入统计和待审批金额
- 查询工资单
- 查看可接的工序任务

回答时请简洁明了，数据展示时使用表格或列表格式。金额单位为元。
"#
                )
            }
        };

        let agent = client
            .agent("glm-4.6")
            .max_tokens(10000)
            .tools((&forwarded_tools).into())
            // .dynamic_context(3, index)
            .rmcp_tools(tools.tools, mcp_client.peer().clone())
            .preamble(&preamble)
            .build();

        Ok(Self { agent, mcp_client })
    }

    pub async fn chat(
        &self,
        prompt: Message,
        frontend_tools: Option<ForwardedTools>,
        chat_history: Vec<Message>,
    ) -> impl Stream<Item = Result<Event, Infallible>> + use<> {
        let rig_stream = self
            .agent
            .stream_chat(prompt, chat_history)
            .with_hook(FrontendToolHook { tools: frontend_tools })
            .await;
        adapt_rig_stream_sse(rig_stream)
    }
}

#[derive(Debug, Clone)]
pub struct FrontendToolHook {
    pub tools: Option<ForwardedTools>,
}

/// 前端工具取消的特殊标记
pub const FRONTEND_TOOL_CANCEL_REASON: &str = "__FRONTEND_TOOL__";

impl<M> StreamingPromptHook<M> for FrontendToolHook
where
    M: rig::completion::CompletionModel,
{
    fn on_tool_call(
        &self,
        tool_name: &str,
        _tool_call_id: Option<String>,
        _args: &str,
        cancel_sig: rig::agent::CancelSignal,
    ) -> impl std::prelude::rust_2024::Future<Output = ()> + Send {
        tracing::info!("on_tool_call: tool_name={}, tools={:?}", tool_name, self.tools.as_ref().map(|t| t.tools.keys().collect::<Vec<_>>()));
        if let Some(tools) = self.tools.as_ref()
            && tools.get(tool_name).is_some()
        {
            tracing::info!("Cancelling frontend tool: {}", tool_name);
            cancel_sig.cancel_with_reason(FRONTEND_TOOL_CANCEL_REASON);
        }
        async {}
    }
}
