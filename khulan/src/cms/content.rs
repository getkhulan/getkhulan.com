use super::field::Field;
use std::collections::HashMap;

use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct Content {
    pub fields: HashMap<String, Field>,
}

impl Content {
    pub fn new(fields: Option<HashMap<String, Field>>) -> Self {
        Content {
            fields: fields.unwrap_or(HashMap::new()),
        }
    }

    pub fn merge(&mut self, content: &Content) {
        self.fields.extend(content.fields.clone());
    }

    #[cfg(feature = "content_folder")]
    pub fn load_txt(&mut self, txt: &str) {
        for yml in txt.split("----\n") {
            for data in YamlLoader::load_from_str(yml).unwrap() {
                data.as_hash().unwrap().iter().for_each(|(key, value)| {
                    let name = key.as_str().unwrap().to_lowercase();
                    let value = value.as_str().unwrap().trim();
                    let fname = key.as_str().unwrap();
                    self.fields.insert(name, Field::new(fname, Some(value)));
                });
            }
        }
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
        let content = Content::new(Some(hashmap! {
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
        let mut content = Content::new(Some(hashmap! {
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
