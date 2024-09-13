use crate::cms::content::Content;
use crate::cms::field::Field;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Model {
    num: String,
    path: PathBuf,
    template: String,
    content: Content,
    // site: &'static Site // TODO: this is a circular dependency?
}

impl Model {
    pub fn title(&self) -> &str {
        self.content.fields.get("title").unwrap().value()
    }

    pub fn uuid(&self) -> &str {
        self.content.fields.get("uuid").unwrap().value()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn num(&self) -> &str {
        &self.num
    }

    pub fn template(&self) -> &str {
        &self.template
    }

    pub fn url(&self) -> String {
        format!(
            "{}/{}",
            "http:://localhost:8000".to_string(),
            self.path.to_string_lossy()
        )
        // TODO: add url from site()
        // format!("{}/{}", self.site.url(), self.path.to_string_lossy())
    }
}

pub struct ModelBuilder {
    num: String,
    path: PathBuf,
    template: String,
    content: Content,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            num: "".to_string(),
            path: PathBuf::new(),
            template: "".to_string(),
            content: Content::new(None),
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.content
            .fields
            .entry("title".to_string())
            .or_insert_with(|| Field::new("title", None))
            .set_value(title.to_string());
        self
    }

    pub fn uuid(&mut self, uuid: &str) -> &mut Self {
        self.content
            .fields
            .entry("uuid".to_string())
            .or_insert_with(|| Field::new("uuid", None))
            .set_value(uuid.to_string());
        self
    }

    pub fn num(&mut self, num: &str) -> &mut Self {
        self.num = num.to_string();
        self
    }

    pub fn path(&mut self, path: PathBuf) -> &mut Self {
        self.path = path.clone();
        self
    }

    pub fn template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_string();
        self
    }

    pub fn content(&mut self, content: &Content) -> &mut Self {
        self.content.merge(content);

        // if not has uuid in content then set the path
        self.content
            .fields
            .entry("uuid".to_string())
            .or_insert_with(|| {
                Field::new(
                    "uuid",
                    Some(self.path.to_string_lossy().to_string().as_str()),
                )
            });

        self
    }

    /*
    pub fn build(&self) -> Result<Model, String> {
        for field in vec!["title".to_string(), "uuid".to_string()] {
            if self.content.fields.get(&field).is_none() {
                return Err(format!("{} is required", field));
            }
        }

        Ok(Model {
            num: self.num.clone(),
            path: self.path.clone(),
            template: self.template.to_string(),
            content: self.content.clone(),
        })
    } */

    pub fn build(&self) -> Model {
        Model {
            num: self.num.clone(),
            path: self.path.clone(),
            template: self.template.to_string(),
            content: self.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .path(PathBuf::from("/hello-world"))
            .uuid("123")
            .num("1")
            .build();

        assert_eq!(model.title(), "Hello, World!");
        assert_eq!(model.uuid(), "123");
        assert_eq!(model.num(), "1");
    }
}
