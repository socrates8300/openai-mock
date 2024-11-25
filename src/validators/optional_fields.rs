use serde::{Serialize, Deserialize};

pub fn validate_temperature(temperature: Option<f32>) -> Result<(), String> {
    if let Some(temp) = temperature {
        if temp < 0.0 || temp > 2.0 {
            return Err(format!("Temperature must be between 0.0 and 2.0, got {}", temp));
        }
    }
    Ok(())
}

pub fn validate_top_p(top_p: Option<f32>) -> Result<(), String> {
    if let Some(p) = top_p {
        if p < 0.0 || p > 1.0 {
            return Err(format!("Top_p must be between 0.0 and 1.0, got {}", p));
        }
    }
    Ok(())
}

pub fn validate_n(n: Option<i32>) -> Result<(), String> {
    if let Some(value) = n {
        if value <= 0 {
            return Err(format!("n must be a positive integer, got {}", value));
        }
    }
    Ok(())
}

pub fn validate_max_tokens(max_tokens: Option<u32>) -> Result<(), String> {
    if let Some(value) = max_tokens {
        if value <= 0 {
            return Err(format!("max_tokens must be a positive integer, got {}", value));
        }
    }
    Ok(())
}

pub fn validate_presence_penalty(presence_penalty: Option<f32>) -> Result<(), String> {
    if let Some(value) = presence_penalty {
        if value < -2.0 || value > 2.0 {
            return Err(format!("Presence penalty must be between -2.0 and 2.0, got {}", value));
        }
    }
    Ok(())
}

pub fn validate_frequency_penalty(frequency_penalty: Option<f32>) -> Result<(), String> {
    if let Some(value) = frequency_penalty {
        if value < -2.0 || value > 2.0 {
            return Err(format!("Frequency penalty must be between -2.0 and 2.0, got {}", value));
        }
    }
    Ok(())
}

pub fn validate_best_of(best_of: Option<i32>, n: Option<i32>) -> Result<(), String> {
    if let Some(best_of_value) = best_of {
        if best_of_value <= 0 {
            return Err(format!("best_of must be a positive integer, got {}", best_of_value));
        }

        if let Some(n_value) = n {
            if best_of_value < n_value {
                return Err(format!(
                    "best_of must be greater than or equal to n, got best_of={} and n={}",
                    best_of_value, n_value
                ));
            }
        }
    }
    Ok(())
}

pub fn validate_logprobs(logprobs: Option<u32>) -> Result<(), String> {
    if let Some(value) = logprobs {
        #[allow(unused_comparisons)]
        if value < 0 {
            return Err(format!("logprobs must be a non-negative integer, got {}", value));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopSequence {
    Single(String),
    Multiple(Vec<String>),
}

pub fn validate_stop(stop: Option<StopSequence>) -> Result<(), String> {
    if let Some(stop_value) = stop {
        match stop_value {
            StopSequence::Single(s) => {
                if s.is_empty() {
                    return Err("Stop sequence cannot be empty".to_string());
                }
            }
            StopSequence::Multiple(sequences) => {
                if sequences.is_empty() {
                    return Err("Stop sequences array cannot be empty".to_string());
                }
                for (i, sequence) in sequences.iter().enumerate() {
                    if sequence.is_empty() {
                        return Err(format!("Stop sequence at index {} cannot be empty", i));
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_temperature() {
        assert!(validate_temperature(None).is_ok());
        assert!(validate_temperature(Some(0.0)).is_ok());
        assert!(validate_temperature(Some(1.0)).is_ok());
        assert!(validate_temperature(Some(2.0)).is_ok());
        assert!(validate_temperature(Some(-0.1)).is_err());
        assert!(validate_temperature(Some(2.1)).is_err());
    }

    #[test]
    fn test_validate_top_p() {
        assert!(validate_top_p(None).is_ok());
        assert!(validate_top_p(Some(0.0)).is_ok());
        assert!(validate_top_p(Some(0.5)).is_ok());
        assert!(validate_top_p(Some(1.0)).is_ok());
        assert!(validate_top_p(Some(-0.1)).is_err());
        assert!(validate_top_p(Some(1.1)).is_err());
    }

    #[test]
    fn test_validate_n() {
        assert!(validate_n(None).is_ok());
        assert!(validate_n(Some(1)).is_ok());
        assert!(validate_n(Some(100)).is_ok());
        assert!(validate_n(Some(0)).is_err());
        assert!(validate_n(Some(-1)).is_err());
    }

    #[test]
    fn test_validate_max_tokens() {
        assert!(validate_max_tokens(None).is_ok());
        assert!(validate_max_tokens(Some(1)).is_ok());
        assert!(validate_max_tokens(Some(100)).is_ok());
        assert!(validate_max_tokens(Some(0)).is_err());
    }

    #[test]
    fn test_validate_presence_penalty() {
        assert!(validate_presence_penalty(None).is_ok());
        assert!(validate_presence_penalty(Some(-2.0)).is_ok());
        assert!(validate_presence_penalty(Some(0.0)).is_ok());
        assert!(validate_presence_penalty(Some(2.0)).is_ok());
        assert!(validate_presence_penalty(Some(-2.1)).is_err());
        assert!(validate_presence_penalty(Some(2.1)).is_err());
    }

    #[test]
    fn test_validate_frequency_penalty() {
        assert!(validate_frequency_penalty(None).is_ok());
        assert!(validate_frequency_penalty(Some(-2.0)).is_ok());
        assert!(validate_frequency_penalty(Some(0.0)).is_ok());
        assert!(validate_frequency_penalty(Some(2.0)).is_ok());
        assert!(validate_frequency_penalty(Some(-2.1)).is_err());
        assert!(validate_frequency_penalty(Some(2.1)).is_err());
    }

    #[test]
    fn test_validate_best_of() {
        // Test basic positive integer validation
        assert!(validate_best_of(None, None).is_ok());
        assert!(validate_best_of(Some(1), None).is_ok());
        assert!(validate_best_of(Some(0), None).is_err());

        // Test relationship with n
        assert!(validate_best_of(Some(5), Some(3)).is_ok());
        assert!(validate_best_of(Some(5), Some(5)).is_ok());
        assert!(validate_best_of(Some(3), Some(5)).is_err());
    }

    #[test]
    fn test_validate_logprobs() {
        assert!(validate_logprobs(None).is_ok());
        assert!(validate_logprobs(Some(0)).is_ok());
        assert!(validate_logprobs(Some(1)).is_ok());
        assert!(validate_logprobs(Some(100)).is_ok());
    }



    #[test]
    fn test_validate_stop() {
        // Test None case
        assert!(validate_stop(None).is_ok());

        // Test single string cases
        assert!(validate_stop(Some(StopSequence::Single("stop".to_string()))).is_ok());
        assert!(validate_stop(Some(StopSequence::Single("".to_string()))).is_err());

        // Test array cases
        assert!(validate_stop(Some(StopSequence::Multiple(vec![
            "stop1".to_string(),
            "stop2".to_string()
        ]))).is_ok());
        assert!(validate_stop(Some(StopSequence::Multiple(vec![]))).is_err());
        assert!(validate_stop(Some(StopSequence::Multiple(vec![
            "valid".to_string(),
            "".to_string()
        ]))).is_err());
    }
}
