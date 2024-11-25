use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Models for the request and response structures.

#[derive(Debug, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: Option<String>,
    #[serde(default = "default_suffix")]
    pub suffix: Option<String>,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: Option<u32>,
    #[serde(default = "default_temperature")]
    pub temperature: Option<f32>,
    #[serde(default = "default_top_p")]
    pub top_p: Option<f32>,
    #[serde(default = "default_n")]
    pub n: Option<u32>,
    #[serde(default = "default_stream")]
    pub stream: Option<bool>,
    pub logprobs: Option<u32>,
    #[serde(default = "default_echo")]
    pub echo: Option<bool>,
    pub stop: Option<Value>, // Can be a string or array of strings
    #[serde(default = "default_presence_penalty")]
    pub presence_penalty: Option<f32>,
    #[serde(default = "default_frequency_penalty")]
    pub frequency_penalty: Option<f32>,
    pub best_of: Option<u32>,
    pub logit_bias: Option<Value>,
    pub user: Option<String>,
}

fn default_suffix() -> Option<String> {
    None
}

fn default_max_tokens() -> Option<u32> {
    Some(16)
}

fn default_temperature() -> Option<f32> {
    Some(1.0)
}

fn default_top_p() -> Option<f32> {
    Some(1.0)
}

fn default_n() -> Option<u32> {
    Some(1)
}

fn default_stream() -> Option<bool> {
    Some(false)
}

fn default_echo() -> Option<bool> {
    Some(false)
}

fn default_presence_penalty() -> Option<f32> {
    Some(0.0)
}

fn default_frequency_penalty() -> Option<f32> {
    Some(0.0)
}

#[derive(Debug, Serialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<Value>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// The module provides functions to create mock handlers.

#[cfg(feature = "actix-web")]
pub mod server {
    use super::*;
    use actix_web::{web, HttpResponse, Responder};
    use serde_json::json;

    /// Handler for the `/v1/completions` endpoint.
    pub async fn completions_handler(
        req: web::Json<CompletionRequest>,
    ) -> impl Responder {
        // Validate the required fields
        if req.model.is_empty() {
            return HttpResponse::BadRequest().json(json!({
                "error": {
                    "message": "`model` is required",
                    "type": "invalid_request_error",
                    "param": "model",
                    "code": None::<String>,
                }
            }));
        }

        // Mock processing logic
        let prompt = req.prompt.clone().unwrap_or_default();
        let max_tokens = req.max_tokens.unwrap_or(16);
        let n = req.n.unwrap_or(1);
        let created_time = chrono::Utc::now().timestamp() as u64;

        let choices: Vec<Choice> = (0..n)
            .map(|i| Choice {
                text: format!("Mock response {} for prompt: {}", i + 1, prompt),
                index: i,
                logprobs: None,
                finish_reason: Some("stop".to_string()),
            })
            .collect();

        let response = CompletionResponse {
            id: format!("cmpl-mock-id-{}", uuid::Uuid::new_v4()),
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

    /// Creates an Actix `App` with the mock OpenAI endpoints.
    pub fn create_mock_app() -> impl actix_web::dev::HttpServiceFactory {
        web::scope("")
            .route("/v1/completions", web::post().to(completions_handler))
        // Add more routes as needed
    }
}

#[cfg(test)]
mod tests {
    use super::server::create_mock_app;
    use actix_web::{body::to_bytes, test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn test_completions_endpoint() {
        let app = test::init_service(App::new().service(create_mock_app())).await;

        let req = test::TestRequest::post()
            .uri("/v1/completions")
            .set_json(&json!({
                "model": "text-davinci-003",
                "prompt": "Test prompt",
                "max_tokens": 5,
                "n": 2
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = to_bytes(resp.into_body()).await.unwrap();
        let resp_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(resp_json["choices"].as_array().unwrap().len(), 2);
        assert_eq!(
            resp_json["choices"][0]["text"],
            "Mock response 1 for prompt: Test prompt"
        );
        assert_eq!(
            resp_json["choices"][1]["text"],
            "Mock response 2 for prompt: Test prompt"
        );
    }

    #[actix_web::test]
    async fn test_completions_endpoint_missing_model() {
        let app = test::init_service(App::new().service(create_mock_app())).await;

        let req = test::TestRequest::post()
            .uri("/v1/completions")
            .set_json(&json!({
                "prompt": "Test prompt",
                "max_tokens": 5
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body = to_bytes(resp.into_body()).await.unwrap();
        let resp_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(resp_json["error"]["param"], "model");
    }
}