#[macro_use]
extern crate rocket;

use crate::cms::site::SiteBuilder;

pub mod cms;
pub mod database;
pub mod routes;
pub mod watcher;

pub fn site() -> SiteBuilder {
    SiteBuilder::new()
}
