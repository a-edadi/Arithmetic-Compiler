use std::io::{self, Write};

pub fn get_value(name: &str) -> f64 {
    loop {
        print!("Enter the value for {}: ", name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input.parse::<f64>() {
            Ok(value) => {
                return value;
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}
