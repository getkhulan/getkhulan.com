use crate::cms::content::Content;
use crate::cms::page::Page;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncReadExt;
pub struct Site {
    dir: String,
    content: Content,
    pages: Vec<Page>,
    // TODO: add Files
}

impl Site {
    pub async fn new(pages: Option<Vec<Page>>, dir: Option<&str>) -> Self {
        let site = Self {
            dir: dir.unwrap_or("").to_string(),
            content: Content { fields: vec![] },
            pages: pages.unwrap_or(vec![]),
        };

        if site.pages.is_empty() {
            site.load().await;
        }

        site
    }

    async fn load(&self) {
        if self.pages.is_empty() == false {
            return;
        }

        #[cfg(feature = "content_folder")]
        self.load_from_kirby().await.unwrap();
    }

    // refactor to a getter and setter where the getter is not mutable
    pub fn pages(&self) -> &Vec<Page> {
        &self.pages
    }

    pub fn set_pages(&mut self, pages: Option<Vec<Page>>) -> &Vec<Page> {
        match pages {
            Some(pages) => {
                self.pages = pages;
                &self.pages
            }
            None => &self.pages,
        }
    }

    pub fn page(&self, search: &str) -> Option<&Page> {
        let page = self
            .pages
            .iter()
            .filter(|page| [page.model().path(), page.model().uuid()].contains(&search))
            .next();
        match page {
            Some(page) => Some(page),
            None => None,
        }
    }

    #[cfg(feature = "content_folder")]
    async fn load_from_kirby(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from(self.dir.as_str());

        // write out the path
        println!("Path: {:?}", path);

        if path.exists() == false {
            println!("Path XXX: {:?}", path);
            return Err("Path does not exist".into());
        }

        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();

            // Check if the entry is a file
            if file_path.is_file() {
                // Open and read the file asynchronously
                let mut file = fs::File::open(&file_path).await?;
                let mut contents = String::new();
                file.read_to_string(&mut contents).await?;

                // Output the file path and its contents
                println!("File: {:?}", file_path);
                println!("Contents:\n{}", contents);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cms::model::ModelBuilder;

    #[tokio::test]
    async fn it_works() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
            .template("default")
            .build();
        let page = Page::new(model.unwrap());
        let site = Site::new(Some(vec![page]), None).await;
        assert_eq!(site.pages.len(), 1);
    }

    #[tokio::test]
    async fn it_gets_page() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
            .template("default")
            .build();
        let page = Page::new(model.unwrap());
        let site = Site::new(Some(vec![page]), None).await;
        let page = site.page("1234");
        assert_eq!(page.unwrap().model().uuid(), "1234");
    }

    #[tokio::test]
    async fn it_sets_pages() {
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
            .template("default")
            .build();
        let page = Page::new(model.unwrap());
        let mut site = Site::new(Some(vec![page]), None).await;
        let model = ModelBuilder::new()
            .title("Hello, World!")
            .uuid("1234")
            .num("1")
            .path("/hello-world")
            .template("default")
            .build();
        let page = Page::new(model.unwrap());
        site.set_pages(Some(vec![page]));
        assert_eq!(site.pages.len(), 1);
    }

    #[tokio::test]
    async fn it_loads_from_kirby() {
        let site = Site::new(None, Some("TODO")).await;
        assert_eq!(site.load_from_kirby().await.is_ok(), true);
    }
}
