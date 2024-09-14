use crate::cms::content::Content;
use crate::cms::model::Model;
use crate::database::kirby::Kirby;
use crate::database::{Database, DatabaseBuilder};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub struct Site {
    dir: PathBuf, // TODO: refactor to roots hashmap
    url: Url,
    pub content: Content,
    pub models: HashMap<String, Model>,
}

impl Site {
    pub async fn new(
        models: Option<HashMap<String, Model>>,
        dir: Option<PathBuf>,
        url: Option<Url>,
    ) -> Self {
        Self {
            dir: dir.unwrap_or(PathBuf::from("")),
            url: url.unwrap_or(Url::parse("http://localhost:8000").unwrap()),
            content: Content::new(None),
            models: models.unwrap_or(HashMap::new()),
        }
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn load(&mut self) -> bool {
        if !self.models.is_empty() {
            return false;
        }

        match DatabaseBuilder::new().build().load(self) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error loading database: {}", e); // Print the error to the terminal
                false
            }
        }
    }

    pub fn page(&self, search: &str) -> Option<&Model> {
        let mut search = search.trim_matches('/');
        search = match search {
            "home" => "", // TODO: make this configurable
            _ => search,
        };
        self.models
            .get(search)
            .or_else(|| self.models.values().find(|model| model.uuid() == search))
    }

    pub fn file(&self, search: &str) -> Option<&Model> {
        let search = search.trim_matches('/');
        self.models.get(search).or_else(|| {
            self.models
                .values()
                .find(|model| model.path().to_string_lossy().to_string() == search)
        })
    }
}

pub struct SiteBuilder {
    dir: PathBuf,
    url: Url,
    content: Content,
    pages: HashMap<String, Model>,
}

impl SiteBuilder {
    pub fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            url: Url::parse("http://localhost:8000").unwrap(),
            content: Content::new(None),
            pages: HashMap::new(),
        }
    }

    pub fn url(&mut self, url: Url) -> &mut Self {
        self.url = url;
        self
    }

    pub fn dir(&mut self, dir: PathBuf) -> &mut Self {
        self.dir = dir;
        self
    }

    pub fn content(&mut self, content: Content) -> &mut Self {
        self.content = content;
        self
    }

    pub fn pages(&mut self, pages: HashMap<String, Model>) -> &mut Self {
        self.pages = pages;
        self
    }

    pub fn build(&self) -> Site {
        Site {
            dir: self.dir.clone(),
            url: self.url.clone(),
            content: self.content.clone(),
            models: self.pages.clone(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cms::model::ModelBuilder;
    use maplit::hashmap;

    #[test]
    fn it_works() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path(PathBuf::from("/hello-world"))
            .template("default")
            .build();
        let site = SiteBuilder::new()
            .pages(hashmap! {
                "1234".to_string() => model
            })
            .build();
        assert_eq!(site.models.len(), 1);
    }

    #[test]
    fn it_gets_page() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path(PathBuf::from("/hello-world"))
            .template("default")
            .build();
        let site = SiteBuilder::new()
            .pages(hashmap! {
                "123.4".to_string() => model
            })
            .build();
        let page = site.page("123.4");
        assert_eq!(page.unwrap().uuid(), "1234");
    }

    #[test]
    fn it_sets_pages() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path(PathBuf::from("/hello-world"))
            .template("default")
            .build();
        let mut site = SiteBuilder::new()
            .pages(hashmap! {
                "1234".to_string() => model
            })
            .build();

        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path(PathBuf::from("/hello-world"))
            .template("default")
            .build();
        site.models.extend(hashmap! {
            "1234".to_string() => model
        });
        assert_eq!(site.models.len(), 1);
    }

    #[test]
    #[cfg(feature = "kirby")]
    fn it_loads_from_kirby() {
        let mut site = SiteBuilder::new()
            .dir(PathBuf::from("/Users/bnomei/Sites/getkhulan-com"))
            .build();
        assert_eq!(site.load(), true);
        assert_eq!(site.models.len() > 0, true);
        println!("{:?}", site.models);
    }
}
