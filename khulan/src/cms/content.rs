use super::field::Field;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Content {
    pub fields: HashMap<String, Field>,
}

impl Content {
    pub fn new(fields: Option<&HashMap<String, Field>>) -> Self {
        Content {
            fields: fields.cloned().unwrap_or_default(),
        }
    }

    pub fn merge(&mut self, content: &Content) {
        self.fields.extend(content.fields.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn it_can_have_no_fields() {
        let content = Content::new(None);
        assert_eq!(content.fields.len(), 0);
    }

    #[test]
    fn it_can_have_fields() {
        let content = Content::new(Some(&hashmap! {
            "title".to_string() => Field::new("title", Some("Hello, World!"))
        }));
        assert_eq!(content.fields.len(), 1);
    }

    #[test]
    fn it_gets_field() {
        let field = Field::new("title", Some("Hello, World!"));
        let mut content = Content::new(None);
        content.fields.insert(field.name().to_string(), field);
        let field = content.fields.get("title");
        assert_eq!(field.unwrap().name(), "title");
    }

    #[test]
    fn it_sets_fields() {
        let field = Field::new("title", Some("Hello, World!"));
        let mut content = Content::new(Some(&hashmap! {
            "title".to_string() => field
        }));
        assert_eq!(content.fields.len(), 1);

        content.fields = hashmap! {
            "1st-title".to_string() => Field::new("1st-title", Some("Hello, New World 1!")),
            "2nd-title".to_string() => Field::new("2nd-title", Some("Hello, New World 2!"))
        };
        assert_eq!(content.fields.len(), 2);
    }
}
