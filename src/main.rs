#[macro_use]
extern crate rocket;
use khulan::cms::helpers::add;
use khulan::routes::*;
use khulan::site;
use maud::{html, Markup};
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::Config;
use rocket_dyn_templates::{context, Template};

#[get("/hbs")]
fn thbs() -> Template {
    let context = context! { hello: format!("world {}", add(2, 2)) };
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
                (format!("world {}", add(2, 2)))
            }
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let dir = std::env::current_dir().unwrap();
    rocket::build()
        .manage(site(dir.clone()).await)
        .mount("/", routes![thbs, tmaud])
        .mount("/", routes![index, robots_txt, sitemap_xml])
        .mount("/", FileServer::from("./public"))
        .attach(Template::fairing())
        .attach(AdHoc::config::<Config>())
}
