use crate::models::completion::CompletionRequest;
use crate::validators::ValidationError;

/// Validates the required fields for a completion request
pub fn validate_required_fields(req: &CompletionRequest) -> Result<(), ValidationError> {
    // Validate model field
    if req.model.trim().is_empty() {
        return Err(ValidationError::new("model field must not be empty"));
    }

    // Note: prompt is already handled by the Optional<Value> type in the struct
    // and will always be present (though it can be None)

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_empty_model() {
        let req = CompletionRequest {
            model: "".to_string(),
            prompt: Some(json!("test prompt")),
            ..Default::default() // This will use all the default values we defined
        };

        let result = validate_required_fields(&req);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "model field must not be empty"
        );
    }

    #[test]
    fn test_validate_valid_request() {
        let req = CompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            prompt: Some(json!("test prompt")),
            ..Default::default()
        };

        let result = validate_required_fields(&req);
        assert!(result.is_ok());
    }
}