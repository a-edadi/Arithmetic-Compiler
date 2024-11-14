use super::{Lexer, Num};
use std::io::{self, Write};

impl<'a> Lexer<'a> {
    /// Gets the values from where ever
    /// TODO: Make it generic
    pub fn get_variable_value(&mut self, var_name: &str) -> Num {
        // If the variable exists -> return its value if not prompt to get the value
        if let Some(value) = self.variables.get(var_name) {
            return value.clone();
        }

        self.prompt_terminal_for_variable_value(var_name)
    }

    // Helper function to handle prompting for a value in the terminal
    fn prompt_terminal_for_variable_value(&mut self, var_name: &str) -> Num {
        println!("Enter value for variable '{}':", var_name);

        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            // Trim input and determine if it's an integer or float
            let trimmed_input = input.trim();
            if trimmed_input.contains('.') {
                match trimmed_input.parse::<f64>() {
                    Ok(value) => {
                        let num_value = Num::Float(value);
                        self.variables
                            .insert(var_name.to_string(), num_value.clone());
                        return num_value;
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            } else {
                match trimmed_input.parse::<i64>() {
                    Ok(value) => {
                        let num_value = Num::Integer(value);
                        self.variables
                            .insert(var_name.to_string(), num_value.clone());
                        return num_value;
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            }
        }
    }
}
