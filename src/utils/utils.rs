use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Generates a new unique UUID v4 string
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Returns the current UTC timestamp
pub fn get_current_timestamp() -> DateTime<Utc> {
    Utc::now()
}
