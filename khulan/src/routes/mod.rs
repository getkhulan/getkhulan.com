use crate::cms::site::Site;
use rocket::{Config, State};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[get("/<path..>")]
pub fn index(path: PathBuf, site_state: &State<Arc<RwLock<Site>>>) -> String {
    println!("=====>>>>> {:?}", path);

    // iterate between read and write locks to make requests that do not detect
    // changes to the site run faster and not block the site with the write lock
    let mut changes;
    {
        //  a read lock to check if the site needs refresh
        let site = site_state.read().unwrap();
        changes = site.changes();
        println!("CHANGES: {:?}", changes.clone());
    }

    if changes.len() > 0 {
        let mut site = site_state.write().unwrap();
        site.load(changes);
    }

    let site = site_state.read().unwrap();
    let page = site.page(&path.to_string_lossy().to_string(), None);

    match page {
        Some(page) => page.title().to_string(),
        None => String::from("404"),
    }
}

#[cfg(feature = "robots_txt")]
#[get("/robots.txt")]
pub fn robots_txt(site: &State<Arc<RwLock<Site>>>, config: &State<Config>) -> String {
    let site = site.read().unwrap();
    match config.profile.as_ref() {
        // TODO: reading profile as string does not work as expected. it's always "default"
        "release" => String::from("User-agent: *\nAllow: *\nSitemap: /sitemap.xml"), // TODO: add link to sitemap based on feature
        _ => String::from("User-agent: *\nDisallow: /"),
    }
}

#[cfg(feature = "sitemap_xml")]
#[get("/sitemap.xml")]
pub fn sitemap_xml(site: &State<Arc<RwLock<Site>>>, config: &State<Config>) -> String {
    let site = site.read().unwrap();
    String::from("TODO") // TODO: sitemap
}
