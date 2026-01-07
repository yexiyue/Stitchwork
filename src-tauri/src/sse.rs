use futures::StreamExt;
use log::{debug, info, warn};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::Mutex;

/// 通知类型（与后端保持一致）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Notification {
    RecordSubmitted {
        user_name: String,
        process_name: String,
        quantity: i32,
    },
    RecordApproved {
        process_name: String,
        quantity: i32,
        amount: String,
    },
    RecordRejected {
        process_name: String,
        quantity: i32,
    },
    PayrollReceived {
        amount: String,
    },
    UserRegistered {
        username: String,
        phone: String,
    },
    StaffJoined {
        username: String,
        phone: String,
    },
}

/// SSE 响应 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    #[serde(flatten)]
    pub notification: Notification,
    pub title: String,
    pub body: String,
}

/// SSE 连接状态
pub struct SseState {
    cancel_token: Option<tokio::sync::watch::Sender<bool>>,
    channel_id: Option<String>,
}

impl Default for SseState {
    fn default() -> Self {
        Self {
            cancel_token: None,
            channel_id: None,
        }
    }
}

pub type SharedSseState = Arc<Mutex<SseState>>;

/// 启动 SSE 连接
async fn start_sse(
    app_handle: AppHandle,
    api_url: String,
    token: String,
    channel_id: Option<String>,
    mut cancel_rx: tokio::sync::watch::Receiver<bool>,
) {
    let url = format!("{}/api/sse/events", api_url);
    let client = reqwest::Client::new();

    loop {
        // 检查是否需要取消
        if *cancel_rx.borrow() {
            info!("SSE connection cancelled");
            break;
        }

        info!("Connecting to SSE: {}", url);
        let request = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token));
        let mut es = EventSource::new(request).unwrap();

        loop {
            tokio::select! {
                _ = cancel_rx.changed() => {
                    if *cancel_rx.borrow() {
                        info!("SSE connection cancelled");
                        es.close();
                        return;
                    }
                }
                event = es.next() => {
                    match event {
                        Some(Ok(Event::Open)) => {
                            info!("SSE connection opened");
                        }
                        Some(Ok(Event::Message(msg))) => {
                            debug!("SSE message: {}", msg.data);
                            if let Ok(payload) = serde_json::from_str::<NotificationPayload>(&msg.data) {
                                // 发送本地通知
                                let mut builder = app_handle
                                    .notification()
                                    .builder()
                                    .title(&payload.title)
                                    .body(&payload.body);

                                // 使用高优先级渠道
                                if let Some(ref ch_id) = channel_id {
                                    builder = builder.channel_id(ch_id);
                                }

                                if let Err(e) = builder.show() {
                                    warn!("Failed to show notification: {}", e);
                                }

                                // 通知前端刷新数据
                                if let Err(e) = app_handle.emit("notification", &payload) {
                                    warn!("Failed to emit notification event: {}", e);
                                }
                            }
                        }
                        Some(Err(e)) => {
                            warn!("SSE error: {}", e);
                            es.close();
                            break; // 退出内层循环，尝试重连
                        }
                        None => {
                            info!("SSE stream ended");
                            break; // 退出内层循环，尝试重连
                        }
                    }
                }
            }
        }

        // 检查是否需要取消
        if *cancel_rx.borrow() {
            break;
        }

        // 等待 5 秒后重连
        info!("Reconnecting in 5 seconds...");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

/// 连接 SSE
#[tauri::command]
pub async fn connect_sse(
    app_handle: AppHandle,
    state: tauri::State<'_, SharedSseState>,
    api_url: String,
    token: String,
    channel_id: Option<String>,
) -> Result<(), String> {
    let mut sse_state = state.lock().await;

    // 如果已有连接，先取消
    if let Some(cancel_tx) = sse_state.cancel_token.take() {
        let _ = cancel_tx.send(true);
    }

    // 创建新的取消令牌
    let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);
    sse_state.cancel_token = Some(cancel_tx);
    sse_state.channel_id = channel_id.clone();

    // 启动 SSE 任务
    let handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        start_sse(handle, api_url, token, channel_id, cancel_rx).await;
    });

    Ok(())
}

/// 断开 SSE 连接
#[tauri::command]
pub async fn disconnect_sse(state: tauri::State<'_, SharedSseState>) -> Result<(), String> {
    let mut sse_state = state.lock().await;

    if let Some(cancel_tx) = sse_state.cancel_token.take() {
        let _ = cancel_tx.send(true);
        info!("SSE disconnected");
    }

    Ok(())
}
