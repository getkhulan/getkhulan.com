#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    // kind: String,
    pub value: String,
}

impl Field {
    // change value into Option<&str>
    pub fn new(name: &str, value: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            value: value.unwrap_or("").to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
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

    // pub fn to_map(&self) -> Vec<(String, String)> {
    //    // TODO: it could be yaml or json, detect and parse then return map
    //      vec![]
    // }
}

#[cfg(test)]
mod tests_field {
    use super::*;

    #[test]
    fn it_works() {
        let field = Field::new("title", Some("Hello, World!"));
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
