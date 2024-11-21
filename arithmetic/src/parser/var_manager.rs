use super::{get_and_parse_user_input, Num};
use std::collections::HashMap;

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

        // self.prompt_terminal_for_variable_value(&normalized_name)
        let value = get_and_parse_user_input(var_name);
        let num_value = if value.fract() == 0.0 {
            Num::Integer(value as i64)
        } else {
            Num::Float(value)
        };

        self.variables.insert(normalized_name, num_value.clone());
        num_value
    }

    pub fn set_variable_value(&mut self, var_name: String, value: Num) {
        let normalized_name = var_name.to_lowercase();
        self.variables.insert(normalized_name, value);
    }

    // fn prompt_terminal_for_variable_value(&mut self, var_name: &str) -> Num {
    //     println!("Enter value for variable '{}':", var_name);

    //     loop {
    //         let mut input = String::new();
    //         print!("> ");
    //         io::stdout().flush().unwrap();
    //         io::stdin().read_line(&mut input).unwrap();

    //         let trimmed_input = input.trim();
    //         if trimmed_input.contains('.') {
    //             match trimmed_input.parse::<f64>() {
    //                 Ok(value) => {
    //                     let num_value = Num::Float(value);
    //                     self.variables
    //                         .insert(var_name.to_string(), num_value.clone());
    //                     return num_value;
    //                 }
    //                 Err(_) => println!("Invalid input. Please enter a valid number."),
    //             }
    //         } else {
    //             match trimmed_input.parse::<i64>() {
    //                 Ok(value) => {
    //                     let num_value = Num::Integer(value);
    //                     self.variables
    //                         .insert(var_name.to_string(), num_value.clone());
    //                     return num_value;
    //                 }
    //                 Err(_) => println!("Invalid input. Please enter a valid number."),
    //             }
    //         }
    //     }
    // }
}
