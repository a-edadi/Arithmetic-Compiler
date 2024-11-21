use std::io::{self, Write};

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
