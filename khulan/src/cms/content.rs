use super::field::Field;

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

    pub fn fields(&mut self, fields: Option<Vec<Field>>) -> &Vec<Field> {
        match fields {
            Some(fields) => {
                self.fields = fields;
                &self.fields
            }
            None => &self.fields,
        }
    }

    pub fn field(&mut self, name: &str) -> Option<&Field> {
        let field = self
            .fields
            .iter()
            .filter(|field| field.name() == name)
            .next();
        match field {
            Some(field) => Some(field),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let field = Field::new("title", "text", Some("Hello, World!"));
        let content = Content::new(Some(vec![field]));
        assert_eq!(content.fields.len(), 1);
    }

    #[test]
    fn it_gets_field() {
        let field = Field::new("title", "text", Some("Hello, World!"));
        let mut content = Content::new(Some(vec![field]));
        let field = content.field("title");
        assert_eq!(field.unwrap().name(), "title");
    }

    #[test]
    fn it_sets_fields() {
        let field = Field::new("title", "text", Some("Hello, World!"));
        let mut content = Content::new(Some(vec![field]));
        let field = Field::new("title", "text", Some("Hello, World!"));
        content.fields(Some(vec![field]));
        assert_eq!(content.fields.len(), 1);
    }
}
