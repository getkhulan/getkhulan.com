use crate::cms::content::Content;
use crate::cms::model::ModelBuilder;
use crate::cms::page::Page;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncReadExt;
use url::Url;

#[derive(Debug)]
pub struct Site {
    dir: PathBuf, // TODO: refactor to roots hashmap
    url: Url,
    pub content: Content,
    pub pages: HashMap<String, Page>,
    // TODO: add Files
}

impl Site {
    pub async fn new(
        pages: Option<HashMap<String, Page>>,
        dir: Option<PathBuf>,
        url: Option<Url>,
    ) -> Self {
        Self {
            dir: dir.unwrap_or(PathBuf::from("")),
            url: url.unwrap_or(Url::parse("http://localhost:8000").unwrap()),
            content: Content::new(None),
            pages: pages.unwrap_or(HashMap::new()),
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub async fn load(&mut self) {
        if self.pages.is_empty() == false {
            return;
        }

        #[cfg(feature = "content_folder")]
        self.load_from_kirby().await.unwrap();
    }

    pub fn page(&self, search: &str) -> Option<&Page> {
        self.pages
            .get(search)
            .or_else(|| self.pages.values().find(|page| page.model.uuid() == search))
    }

    #[cfg(feature = "content_folder")]
    async fn load_from_kirby(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: make the content folder configurable
        let path = PathBuf::from(format!("{}/storage/content", self.dir.to_str().unwrap()));
        let p = path.clone();
        println!("{:#?}", path.clone());
        if path.exists() == false {
            return Err("Path does not exist".into());
        }

        let mut entries = fs::read_dir(path).await?;

        // TODO: make this recursive
        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            println!("{:#?}", file_path.clone());

            // Check if the entry is a file
            // TODO: only load those that will be page objects
            if file_path.is_file() && file_path.extension().unwrap() == "txt" {
                let mut file = fs::File::open(&file_path).await?;
                let mut contents = String::new();
                file.read_to_string(&mut contents).await?;

                // TODO: convert to builder pattern
                let mut content = Content::new(None);
                content.load_txt(contents.as_str());

                // TODO: this should remove the filename as well
                // TODO: handle site.en.txt
                let rel_path = file_path
                    .strip_prefix(p.clone())
                    .ok()
                    .map(PathBuf::from)
                    .unwrap();
                let model = ModelBuilder::new()
                    .path(rel_path) // TODO: make it relative to content folder
                    .template("default") // TODO: get the template
                    .content(&content)
                    .build();
                let page = Page::new(model);
                println!("{:#?}", page.clone());
                self.pages
                    .insert(page.model.path().to_string_lossy().to_string(), page);
            }
        }

        Ok(())
    }
}

pub struct SiteBuilder {
    dir: PathBuf,
    url: Url,
    content: Content,
    pages: HashMap<String, Page>,
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

    pub fn pages(&mut self, pages: HashMap<String, Page>) -> &mut Self {
        self.pages = pages;
        self
    }

    pub fn build(&self) -> Site {
        Site {
            dir: self.dir.clone(),
            url: self.url.clone(),
            content: self.content.clone(),
            pages: self.pages.clone(),
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
        let page = Page::new(model);
        let site = SiteBuilder::new()
            .pages(hashmap! {
                "1234".to_string() => page
            })
            .build();
        assert_eq!(site.pages.len(), 1);
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
        let page = Page::new(model);
        let site = SiteBuilder::new()
            .pages(hashmap! {
                "1234".to_string() => page
            })
            .build();
        let page = site.page("1234");
        assert_eq!(page.unwrap().model.uuid(), "1234");
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
        let page = Page::new(model);
        let mut site = SiteBuilder::new()
            .pages(hashmap! {
                "1234".to_string() => page
            })
            .build();

        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path(PathBuf::from("/hello-world"))
            .template("default")
            .build();
        let page = Page::new(model);
        site.pages.extend(hashmap! {
            "1234".to_string() => page
        });
        assert_eq!(site.pages.len(), 1);
    }

    #[tokio::test]
    #[cfg(feature = "content_folder")]
    async fn it_loads_from_kirby() {
        let mut site = SiteBuilder::new()
            .dir(PathBuf::from("/Users/bnomei/Sites/getkhulan-com"))
            .build();
        assert_eq!(site.load_from_kirby().await.is_ok(), true);
    }
}
