use crate::cms::site::Site;
use rocket::{Config, State};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[get("/<path..>")]
pub fn index(path: PathBuf, site: &State<Arc<RwLock<Site>>>) -> String {
    let site = site.read().unwrap();
    let page = site.pages.get(&path.to_string_lossy().to_string());
    match page {
        Some(page) => page.model.title().to_string(),
        None => String::from("404"),
    }

    // let home = Model::build()
    //     .title("Home")
    //     .uuid("home")
    //     .num("0")
    //     .path("/")
    //     .template("home")
    //     .content(Content::new(None))
    //     .build().unwrap();
    //
    // home.title().to_string()
}

#[cfg(feature = "robots_txt")]
#[get("/robots.txt")]
pub fn robots_txt(config: &State<Config>) -> String {
    match config.profile.as_ref() {
        // TODO: reading profile as string does not work as expected. it's always "default"
        "release" => String::from("User-agent: *\nAllow: *\nSitemap: /sitemap.xml"), // TODO: add link to sitemap based on feature
        _ => String::from("User-agent: *\nDisallow: /"),
    }
}

#[cfg(feature = "sitemap_xml")]
#[get("/sitemap.xml")]
pub fn sitemap_xml() -> String {
    String::from("TODO") // TODO: sitemap
}
