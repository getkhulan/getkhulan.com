use crate::cms::content::Content;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Model {
    num: u16,
    path: PathBuf,
    template: String,
    content: Content,
}

pub struct ModelBuilder {
    num: u16,
    path: PathBuf,
    template: String,
    content: Content,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            num: 0, // TODO: this would mess up sorting, num should be None by default
            path: PathBuf::new(),
            template: "".to_string(),
            content: Content::new(None),
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.content.field_mut("title", Some(title));
        self
    }

    pub fn uuid(&mut self, uuid: &str) -> &mut Self {
        self.content.field_mut("uuid", Some(uuid));
        self
    }

    pub fn num(&mut self, num: u16) -> &mut Self {
        self.num = num;
        self
    }

    pub fn path(&mut self, path: PathBuf) -> &mut Self {
        self.path = path;
        self
    }

    pub fn template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_string();
        self
    }

    pub fn content(&mut self, content: Content) -> &mut Self {
        self.content.merge(content);
        self
    }

    pub fn build(&self) -> Result<Model, &'static str> {
        // if self.title.is_empty() || self.uuid.is_empty() || self.num.is_empty() {
        //     return Err("title, uuid, and num are required");
        // }
        Ok(Model {
            num: self.num.clone(),
            path: self.path.clone(),
            template: self.template.to_string(),
            content: self.content.clone(),
        })
    }
}

impl Model {
    pub fn build() -> ModelBuilder {
        ModelBuilder::new()
    }

    pub fn title(&self) -> &str {
        match self.content.field("title") {
            Some(field) => field.value(),
            None => "",
        }
    }

    pub fn uuid(&self) -> &str {
        match self.content.field("uuid") {
            Some(field) => field.value(),
            None => "",
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn num(&self) -> &u16 {
        &self.num
    }

    pub fn template(&self) -> &str {
        &self.template
    }

    pub fn url(&self) -> String {
        format!("/{}", self.path.to_string_lossy()) // TODO: prefix with site url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let model = Model::build()
            .title("Hello, World!")
            .uuid("123")
            .num(1)
            .build()
            .unwrap();
        assert_eq!(model.title(), "Hello, World!");
        assert_eq!(model.uuid(), "123");
        assert_eq!(*model.num(), 1);
    }
}
