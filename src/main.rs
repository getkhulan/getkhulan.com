#[macro_use]
extern crate rocket;

use khulan::cms::site::Site;
use khulan::routes::*;
use khulan::site;
use maud::{html, Markup};
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::Config;
use rocket_dyn_templates::{context, Template};
use std::sync::{Arc, RwLock};
use url::Url;

#[get("/hbs")]
fn thbs() -> Template {
    let context = context! { hello: "world!".to_string() };
    Template::render("index", &context)
}

#[get("/maud")]
fn tmaud() -> Markup {
    html! {
        html {
            head {
                title { "Title" }
            }
            body {
                h1 { "Hello, World!" }
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let site = site()
        .dir(&std::env::current_dir().unwrap())
        .url(&Url::parse("http://localhost:8000").unwrap()) // TODO: get from rocket?!
        .build();

    let site_arc = Arc::new(RwLock::new(site));
    Site::load(site_arc.clone(), &vec![]);

    rocket::build()
        .manage(site_arc.clone())
        .mount("/", routes![thbs, tmaud])
        .mount("/", routes![index, api_page, robots_txt, sitemap_xml])
        .mount("/", FileServer::from("./public"))
        .attach(Template::fairing())
        .attach(AdHoc::config::<Config>())
}
