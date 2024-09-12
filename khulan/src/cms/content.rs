use super::field::Field;

use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct Content {
    pub fields: Vec<Field>,
}

impl Content {
    pub fn new(fields: Option<Vec<Field>>) -> Self {
        Content {
            fields: fields.unwrap_or(vec![]),
        }
    }

    pub fn fields_mut(&mut self, fields: Option<Vec<Field>>) -> &Vec<Field> {
        match fields {
            Some(fields) => {
                self.fields = fields;
                &self.fields
            }
            None => &self.fields,
        }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn field_mut(&mut self, name: &str, set: Option<&str>) -> Option<&mut Field> {
        let get = self
            .fields
            .iter_mut()
            .filter(|get| get.name() == name)
            .next();
        match get {
            Some(get) => match set {
                Some(set) => {
                    get.set_value(Some(set));
                    Some(get)
                }
                None => Some(get),
            },
            None => None,
        }
    }

    pub fn field(&self, name: &str) -> Option<&Field> {
        let get = self.fields.iter().filter(|get| get.name() == name).next();
        match get {
            Some(get) => Some(get),
            None => None,
        }
    }

    pub fn merge(&mut self, content: Content) {
        for field in content.fields {
            let get = self
                .fields
                .iter_mut()
                .filter(|get| get.name() == field.name())
                .next();
            match get {
                Some(get) => {
                    get.set_value(Some(field.value()));
                }
                None => {
                    self.fields.push(field);
                }
            }
        }
    }

    #[cfg(feature = "content_folder")]
    pub fn load_txt(&mut self, txt: &str) {
        let fields = txt.split("----\n");
        for yml in fields {
            // use yaml-rust to parse field
            for data in YamlLoader::load_from_str(yml).unwrap() {
                let data = data.as_hash().unwrap();
                for (key, value) in data {
                    let name = key.as_str().unwrap().to_lowercase();
                    let value = value.as_str().unwrap().trim();
                    let field = Field::new(name.as_str(), Some(value));
                    self.fields.push(field);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let field = Field::new("title", Some("Hello, World!"));
        let content = Content::new(Some(vec![field]));
        assert_eq!(content.fields.len(), 1);
    }

    #[test]
    fn it_gets_field() {
        let field = Field::new("title", Some("Hello, World!"));
        let content = Content::new(Some(vec![field]));
        let field = content.field("title");
        assert_eq!(field.unwrap().name(), "title");
    }

    #[test]
    fn it_sets_fields() {
        let field = Field::new("title", Some("Hello, World!"));
        let mut content = Content::new(Some(vec![field]));
        let field = Field::new("title", Some("Hello, World!"));
        content.fields_mut(Some(vec![field]));
        assert_eq!(content.fields.len(), 1);
    }
}
