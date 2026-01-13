use crate::chat::ai_sdk_adapter::AISdkEvent;
use crate::mcp;
use crate::service::auth::Claims;
use anyhow::Result;
use async_stream::stream;
use axum::response::sse::Event;
use entity::user::Role;
use futures::{Stream, StreamExt};
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::message::Message;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::streaming::{
    StreamedAssistantContent, StreamedUserContent, StreamingChat, ToolCallDeltaContent,
};
use rig::vector_store::in_memory_store::InMemoryVectorIndex;
use rig::{client::CompletionClient, providers::openai};
use rmcp::service::RunningService;
use rmcp::ServiceExt;
use sea_orm::DbConn;
use std::collections::HashMap;
use tokio::io::duplex;
pub mod ai_sdk_adapter;
pub mod knowledge_base;
pub mod session_manager;

pub struct ChatSession {
    pub agent: Agent<ResponsesCompletionModel>,
    pub mcp_client: RunningService<rmcp::RoleClient, rmcp::model::InitializeRequestParam>,
}

impl ChatSession {
    pub async fn new(
        db: &DbConn,
        client: &openai::Client,
        claims: Claims,
        // index: InMemoryVectorIndex<openai::EmbeddingModel, String>,
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

        let today = chrono::Local::now().format("%Y年%m月%d日").to_string();

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

回答时请简洁明了，数据展示时使用表格或列表格式。金额单位为元。"#
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

回答时请简洁明了，数据展示时使用表格或列表格式。金额单位为元。"#
                )
            }
        };

        let agent = client
            .agent("gpt-5-codex")
            // .dynamic_context(3, index)
            .rmcp_tools(tools.tools, mcp_client.peer().clone())
            .preamble(&preamble)
            .build();

        Ok(Self { agent, mcp_client })
    }

    pub async fn chat(
        &self,
        prompt: Message,
        chat_history: Vec<Message>,
    ) -> impl Stream<Item = Result<Event, anyhow::Error>> {
        tracing::debug!("chat: starting stream_chat");
        let mut res = self.agent.stream_chat(prompt, chat_history).await;
        tracing::debug!("chat: stream_chat returned");

        stream! {
            let mut events = ai_sdk_adapter::AISdkStreamBuilder::new();
            let mut tool_names: HashMap<String, String> = HashMap::new();

            tracing::debug!("chat: yielding start event");
            yield Ok(Event::from(events.start()));

            while let Some(msg) = res.next().await {
                tracing::debug!("chat: received stream item: {:?}", msg);
                let msg = msg?;
                match msg {
                    MultiTurnStreamItem::StreamUserItem(StreamedUserContent::ToolResult(result)) => {
                        let tool_call_id = result.call_id.as_ref().unwrap_or(&result.id);
                        yield Ok(Event::from(AISdkEvent::ToolOutputAvailable {
                            tool_call_id: tool_call_id.clone(),
                            output: serde_json::to_value(&result.content).unwrap_or_default(),
                        }));
                    }
                    MultiTurnStreamItem::StreamAssistantItem(assistant) => match assistant {
                        StreamedAssistantContent::Text(text) => {
                            if let Some(reasoning_end) = events.reasoning_end() {
                                yield Ok(Event::from(reasoning_end));
                                yield Ok(Event::from(events.text_start()));
                            }

                            if let Some(text_delta) = events.text_delta(text.text) {
                                yield Ok(Event::from(text_delta));
                            }
                        }
                        StreamedAssistantContent::ToolCall(tool_call) => {
                            let tool_call_id = tool_call.call_id.as_ref().unwrap_or(&tool_call.id);
                            yield Ok(Event::from(AISdkEvent::ToolInputAvailable {
                                tool_call_id: tool_call_id.clone(),
                                tool_name: tool_call.function.name,
                                input: tool_call.function.arguments,
                            }));
                        }
                        StreamedAssistantContent::ToolCallDelta { id, content } => {
                            match content {
                                ToolCallDeltaContent::Name(name) => {
                                    tool_names.insert(id.clone(), name.clone());
                                    yield Ok(Event::from(AISdkEvent::ToolInputStart {
                                        tool_call_id: id,
                                        tool_name: name,
                                    }));
                                }
                                ToolCallDeltaContent::Delta(delta) => {
                                    yield Ok(Event::from(AISdkEvent::ToolInputDelta {
                                        tool_call_id: id,
                                        delta,
                                    }));
                                }
                            }
                        }
                        StreamedAssistantContent::Reasoning(reasoning) => {
                            yield Ok(Event::from(events.reasoning_start()));
                            for item in &reasoning.reasoning {
                                if let Some(delta) = events.reasoning_delta(item) {
                                    yield Ok(Event::from(delta));
                                }
                            }
                        }
                        StreamedAssistantContent::ReasoningDelta { reasoning, .. } => {
                            if let Some(reasoning_delta) = events.reasoning_delta(reasoning) {
                                yield Ok(Event::from(reasoning_delta));
                            }
                        }
                        StreamedAssistantContent::Final(_) => {
                            if let Some(text_end) = events.text_end() {
                                yield Ok(Event::from(text_end));
                            }
                        }
                    },
                    MultiTurnStreamItem::FinalResponse(final_response) => {
                        yield Ok(Event::from(AISdkEvent::custom_data("usage", final_response.usage())));
                    }
                    _ => {}
                }
            }

            tracing::debug!("chat: stream finished");
            yield Ok(Event::from(events.finish()));
            yield Ok(Event::from(events.done()));
        }
    }
}
