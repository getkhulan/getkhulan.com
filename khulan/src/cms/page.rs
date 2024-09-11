use crate::cms::model::Model;

pub struct Page {
    model: Model,
}

impl Page {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    pub fn model(&self) -> &Model {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cms::model::ModelBuilder;

    #[test]
    fn it_works() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
            .template("default")
            .build();
        let page = Page::new(model.unwrap());
        assert_eq!(page.model().title(), "Hello, World!");
    }
}
