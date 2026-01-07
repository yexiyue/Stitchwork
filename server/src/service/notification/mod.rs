mod controller;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

pub use controller::router;

/// 通知类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Notification {
    /// 员工提交计件 → 通知老板
    RecordSubmitted {
        user_name: String,
        process_name: String,
        quantity: i32,
    },
    /// 老板审批通过 → 通知员工
    RecordApproved {
        process_name: String,
        quantity: i32,
        amount: String,
    },
    /// 老板审批拒绝 → 通知员工
    RecordRejected {
        process_name: String,
        quantity: i32,
    },
    /// 工资发放 → 通知员工
    PayrollReceived { amount: String },
    /// 新用户注册 → 通知超管
    UserRegistered { username: String, phone: String },
    /// 员工加入工坊 → 通知老板
    StaffJoined { username: String, phone: String },
}

impl Notification {
    pub fn title(&self) -> &str {
        match self {
            Self::RecordSubmitted { .. } => "新计件待审核",
            Self::RecordApproved { .. } => "计件已通过",
            Self::RecordRejected { .. } => "计件被拒绝",
            Self::PayrollReceived { .. } => "收到工资",
            Self::UserRegistered { .. } => "新用户注册",
            Self::StaffJoined { .. } => "新员工加入",
        }
    }

    pub fn body(&self) -> String {
        match self {
            Self::RecordSubmitted {
                user_name,
                process_name,
                quantity,
            } => format!("{}提交了{}计件 {}件", user_name, process_name, quantity),
            Self::RecordApproved {
                process_name,
                quantity,
                amount,
            } => format!("{}工序 {}件 ¥{}", process_name, quantity, amount),
            Self::RecordRejected {
                process_name,
                quantity,
            } => format!("{}工序 {}件", process_name, quantity),
            Self::PayrollReceived { amount } => format!("¥{}", amount),
            Self::UserRegistered { username, phone } => format!("{} ({})", username, phone),
            Self::StaffJoined { username, phone } => format!("{} ({}) 加入了工坊", username, phone),
        }
    }
}

/// 通知服务 - 管理用户订阅和消息广播
pub struct Notifier {
    /// 每个用户一个 broadcast channel
    channels: DashMap<Uuid, broadcast::Sender<Notification>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            channels: DashMap::new(),
        }
    }

    /// 用户订阅通知
    pub fn subscribe(&self, user_id: Uuid) -> broadcast::Receiver<Notification> {
        self.channels
            .entry(user_id)
            .or_insert_with(|| broadcast::channel(16).0)
            .subscribe()
    }

    /// 发送通知给指定用户
    pub fn send(&self, user_id: Uuid, notification: Notification) {
        if let Some(sender) = self.channels.get(&user_id) {
            let _ = sender.send(notification);
        }
    }

    /// 发送通知给多个用户
    pub fn send_many(&self, user_ids: &[Uuid], notification: Notification) {
        for user_id in user_ids {
            self.send(*user_id, notification.clone());
        }
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}

/// 包装类型，方便共享
pub type SharedNotifier = Arc<Notifier>;
