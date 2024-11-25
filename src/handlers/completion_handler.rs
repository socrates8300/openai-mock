use crate::models::{Choice, CompletionRequest, CompletionResponse, Usage};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

pub async fn completions_handler(
    req: web::Json<CompletionRequest>,
) -> impl Responder {
    // Validate the required fields
    if req.model.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": {
                "message": "`model` is required",
                "type": "invalid_request_error",
                "param": "model",
                "code": null,
            }
        }));
    }

    // Mock processing logic
    let prompt = req.prompt.clone().unwrap_or_default();
    let max_tokens = req.max_tokens.unwrap_or(16);
    let n = req.n.unwrap_or(1);
    let created_time = Utc::now().timestamp() as u64;

    let choices: Vec<Choice> = (0..n)
        .map(|i| Choice {
            text: format!("Mock response {} for prompt: {}", i + 1, prompt),
            index: i,
            logprobs: None,
            finish_reason: Some("stop".to_string()),
        })
        .collect();

    let response = CompletionResponse {
        id: format!("cmpl-mock-id-{}", Uuid::new_v4()),
        object: "text_completion".to_string(),
        created: created_time,
        model: req.model.clone(),
        choices,
        usage: Usage {
            prompt_tokens: count_tokens(&prompt),
            completion_tokens: max_tokens,
            total_tokens: count_tokens(&prompt) + max_tokens,
        },
    };
    HttpResponse::Ok().json(response)
}

/// Helper function to count tokens in a string (mock implementation).
fn count_tokens(text: &str) -> u32 {
    // This is a placeholder. In a real scenario, you might use a tokenizer.
    text.split_whitespace().count() as u32
}