// TODO: add a prelude

#[macro_use]
extern crate rocket;

use rocket::{Config, State};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
