use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: Option<String>,
    #[serde(default, rename = "suffix")]
    pub suffix: Option<String>,
    #[serde(default = "default_max_tokens", rename = "max_tokens")]
    pub max_tokens: Option<u32>,
    #[serde(default = "default_temperature", rename = "temperature")]
    pub temperature: Option<f32>,
    #[serde(default = "default_top_p", rename = "top_p")]
    pub top_p: Option<f32>,
    #[serde(default = "default_n", rename = "n")]
    pub n: Option<u32>,
    #[serde(default = "default_stream", rename = "stream")]
    pub stream: Option<bool>,
    pub logprobs: Option<u32>,
    #[serde(default = "default_echo", rename = "echo")]
    pub echo: Option<bool>,
    pub stop: Option<Value>, // Can be a string or array of strings
    #[serde(default = "default_presence_penalty", rename = "presence_penalty")]
    pub presence_penalty: Option<f32>,
    #[serde(default = "default_frequency_penalty", rename = "frequency_penalty")]
    pub frequency_penalty: Option<f32>,
    pub best_of: Option<u32>,
    pub logit_bias: Option<Value>,
    pub user: Option<String>,
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