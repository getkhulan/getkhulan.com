use crate::cms::content::Content;
use crate::cms::field::Field;
use crate::cms::model::ModelKind::File;
use crate::cms::site::Site;
use rocket::serde::Serialize;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug, Clone)]
pub struct Model {
    num: String,
    kind: ModelKind,
    language: String,
    path: String,
    template: String,
    content: Content,
    root: String,
    last_modified: SystemTime,
}

impl Model {
    pub fn title(&self) -> &str {
        match self.content.fields.get("title") {
            Some(title) => title.value(),
            None => "",
        }
    }

    pub fn uuid(&self) -> &str {
        match self.content.fields.get("uuid") {
            Some(uuid) => uuid.value(),
            None => "",
        }
    }

    // NOTE: since path is used in hashmaps a key, it's better for convenience to return string here than a reference to a str
    pub fn path(&self) -> String {
        let mut path = self.path.clone();
        if path == "home" {
            // TODO: make this configurable
            path = "".to_string();
        }
        if self.kind == ModelKind::Site {
            path = "$".to_string(); // hack to make site model not overlap with home
        }
        if self.kind == ModelKind::File {
            path = format!(
                "{}/{}",
                self.path,
                PathBuf::from(&self.root)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
        }

        // the following works for both single and multi-language sites
        // because of the trim and lang being "" for single language
        format!("{}/{}", self.language, path)
            .trim_matches('/')
            .to_string()
    }

    pub fn last_modified(&self) -> SystemTime {
        self.last_modified
    }

    pub fn num(&self) -> Option<u16> {
        if self.num.is_empty() {
            if self.kind == File {
                // try sort field, like for files
                match self.content.fields.get("sort") {
                    Some(sort) => match sort.value().parse::<u16>() {
                        Ok(num) => Some(num),
                        Err(_) => None,
                    },
                    None => None,
                }
            } else {
                None
            }
        } else {
            match self.num.parse::<u16>() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        }
    }

    pub fn kind(&self) -> &ModelKind {
        &self.kind
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn template(&self) -> &str {
        if self.template.is_empty() {
            match self.content.fields.get("template") {
                Some(template) => template.value(),
                None => "",
            }
        } else {
            &self.template
        }
    }

    pub fn root(&self) -> String {
        self.root.clone()
    }

    pub fn modified(&self) -> SystemTime {
        self.last_modified
    }

    // infer these from the path and num
    pub fn is_draft(&self) -> bool {
        let root_path = PathBuf::from(&self.root);

        if let Some(parent) = root_path.parent() {
            parent.to_str().map_or(false, |p| p.contains("_drafts"))
        } else {
            false
        }
    }

    fn is_unlisted(&self) -> bool {
        self.num.is_empty()
    }
    fn is_listed(&self) -> bool {
        !self.is_unlisted()
    }
    fn is_published(&self) -> bool {
        !self.is_draft()
    }

    pub fn parent(&self, site: &Site) -> Option<Model> {
        // Split the path by '/' and remove the last segment
        let mut segments: Vec<&str> = self.path.split('/').collect();
        if segments.pop().is_none() {
            return None; // If there's no parent
        }

        // Join the remaining segments back into a parent path
        let parent_path = segments.join("/");

        // Search the models in the site to find the one with a matching path
        let model = site.models.values().find(|model| model.path == parent_path);

        match model {
            None => None,
            Some(m) => Some(m.clone()),
        }
    }

    pub fn children(&self, site: &Site) -> Vec<Model> {
        // let mut children = vec![];
        site.models
            .values()
            .filter(|model| {
                model.path.starts_with(&self.path) && model.path != self.path
                // && model.path.split('/').count() == self.path.split('/').count() + 1
            })
            .map(|m| m.clone())
            .collect()
    }

    pub fn url(&self) -> String {
        format!("{}/{}", "http:://localhost:8000".to_string(), self.path)
        // TODO: add url from site()
        // format!("{}/{}", self.site.url(), self.path.to_string_lossy())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
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
    root: String,
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
            root: "".to_string(),
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

    pub fn template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_string();
        self
    }

    pub fn root(&mut self, root: &str) -> &mut Self {
        self.root = root.to_string();
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
            path: self.path.clone().trim_matches('/').to_string(),
            template: self.template.to_string(),
            content: self.content.clone(),
            last_modified: self.last_modified,
            root: self.root.clone(),
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
            .root("/some/fl/root")
            .last_modified(modified_at)
            .build();

        assert_eq!(model.path(), "en/hello-world");
        assert_eq!(model.title(), "Hello, World!");
        assert_eq!(model.uuid(), "123");
        assert_eq!(model.language(), "en");
        assert_eq!(model.num().unwrap(), 1);
        assert_eq!(*model.kind(), ModelKind::Page);
        assert_eq!(model.last_modified(), modified_at);
        assert_eq!(model.root(), "/some/fl/root");
        assert_eq!(model.is_draft(), false);
        assert_eq!(model.is_published(), true);
        assert_eq!(model.is_unlisted(), false);
        assert_eq!(model.is_listed(), true);
    }
}
