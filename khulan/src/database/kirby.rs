use crate::cms::content::Content;
use crate::cms::field::Field;
use crate::cms::model::{Model, ModelBuilder, ModelKind};
use crate::cms::site::Site;
use crate::database::{Database, DatabaseError};
use crate::watcher::file::FileWatcher;
use dotenvy;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

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
        let (dir_path, mut num, mut template, lang, filename) =
            Kirby::extract_components(&rel_path.to_path_buf());

        let kind = match (
            template.as_str(),
            lang.is_empty(),
            filename.chars().filter(|&c| c == '.').count(),
        ) {
            ("site", _, _) => ModelKind::Site,
            (_, true, dot_count) if dot_count > 1 => ModelKind::File,
            (_, false, dot_count) if dot_count > 2 => ModelKind::File,
            _ => ModelKind::Page,
        };

        if kind == ModelKind::File {
            template = "".to_string(); // get from content field "template"
            num = "".to_string(); // get from content field "sort"
        }

        // println!("--> {:?} ==== {}", filename, file_path.to_string_lossy());

        Some(
            ModelBuilder::new()
                .kind(&kind)
                .num(&num)
                .language(&lang)
                .path(&dir_path) // TODO: pages store the dir_path but files store the dir_path + filename
                .template(&template)
                .content(&content)
                .last_modified(file_path.metadata().unwrap().modified().unwrap())
                .root(file_path.to_str()?)
                .build(),
        )
    }

    pub fn content_from_string(text: &str) -> Content {
        let mut content = Content::new(None);
        for yml in text.split("----\n") {
            let parts: Vec<&str> = yml.splitn(2, ":").collect();
            if parts.len() == 2 {
                content.fields.insert(
                    parts[0].trim().to_lowercase(),
                    Field::new(
                        parts[0].trim().to_lowercase().as_str(),
                        Some(parts[1].trim()),
                    ),
                );
            } else {
                // println!("No valid key-value pair found");
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
        #[cfg(feature = "multi_language")]
        let lang = if parts.len() > 1 {
            parts[parts.len() - 2] // Access the second-to-last part
        } else {
            ""
        }
        .to_string();
        #[cfg(not(feature = "multi_language"))]
        let lang = "".to_string();

        // 5. filename
        let filename = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        (dir_path, num, template, lang, filename)
    }

    pub fn load_recursive(
        site: &mut Site,
        root_path: &PathBuf, // Change to reference
        dir_path: &PathBuf,  // Change to reference
    ) -> Result<(), DatabaseError> {
        // Read the directory synchronously
        let entries = fs::read_dir(dir_path).map_err(DatabaseError::from)?;

        // Iterate over directory entries
        for entry in entries {
            let entry = entry.map_err(DatabaseError::from)?;
            let file_path = entry.path();

            // Recurse into directories
            if file_path.is_dir()
                && !file_path
                    .to_str()
                    .map_or(false, |p| p.contains("_versions"))
            // K5: _versions is a directory that contains versioned content
            {
                Self::load_recursive(site, root_path, &file_path)?;
            } else if file_path.is_file()
                && file_path.extension().and_then(|ext| ext.to_str()) == Some("txt")
            // TODO: Add support for markdown files?
            {
                // If it's a .txt file, read its contents
                let mut file = fs::File::open(&file_path).map_err(DatabaseError::from)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(DatabaseError::from)?;

                // Add the model to the site (assuming this is defined elsewhere)
                Self::add_model_to_site(site, root_path, &file_path, &contents);
            }
        }

        Ok(())
    }

    pub fn content_folder_path(site: &Site) -> PathBuf {
        // load from env variable
        let dir = dotenvy::var("KIRBY_CONTENT")
            .unwrap_or_else(|_| format!("{}/storage/content", site.dir().to_str().unwrap()));
        PathBuf::from(dir)
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
        assert_eq!(filename, "default.en.txt");

        let file_path = PathBuf::from("home/home.en.txt");
        let (dir_path, num, template, lang, filename) = Kirby::extract_components(&file_path);
        assert_eq!(dir_path, "home");
        assert_eq!(num, "");
        assert_eq!(template, "home");
        assert_eq!(lang, "en");
        assert_eq!(filename, "home.en.txt");
    }
}

impl Database for Kirby {
    fn load(&self, site: &mut Site, changes: Vec<String>) -> Result<(), DatabaseError> {
        let root_path = Self::content_folder_path(site);

        // if empty changes then load all from root_path
        if changes.is_empty() {
            Self::load_recursive(site, &root_path, &root_path)
        } else {
            // else load each changed dirs/files separately
            for change in changes {
                Self::load_recursive(site, &root_path, &PathBuf::from(change))?;
            }
            Ok(())
        }
    }

    fn changes(&self, site: &Site) -> Vec<String> {
        let root_path = Self::content_folder_path(site);
        let state_from_models: HashMap<String, SystemTime> = site
            .models
            .iter()
            .map(|(_, model)| (model.root(), model.last_modified()))
            .collect();

        FileWatcher::new(
            root_path,
            Some(state_from_models),
            Some(vec!["txt".to_string()]), // kirby txt files only
        )
        .changes()
    }
}
