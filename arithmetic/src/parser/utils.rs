use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_and_parse_user_input(name: &str) -> f64 {
    loop {
        println!("\nPlease Enter the value for {}:", name);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        match trimmed_input.parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
/// Generate random number
pub fn generate_random_4_digits() -> u16 {
    // Get the current time in milliseconds
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = duration.as_secs() as u16;

    (seed % 9000) + 1000
}
