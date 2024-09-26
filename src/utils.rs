use uuid::Uuid;

/// Generates a unique identifier
pub fn generate_unique_id() -> String {
    Uuid::new_v4().to_string()
}

/// Gets the current timestamp in milliseconds
pub fn current_timestamp() -> u128 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    now.as_millis()
}
