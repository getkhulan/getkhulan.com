#[macro_use]
extern crate rocket;

use crate::cms::site::SiteBuilder;

pub mod cms;
pub mod routes;

pub fn site() -> SiteBuilder {
    SiteBuilder::new()
}
