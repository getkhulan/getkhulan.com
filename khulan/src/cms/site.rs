use crate::cms::model::{Model, ModelKind};
use crate::database::{Database, DatabaseBuilder};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub struct Site {
    dir: PathBuf, // TODO: refactor to roots hashmap
    url: Url,
    pub models: HashMap<String, Model>,
}

impl Site {
    pub fn new(
        models: Option<HashMap<String, Model>>,
        dir: Option<PathBuf>,
        url: Option<Url>,
    ) -> Self {
        Self {
            dir: dir.unwrap_or(PathBuf::from("")),
            url: url.unwrap_or(Url::parse("http://localhost:8000").unwrap()),
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

    #[cfg(feature = "multi_language")]
    fn map_home_to_empty_route(&self, search: &str) -> String {
        let search = search.trim_matches('/').to_string();
        if search.chars().filter(|&c| c == '/').count() == 0 {
            format!("{}/{}", search, "home") // TODO: make this configurable
        } else {
            search
        }
    }

    #[cfg(not(feature = "multi_language"))]
    fn map_home_to_empty_route(&self, search: &str) -> String {
        let search = search.trim_matches('/').to_string();
        search = match search {
            "" => "home", // TODO: make this configurable
            _ => search,
        };
    }

    pub fn page(&self, search: &str, lang: Option<&str>) -> Option<&Model> {
        let search = self.map_home_to_empty_route(search);

        match lang {
            Some(lang) => self.models.values().find(|model| {
                model.language() == lang
                    && model.kind() == &ModelKind::Page
                    && (model.path() == search || model.uuid() == search)
            }),
            None => self.models.get(search.as_str()).or_else(|| {
                self.models.values().find(|model| {
                    model.kind() == &ModelKind::Page
                        && (model.path() == search || model.uuid() == search)
                })
            }),
        }
    }

    pub fn find(&self, search: &str) -> Option<&Model> {
        let search = search.trim_matches('/');
        self.models
            .get(search)
            .or_else(|| self.models.values().find(|model| model.path() == search))
    }
}

pub struct SiteBuilder {
    dir: PathBuf,
    url: Url,
    models: HashMap<String, Model>,
}

impl SiteBuilder {
    pub fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            url: Url::parse("http://localhost:8000").unwrap(),
            models: HashMap::new(),
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

    pub fn models(&mut self, models: HashMap<String, Model>) -> &mut Self {
        self.models = models;
        self
    }

    pub fn build(&self) -> Site {
        Site {
            dir: self.dir.clone(),
            url: self.url.clone(),
            models: self.models.clone(),
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
            .path("/hello-world")
            .template("default")
            .build();
        let site = SiteBuilder::new()
            .models(hashmap! {
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
            .path("/hello-world")
            .template("default")
            .build();
        let site = SiteBuilder::new()
            .models(hashmap! {
                "123.4".to_string() => model
            })
            .build();
        let page = site.page("123.4", None);
        assert_eq!(page.unwrap().uuid(), "1234");
    }

    #[test]
    fn it_sets_pages() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
            .template("default")
            .build();
        let mut site = SiteBuilder::new()
            .models(hashmap! {
                "1234".to_string() => model
            })
            .build();

        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
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
