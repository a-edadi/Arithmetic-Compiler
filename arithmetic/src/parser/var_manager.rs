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
}
