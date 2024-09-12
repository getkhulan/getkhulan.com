#[macro_use]
extern crate rocket;

use crate::cms::site::Site;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub mod cms;
pub mod routes;

pub async fn site(dir: PathBuf) -> Arc<RwLock<Site>> {
    // TODO: switch to builder pattern and provide some config values like current root dir
    Arc::new(RwLock::new(Site::new(None, Some(dir)).await))
}
