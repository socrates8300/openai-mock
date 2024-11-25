use tiktoken_rs::{cl100k_base, p50k_base, o200k_base};
use crate::models::completion::Usage;

pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct TokenCounter {
    encoding: tiktoken_rs::CoreBPE,
}

impl TokenCounter {
    pub fn new(model: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let encoding = match model {
            "gpt-4" | "gpt-3.5-turbo" | "text-embedding-ada-002" => {
                cl100k_base()?
            },
            "gpt-4o" | "gpt-4o-mini" => {
                o200k_base()?
            },
            "text-davinci-002" | "text-davinci-003" => {
                p50k_base()?
            },
            _ => cl100k_base()? // default to cl100k_base
        };

        Ok(Self { encoding })
    }

    pub fn count_tokens(&self, text: &str) -> u32 {
        self.encoding.encode_with_special_tokens(text).len() as u32
    }

    pub fn count_messages_tokens(&self, messages: &[ChatMessage]) -> u32 {
        // Add 3 tokens for each message for ChatML format
        let per_message_tokens = 3;

        messages.iter().map(|msg| {
            self.count_tokens(&msg.content) +
            self.count_tokens(&msg.role) +
            per_message_tokens
        }).sum()
    }

    /// Creates a Usage struct with token counts for prompt and completion
    pub fn calculate_usage(&self, prompt: &str, completion: &str) -> Usage {
        let prompt_tokens = self.count_tokens(prompt);
        let completion_tokens = self.count_tokens(completion);

        Usage {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }

    /// Truncates text to approximately fit within max_tokens
    pub fn truncate_to_tokens(&self, text: &str, max_tokens: u32) -> String {
        let tokens = self.encoding.encode_with_special_tokens(text);
        if tokens.len() as u32 <= max_tokens {
            return text.to_string();
        }

        let truncated_tokens = tokens[..max_tokens as usize].to_vec();
        self.encoding.decode(truncated_tokens).unwrap()
    }
}
