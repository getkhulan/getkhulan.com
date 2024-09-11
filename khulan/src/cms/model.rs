use crate::cms::content::Content;

#[derive(Debug, Clone)]
pub struct Model {
    title: String,
    uuid: String,
    num: String,
    path: String,
    template: String,
    content: Content,
}

pub struct ModelBuilder {
    title: String,
    uuid: String,
    num: String,
    path: String,
    template: String,
    content: Content,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            uuid: "".to_string(),
            num: "".to_string(),
            path: "".to_string(),
            template: "".to_string(),
            content: Content::new(None),
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn uuid(&mut self, uuid: &str) -> &mut Self {
        self.uuid = uuid.to_string();
        self
    }

    pub fn num(&mut self, num: &str) -> &mut Self {
        self.num = num.to_string();
        self
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    pub fn template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_string();
        self
    }

    pub fn content(&mut self, content: Content) -> &mut Self {
        self.content = content;
        self
    }

    pub fn build(&self) -> Result<Model, &'static str> {
        if self.title.is_empty() || self.uuid.is_empty() || self.num.is_empty() {
            return Err("title, uuid, and num are required");
        }
        Ok(Model {
            title: self.title.clone(),
            uuid: self.uuid.clone(),
            num: self.num.clone(),
            path: "".to_string(),
            template: "".to_string(),
            content: Content::new(None),
        })
    }
}

impl Model {
    pub fn build() -> ModelBuilder {
        ModelBuilder::new()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn url(&self) -> String {
        format!("/{}", self.path) // TODO: prefix with site url
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
            .num("1")
            .build()
            .unwrap();
        assert_eq!(model.title(), "Hello, World!");
    }
}
