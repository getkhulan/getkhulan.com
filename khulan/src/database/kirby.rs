use crate::cms::content::Content;
use crate::cms::field::Field;
use crate::cms::model::{Model, ModelBuilder, ModelKind};
use crate::cms::site::Site;
use crate::database::{Database, DatabaseError};
use std::fmt::Debug;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Kirby {}

impl Kirby {
    pub fn add_model_to_site(
        site: &mut Site,
        root_path: &PathBuf,
        file_path: &PathBuf,
        text: &str,
    ) {
        let model = Self::model_from_string(root_path, file_path, text);

        match model {
            Some(model) => {
                site.models.insert(model.path(), model);
            }
            None => {
                println!(
                    "Failed to add model at path '{}' to site",
                    file_path.to_string_lossy().to_string()
                );
            }
        }
    }

    pub fn model_from_string(
        root_path: &PathBuf,
        file_path: &PathBuf,
        text: &str,
    ) -> Option<Model> {
        let content = Self::content_from_string(text);
        let rel_path = file_path
            .strip_prefix(root_path.clone())
            .unwrap()
            .to_path_buf();
        // TODO: differentiate between page, site and file in model as prop (like in khulan kirby mongodb table)
        // return None
        let (dir_path, num, template, lang, filename) =
            Kirby::extract_components(&rel_path.to_path_buf());

        let mut kind = ModelKind::None;
        if template == "site" {
            kind = ModelKind::Site;
        } else if lang.is_empty() && filename.chars().filter(|&c| c == '.').count() > 1 {
            kind = ModelKind::File;
        } else if !lang.is_empty() && filename.chars().filter(|&c| c == '.').count() > 2 {
            kind = ModelKind::File;
        } else {
            kind = ModelKind::Page;
        }

        Some(
            ModelBuilder::new()
                .kind(&kind)
                .num(&num)
                .language(&lang)
                .path(&dir_path) // TODO: pages store the dir_path but files store the dir_path + filename
                .template(&template)
                .content(&content)
                .build(),
        )
    }

    pub fn content_from_string(text: &str) -> Content {
        let mut content = Content::new(None);
        for yml in text.split("----\n") {
            for data in YamlLoader::load_from_str(yml).unwrap() {
                data.as_hash().unwrap().iter().for_each(|(key, value)| {
                    let name = key.as_str().unwrap().to_lowercase();
                    let value = value.as_str().unwrap().trim();
                    let fname = key.as_str().unwrap();
                    content.fields.insert(name, Field::new(fname, Some(value)));
                });
            }
        }
        content
    }

    pub fn extract_components(file_path: &PathBuf) -> (String, String, String, String, String) {
        // 1. Extract `dir_path` without the filename
        let dir_path_buf = file_path
            .parent()
            .unwrap()
            .iter()
            .map(|segment| {
                let segment_str = segment.to_str().unwrap();

                // Split the segment by '_'
                match segment_str.split_once('_') {
                    // If there's a right-hand side after `_`, and it's non-empty, keep it
                    Some((_, right)) if !right.is_empty() => right.to_string(),
                    // If no split or right-hand side is empty, keep the original segment
                    _ => segment_str.to_string(),
                }
            })
            .collect::<PathBuf>();
        let dir_path = dir_path_buf.to_str().unwrap().to_string();

        // 2. Extract the last segment before the filename
        let last_segment = file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // Extract `num` by splitting the last segment on "_"
        let num = if last_segment.contains('_') {
            last_segment.split('_').next().unwrap_or("").to_string()
        } else {
            "".to_string() // No number exists in the last segment
        };

        // 3. Extract the `template` (filename before the first ".")
        let file_stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let template = file_stem.split('.').next().unwrap_or("").to_string();

        // 4. Extract the `lang` (part between the first and last ".")
        let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let parts: Vec<&str> = file_name.split('.').collect();
        let lang = if parts.len() > 1 { parts[1] } else { "" }.to_string();

        // 5. filename
        let filename = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        (dir_path, num, template, lang, filename)
    }

    // TODO: switch this to the file watcher which will load all on first run
    pub fn load_recursive<'a>(
        site: &'a mut Site,
        root_path: PathBuf,
        dir_path: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(dir_path.clone())
                .await
                .map_err(DatabaseError::from)?;

            while let Some(entry) = entries.next_entry().await.map_err(DatabaseError::from)? {
                let file_path = entry.path();

                if file_path.is_dir() {
                    // If it's a directory, recurse into it
                    Self::load_recursive(site, root_path.clone(), file_path).await?;
                } else if file_path.is_file() && file_path.extension().unwrap() == "txt" {
                    // If it's a file and has the `.txt` extension, process it
                    let mut file = fs::File::open(&file_path)
                        .await
                        .map_err(DatabaseError::from)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .await
                        .map_err(DatabaseError::from)?;

                    // Add the model to the site
                    Self::add_model_to_site(site, &root_path.clone(), &file_path, &contents);
                }
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_make_content_from_txt() {
        let text = " Title: Hello \n----\n\nDesc: World";
        let content = Kirby::content_from_string(text);
        assert_eq!(content.fields.len(), 2);
        assert_eq!(content.fields.get("title").unwrap().value(), "Hello");
        assert_eq!(content.fields.get("desc").unwrap().value(), "World");
    }

    #[test]
    fn it_can_extract_components() {
        let file_path = PathBuf::from("content/1_some/default.en.txt");
        let (dir_path, num, template, lang, filename) = Kirby::extract_components(&file_path);
        assert_eq!(dir_path, "content/some");
        assert_eq!(num, "1");
        assert_eq!(template, "default");
        assert_eq!(lang, "en");

        let file_path = PathBuf::from("home/home.en.txt");
        let (dir_path, num, template, lang, filename) = Kirby::extract_components(&file_path);
        assert_eq!(dir_path, "home");
        assert_eq!(num, "");
        assert_eq!(template, "home");
        assert_eq!(lang, "en");
    }
}

impl Database for Kirby {
    fn load(&self, site: &mut Site) -> Result<(), DatabaseError> {
        let rt = Runtime::new()
            .map_err(|_| DatabaseError::OtherError("Failed to create runtime".to_string()))?;

        rt.block_on(async {
            let root_path =
                PathBuf::from(format!("{}/storage/content", site.dir().to_str().unwrap()));

            Self::load_recursive(site, root_path.clone(), root_path.clone()).await
        })
    }
}
