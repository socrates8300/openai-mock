use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::validators::StopSequence;

/// The request payload for the Completions API.
#[derive(Debug, Deserialize)]
pub struct CompletionRequest {
    /// ID of the model to use.
    pub model: String,

    /// The prompt(s) to generate completions for.
    #[serde(default)]
    pub prompt: Option<Value>, // Can be a string, array of strings, or null.

    /// The suffix that comes after a completion of inserted text.
    #[serde(default)]
    pub suffix: Option<String>,

    /// The maximum number of tokens to generate.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: Option<u32>,

    /// What sampling temperature to use.
    #[serde(default = "default_temperature")]
    pub temperature: Option<f32>,

    /// Nucleus sampling probability.
    #[serde(default = "default_top_p")]
    pub top_p: Option<f32>,

    /// How many completions to generate for each prompt.
    #[serde(default = "default_n")]
    pub n: Option<i32>,

    /// Whether to stream back partial progress.
    #[serde(default = "default_stream")]
    pub stream: Option<bool>,

    /// Include the log probabilities on the `logprobs` most likely tokens.
    #[serde(default)]
    pub logprobs: Option<u32>,

    /// Echo back the prompt in addition to the completion.
    #[serde(default = "default_echo")]
    pub echo: Option<bool>,

    /// The sequences where the API will stop generating further tokens.
    #[serde(default)]
    pub stop: Option<StopSequence>,

    /// Penalizes repeated tokens (as per frequency).
    #[serde(default = "default_presence_penalty")]
    pub presence_penalty: Option<f32>,

    /// Penalizes repeated tokens (as per frequency).
    #[serde(default = "default_frequency_penalty")]
    pub frequency_penalty: Option<f32>,

    /// Generates best_of completions server-side and returns the "best" one.
    #[serde(default)]
    pub best_of: Option<i32>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    #[serde(default)]
    pub logit_bias: Option<HashMap<String, i32>>,

    /// A unique identifier representing your end-user.
    #[serde(default)]
    pub user: Option<String>,
}

// Default values for optional parameters
fn default_max_tokens() -> Option<u32> {
    Some(16)
}

fn default_temperature() -> Option<f32> {
    Some(1.0)
}

fn default_top_p() -> Option<f32> {
    Some(1.0)
}

fn default_n() -> Option<i32> {
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

impl Default for CompletionRequest {
    fn default() -> Self {
        Self {
            model: String::new(),
            prompt: None,
            suffix: None,
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            top_p: default_top_p(),
            n: default_n(),
            stream: default_stream(),
            logprobs: None,
            echo: default_echo(),
            stop: None,
            presence_penalty: default_presence_penalty(),
            frequency_penalty: default_frequency_penalty(),
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }
}

/// The response from the Completions API.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Unique identifier for the completion.
    pub id: String,

    /// The object type (e.g., "text_completion").
    pub object: String,

    /// Creation time in epoch seconds.
    pub created: u64,

    /// The model used for the completion.
    pub model: String,

    /// The list of generated completions.
    pub choices: Vec<Choice>,

    /// Usage statistics for the completion.
    pub usage: Usage,
}

/// A single completion choice.
#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    /// The generated text.
    pub text: String,

    /// The index of this choice in the returned list.
    pub index: i32,

    /// The log probabilities of the tokens (if requested).
    #[serde(default)]
    pub logprobs: Option<Logprobs>,

    /// The reason why the completion ended (e.g., "stop", "length").
    #[serde(default)]
    pub finish_reason: Option<String>,
}

/// Log probabilities of the tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Logprobs {
    /// The tokens generated.
    pub tokens: Vec<String>,

    /// The log probabilities of the tokens.
    pub token_logprobs: Vec<f32>,

    /// The top log probabilities of tokens.
    #[serde(default)]
    pub top_logprobs: Option<Vec<HashMap<String, f32>>>,

    /// The character offset of each token.
    pub text_offset: Vec<u32>,
}

/// Usage statistics for the completion.
#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    /// The number of tokens in the prompt.
    pub prompt_tokens: u32,

    /// The number of tokens in the completion.
    pub completion_tokens: u32,

    /// The total number of tokens used.
    pub total_tokens: u32,
}