use rocket::serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Field {
    pub name: String,
    // kind: String,
    pub value: String,
}

impl Field {
    // change value into Option<&str>
    pub fn new(name: &str, value: Option<&str>) -> Self {
        Self {
            name: name.to_string().trim().to_lowercase(),
            value: value.unwrap_or("").to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn to_int(&self) -> i32 {
        self.value.parse::<i32>().unwrap()
    }

    pub fn to_float(&self) -> f32 {
        self.value.parse::<f32>().unwrap()
    }

    pub fn to_bool(&self) -> bool {
        self.value.parse::<bool>().unwrap()
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.value.split(",").map(|s| s.to_string()).collect()
    }

    pub fn to_systemtime(&self) -> SystemTime {
        match self.value.parse::<u64>() {
            Ok(seconds) => UNIX_EPOCH + std::time::Duration::from_secs(seconds),
            Err(_) => SystemTime::now(), // Fallback in case of parse failure
        }
    }
}

#[cfg(test)]
mod tests_field {
    use super::*;

    #[test]
    fn it_works() {
        let field = Field::new("Title", Some("Hello, World!"));
        assert_eq!(field.name, "title");
        assert_eq!(field.name(), "title");
        assert_eq!(field.value, "Hello, World!");
        assert_eq!(field.value(), "Hello, World!");
    }

    #[test]
    fn it_sets_value() {
        let mut field = Field::new("title", None);
        field.value = "Hello, World!".to_string();
        assert_eq!(field.value(), "Hello, World!");
    }
}
