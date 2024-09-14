use crate::cms::content::Content;
use crate::cms::model::ModelBuilder;
use crate::cms::site::Site;
use crate::database::{Database, DatabaseError};
use std::fmt::Debug;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct Kirby {}

impl Database for Kirby {
    fn load(&self, site: &mut Site) -> Result<(), DatabaseError> {
        let rt = Runtime::new()
            .map_err(|_| DatabaseError::OtherError("Failed to create runtime".to_string()))?;

        rt.block_on(async {
            let path = PathBuf::from(format!("{}/storage/content", site.dir().to_str().unwrap()));
            let p = path.clone();

            if !path.exists() {
                return Err(DatabaseError::PathError("Path does not exist".to_string()));
            }

            let mut entries = tokio::fs::read_dir(path)
                .await
                .map_err(DatabaseError::from)?;

            while let Some(entry) = entries.next_entry().await.map_err(DatabaseError::from)? {
                let file_path = entry.path();

                if file_path.is_file() && file_path.extension().unwrap() == "txt" {
                    let mut file = tokio::fs::File::open(&file_path)
                        .await
                        .map_err(DatabaseError::from)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .await
                        .map_err(DatabaseError::from)?;

                    let mut content = Content::new(None);
                    content.load_txt(&contents);

                    let rel_path = file_path
                        .strip_prefix(p.clone())
                        .map(PathBuf::from)
                        .map_err(|_| {
                            DatabaseError::PathError("Failed to strip prefix".to_string())
                        })?;

                    let model = ModelBuilder::new()
                        .path(rel_path)
                        .template("default") // todo get template from filename
                        .content(&content)
                        .build();

                    // println!("{:?}", model);

                    site.models
                        .insert(model.path().to_string_lossy().to_string(), model);
                }
            }

            Ok(())
        })
    }

    /*
    fn loadAsync<'a>(&'a self, site: &mut Site) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send + 'a>> {
        Box::pin(async move {
            let path = PathBuf::from(format!("{}/storage/content", site.dir().to_str().unwrap()));
            let p = path.clone();

            if !path.exists() {
                return Err(DatabaseError::PathError("Path does not exist".to_string()));
            }

            let mut entries = fs::read_dir(path).await.map_err(DatabaseError::from)?;

            while let Some(entry) = entries.next_entry().await.map_err(DatabaseError::from)? {
                let file_path = entry.path();

                if file_path.is_file() && file_path.extension().unwrap() == "txt" {
                    let mut file = fs::File::open(&file_path).await.map_err(DatabaseError::from)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).await.map_err(DatabaseError::from)?;

                    let mut content = Content::new(None);
                    content.load_txt(&contents);

                    let rel_path = file_path
                        .strip_prefix(p.clone())
                        .map(PathBuf::from)
                        .map_err(|_| DatabaseError::PathError("Failed to strip prefix".to_string()))?;

                    let model = ModelBuilder::new()
                        .path(rel_path)
                        .template("default")
                        .content(&content)
                        .build();

                    site.models.insert(model.path().to_string_lossy().to_string(), model);
                }
            }

            Ok(())
        })
    }*/
}
