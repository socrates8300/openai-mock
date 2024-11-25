use crate::models::completion::Choice;
use crate::utils::token_counting::TokenCounter;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use crate::models::completion::Logprobs;


impl Choice {
    pub fn new(index: i32, text: String, echo: bool, prompt: &str) -> Self {
        let final_text = if echo {
            format!("{}{}", prompt, text)
        } else {
            text
        };

        Choice {
            text: final_text,
            index,
            logprobs: None,
            finish_reason: None,
        }
    }

    fn generate_mock_logprobs(&self, text: &str, logprobs_n: u32) -> Logprobs {
        let mut rng = thread_rng();

        // Split text into mock tokens (simple word-based splitting for mock data)
        let tokens: Vec<String> = text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let mut current_offset = 0;
        let mut text_offset: Vec<usize> = Vec::new();

        // Generate text offsets
        for token in &tokens {
            text_offset.push(current_offset);
            current_offset += token.len() + 1; // +1 for space
        }

        // Generate mock token logprobs
        let token_logprobs: Vec<f32> = (0..tokens.len())
            .map(|_| -rng.gen_range(0.0..5.0))
            .collect();

        // Generate top logprobs for each token
        let top_logprobs: Vec<HashMap<String, f32>> = tokens
            .iter()
            .map(|_| {
                let mut map = HashMap::new();
                for _ in 0..logprobs_n {
                    let mock_token = format!("token_{}", rng.gen_range(0..100));
                    let mock_logprob = -rng.gen_range(0.0..10.0);
                    map.insert(mock_token, mock_logprob);
                }
                map
            })
            .collect();

        Logprobs {
            tokens,
            token_logprobs,
            text_offset,
            top_logprobs,
        }
    }

    pub fn generate_text(
        &mut self,
        prompt: &str,
        stop_sequences: &[String],
        max_tokens: u32,
        echo: bool,
        logprobs_n: Option<u32>,
        model: &str
    ) {
        let mut generated = if echo {
            prompt.to_string()
        } else {
            String::new()
        };

        // Check for stop sequences
        for stop_seq in stop_sequences {
            if generated.contains(stop_seq) {
                self.finish_reason = Some("stop".to_string());
                generated = generated.split(stop_seq).next().unwrap_or("").to_string();
                self.text = generated;
                return;
            }
        }
        let token_counter = TokenCounter::new(&model);
        match token_counter {
            Ok(token_counter) => {
                // More robust token count estimation
                let estimated_tokens = token_counter.count_tokens(&generated);
                if estimated_tokens >= max_tokens {
            self.finish_reason = Some("length".to_string());
            self.text = token_counter.truncate_to_tokens(&generated, max_tokens);
                    return;
                }
            },
            Err(e) => {
                eprintln!("Error creating token counter: {}", e);
            }
        }

        self.text = generated;
        self.finish_reason = Some("content".to_string());

        // Generate logprobs if requested
        if let Some(n) = logprobs_n {
            self.logprobs = Some(self.generate_mock_logprobs(&self.text, n));
        }
    }
}

pub fn create_choices(
    n: i32,
    prompt: &str,
    stop_sequences: &[String],
    max_tokens: u32,
    echo: bool,
    logprobs: Option<u32>,
    model: &str
) -> Vec<Choice> {
    let mut choices = Vec::with_capacity(n as usize);

    for i in 0..n {
        let mut choice = Choice::new(i, String::new(), echo, prompt);
        choice.generate_text(prompt, stop_sequences, max_tokens, echo, logprobs, model);
        choices.push(choice);
    }

    choices
}
