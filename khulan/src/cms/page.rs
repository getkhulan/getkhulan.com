use crate::cms::model::Model;

#[derive(Debug, Clone)]
#[deprecated]
pub struct Page {
    pub model: Model,
}

impl Page {
    pub fn new(model: Model) -> Self {
        Self { model }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cms::model::ModelBuilder;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path(PathBuf::from("/hello-world"))
            .template("default")
            .build();
        let page = Page::new(model);
        assert_eq!(page.model.title(), "Hello, World!");
    }
}
