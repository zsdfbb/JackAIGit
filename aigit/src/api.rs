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
) -> Result<String, Box<dyn std::error::Error>>;

pub fn dummy_chat_fn(
    _model: String,
    _api_key: String,
    _msgs: Vec<ChatMessage>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Dummy chat function called. Please specify the legal platform name.");
    Ok("Dummy response.".to_string())
}

lazy_static! {
    static ref CHAT_FN_MAP: Vec<(String, ChatFn)> = vec![
        ("ollama".to_string(), ollama::chat),
    ];
}

pub fn get_chat(platform: String) -> ChatFn {
    for (key, f) in CHAT_FN_MAP.iter() {
        if *key == platform {
            return *f;
        }
    }
    
    dummy_chat_fn
}

pub fn get_platform_list() -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    for (key, _) in CHAT_FN_MAP.iter() {
        ret.push(key.clone());
    }
    ret
}