use std::time::{SystemTime, UNIX_EPOCH};

/// Generate random number
pub fn generate_random_4_digits() -> u16 {
    // Get the current time in milliseconds
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = duration.as_secs() as u16;

    (seed % 9000) + 1000
}
