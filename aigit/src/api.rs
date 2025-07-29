use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

use crate::ollama::{self};

// 定义消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

// API请求数据结构
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

pub type ChatFn = fn(
    model: String,
    api_key: String,
    msgs: Vec<ChatMessage>,  // 假设 ChatMessage 是具体类型
) -> Result<(), Box<dyn std::error::Error>>;


lazy_static! {
    static ref CHAT_FN_MAP: Vec<(String, ChatFn)> = vec![
        ("ollama".to_string(), ollama::chat),
    ];
}
