use reqwest::blocking::{Client};
use serde::{Deserialize, Serialize};
use log::{error, debug};

// 定义消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

// API请求数据结构
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}


// 定义完整的响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaChatResponse {
    pub model: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub message: ChatMessage,
    #[serde(rename = "done_reason")]
    pub done_reason: Option<String>, // 可能为 null 或不存在
    pub done: bool,
    #[serde(rename = "total_duration")]
    pub total_duration: u64,
    #[serde(rename = "load_duration")]
    pub load_duration: u64,
    #[serde(rename = "prompt_eval_count")]
    pub prompt_eval_count: u32,
    #[serde(rename = "prompt_eval_duration")]
    pub prompt_eval_duration: u64,
    #[serde(rename = "eval_count")]
    pub eval_count: u32,
    #[serde(rename = "eval_duration")]
    pub eval_duration: u64,
}

// 提取 <think> 和最终答案
fn extract_think_and_answer(content: &str) -> Option<(String, String)> {
    let think_start = "<think>";
    let think_end = "</think>";
    
    if let Some(start_idx) = content.find(think_start) {
        /* This is for thinking model */
        let think_start = start_idx + think_start.len();
        if let Some(end_idx) = content[think_start..].find(think_end) {
            let think_content = content[think_start..think_start + end_idx].trim().to_string();
            let answer = content[think_start + end_idx + think_end.len()..].trim().to_string();
            return Some((think_content, answer));
        }
    } else {
        /* this is for no thinking model */
        return Some(("".to_string(), content.trim().to_string()));
    }
    
    None
}

pub fn call(model:String, _api_key: String, msgs: Vec<ChatMessage>) -> Result<(), Box<dyn std::error::Error>> 
{
    // 构建请求
    let endpoint = "http://localhost:11434/api/chat";
    let client = Client::new();
    let request = ChatRequest {
        model: model,
        messages: msgs,
        stream: false,
    };

    debug!("ChatRequest: {:?}", request);
    
    let response_json = client
        .post(endpoint)
        .json(&request)
        .send()?
        .text()?;

    debug!("ChatResponse: {}", response_json);

    match serde_json::from_str::<OllamaChatResponse>(&response_json) {
        Ok(response) => {
            // 提取思考过程
            if let Some((__think, answer)) = extract_think_and_answer(&response.message.content) {
                debug!("翻译结果:");
                println!("{}", answer);
            }
        }
        Err(e) => {
            error!("Fail to get response: {}", e);
            // 调试用：尝试解析为 Value
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&response_json) {
                error!("original JSON value: {:#?}", value);
            }
        }
    }

    return Ok(());
}

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let msgs: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "What is the meaning of life?".to_string(),
        },
    ];
    let resp = call("deepseek-r1:8b".to_string(),
                        "Do not need".to_string(),
                        msgs)?;
    println!("{:?}", resp);

    Ok(())
}
