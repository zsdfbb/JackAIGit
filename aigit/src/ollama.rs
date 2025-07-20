use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};

async fn call(model: String, assistant_prompt: String, user_prompt: String) -> Result<(String), Box<dyn std::error::Error>>
{
    let ollama = Ollama::default();

    let res = ollama.send_chat_messages(
        ChatMessageRequest::new(
            model,
            vec![ChatMessage::assistant(assistant_prompt), ChatMessage::user(user_prompt)],
        ),
    ).await?;

    let assistant_message = res.message.content;

    return Ok(assistant_message);
}