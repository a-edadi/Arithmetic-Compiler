use super::Num;
use std::collections::HashMap;
use std::io::{self, Write};

pub struct VariableManager {
    variables: HashMap<String, Num>,
}

impl VariableManager {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn get_variable_value(&mut self, var_name: &str) -> Num {
        let normalized_name = var_name.to_lowercase();

        if let Some(value) = self.variables.get(&normalized_name) {
            return value.clone();
        }

        self.prompt_terminal_for_variable_value(&normalized_name)
    }

    fn prompt_terminal_for_variable_value(&mut self, var_name: &str) -> Num {
        println!("Enter value for variable '{}':", var_name);

        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

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
