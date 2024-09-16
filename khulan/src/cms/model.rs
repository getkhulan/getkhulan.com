use crate::cms::content::Content;
use crate::cms::field::Field;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Model {
    num: String,
    kind: ModelKind,
    language: String,
    path: String,
    template: String,
    content: Content,
    last_modified: SystemTime,
    // site: &'static Site // TODO: this is a circular dependency?
}

impl Model {
    pub fn title(&self) -> &str {
        self.content.fields.get("title").unwrap().value()
    }

    pub fn uuid(&self) -> &str {
        match self.content.fields.get("uuid") {
            Some(uuid) => uuid.value(),
            None => "",
        }
    }

    // NOTE: since path is used in hashmaps a key, it's better for convenience to return string here than a reference to a str
    pub fn path(&self) -> String {
        // In multi-language mode, the path is prefixed with the language
        // because the path is used as a key in the site's models hashmap.
        // we want the path stored in the model to be the same for all languages.
        #[cfg(feature = "multi_language")]
        return format!("{}/{}", self.language, self.path);

        #[cfg(not(feature = "multi_language"))]
        self.path.clone()
    }

    pub(crate) fn last_modified(&self) -> SystemTime {
        self.last_modified
    }

    pub fn num(&self) -> &str {
        &self.num
    }

    pub fn kind(&self) -> &ModelKind {
        &self.kind
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn template(&self) -> &str {
        &self.template
    }

    pub fn url(&self) -> String {
        format!("{}/{}", "http:://localhost:8000".to_string(), self.path)
        // TODO: add url from site()
        // format!("{}/{}", self.site.url(), self.path.to_string_lossy())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelKind {
    Page,
    Site,
    File,
    User,
    None,
}

pub struct ModelBuilder {
    num: String,
    kind: ModelKind,
    language: String,
    path: String,
    template: String,
    content: Content,
    last_modified: SystemTime,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            num: "".to_string(),
            kind: ModelKind::None,
            language: "".to_string(),
            path: "".to_string(),
            template: "".to_string(),
            content: Content::new(None),
            last_modified: SystemTime::now(),
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.content
            .fields
            .entry("title".to_string())
            .or_insert_with(|| Field::new("title", None))
            .set_value(title);
        self
    }

    pub fn uuid(&mut self, uuid: &str) -> &mut Self {
        self.content
            .fields
            .entry("uuid".to_string())
            .or_insert_with(|| Field::new("uuid", None))
            .set_value(uuid);
        self
    }

    pub fn num(&mut self, num: &str) -> &mut Self {
        self.num = num.to_string();
        self
    }

    pub fn language(&mut self, language: &str) -> &mut Self {
        self.language = language.to_string();
        self
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    pub fn kind(&mut self, kind: &ModelKind) -> &mut Self {
        self.kind = kind.clone();
        self
    }

    // infer these from the path and num
    // TODO: fn is_draft()
    // TODO: fn is_unlisted()
    // TODO: fn is_listed()
    // TODO: fn is_published()

    pub fn template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_string();
        self
    }

    pub fn content(&mut self, content: &Content) -> &mut Self {
        self.content.merge(content);
        self
    }

    pub fn last_modified(&mut self, last_modified: SystemTime) -> &mut Self {
        self.last_modified = last_modified;
        self
    }

    pub fn build(&self) -> Model {
        Model {
            num: self.num.clone(),
            kind: self.kind.clone(),
            language: self.language.clone(),
            path: self.path.clone(),
            template: self.template.to_string(),
            content: self.content.clone(),
            last_modified: self.last_modified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;

    #[test]
    fn it_works() {
        let modified_at = SystemTime::now().sub(std::time::Duration::from_secs(60));
        let model = ModelBuilder::new()
            .kind(&ModelKind::Page)
            .title("Hello, World!")
            .language("en")
            .path("/hello-world")
            .uuid("123")
            .num("1")
            .last_modified(modified_at)
            .build();

        assert_eq!(model.path(), "/hello-world");
        assert_eq!(model.title(), "Hello, World!");
        assert_eq!(model.uuid(), "123");
        assert_eq!(model.language(), "en");
        assert_eq!(model.num(), "1");
        assert_eq!(model.kind(), &ModelKind::Page);
        assert_eq!(model.last_modified(), modified_at);
    }
}
