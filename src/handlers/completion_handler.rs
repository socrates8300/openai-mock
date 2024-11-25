//! This module handles HTTP requests for generating text completions.
//!
//! It provides the `completions_handler` function, which processes incoming
//! completion requests, validates them, and returns appropriate responses.

use crate::models::{CompletionRequest, CompletionResponse, Usage};
use crate::validators::{
    validate_temperature, validate_top_p, validate_n, validate_max_tokens,
    validate_presence_penalty, validate_frequency_penalty, validate_best_of,
    validate_logprobs, validate_stop,
};
use crate::validators::StopSequence;
use crate::validators::validate_required_fields;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::utils::utils::{generate_uuid, get_current_timestamp};
use crate::utils::choices::create_choices;

/// Handles the `/completions` endpoint for generating text completions.
///
/// This asynchronous function processes a `CompletionRequest`, validates
/// the required and optional fields, generates completion choices, and
/// constructs a `CompletionResponse`. In case of validation errors, it
/// returns a `BadRequest` response with relevant error messages.
///
/// # Parameters
///
/// - `req`: A JSON payload deserialized into `CompletionRequest`.
///
/// # Returns
///
/// An `HttpResponse` containing the `CompletionResponse` on success or
/// an error message on failure.
pub async fn completions_handler(
    req: web::Json<CompletionRequest>,
) -> impl Responder {
    // Validate the required fields using the validator
    if let Err(validation_error) = validate_required_fields(&req) {
        return HttpResponse::BadRequest().json(json!({
            "error": {
                "message": validation_error.to_string(),
                "type": "invalid_request_error",
                "param": "model",
                "code": null,
            }
        }));
    }

    // Validate optional fields
    let validators = [
        ("temperature", validate_temperature(req.temperature)),
        ("top_p", validate_top_p(req.top_p)),
        ("n", validate_n(req.n)),
        ("max_tokens", validate_max_tokens(req.max_tokens)),
        ("presence_penalty", validate_presence_penalty(req.presence_penalty)),
        ("frequency_penalty", validate_frequency_penalty(req.frequency_penalty)),
        ("logprobs", validate_logprobs(req.logprobs)),
        ("stop", validate_stop(req.stop.clone())),
        ("best_of", validate_best_of(req.best_of, req.n)),
    ];

    // Check each validation result
    for (field, result) in validators {
        if let Err(validation_error) = result {
            return HttpResponse::BadRequest().json(json!({
                "error": {
                    "message": validation_error,
                    "type": "invalid_request_error",
                    "param": field,
                    "code": null,
                }
            }));
        }
    }

    // Mock processing logic
    let prompt = req.prompt.clone().unwrap_or_default();
    let max_tokens = req.max_tokens.unwrap_or(16);
    let n = req.n.unwrap_or(1);
    let echo = req.echo.unwrap_or(false);
    let logprobs = req.logprobs;

    let stop_sequences = match &req.stop {
        Some(StopSequence::Single(s)) => vec![s.clone()],
        Some(StopSequence::Multiple(v)) => v.clone(),
        None => Vec::new(),
    };

    let choices = create_choices(
        n,
        &prompt.to_string(),
        &stop_sequences,
        max_tokens,
        echo,
        logprobs,
        &req.model
    );

    let response = CompletionResponse {
        id: format!("cmpl-mock-id-{}", generate_uuid()),
        object: "text_completion".to_string(),
        created: get_current_timestamp().timestamp() as u64,
        model: req.model.clone(),
        choices,
        usage: Usage {
            prompt_tokens: count_tokens(&prompt.to_string()),
            completion_tokens: max_tokens,
            total_tokens: count_tokens(&prompt.to_string()) + max_tokens,
        },
    };
    HttpResponse::Ok().json(response)
}

/// Counts the number of tokens in a given text.
///
/// This is a mock implementation that simply counts whitespace-separated
/// words. In a real-world scenario, a proper tokenizer should be used.
///
/// # Parameters
///
/// - `text`: The input string to tokenize.
///
/// # Returns
///
/// The number of tokens as a `u32`.
fn count_tokens(text: &str) -> u32 {
    // This is a placeholder. In a real scenario, you might use a tokenizer.
    text.split_whitespace().count() as u32
}