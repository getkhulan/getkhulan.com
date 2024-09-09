#[macro_use]
extern crate rocket;
use khulan;

use maud::{html, Markup};
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::Config;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let context = context! { hello: format!("world {}", khulan::add(2, 2)) };
    Template::render("index", &context)
}

#[get("/hbs")]
fn thbs() -> Template {
    let context = context! { hello: format!("world {}", khulan::add(2, 2)) };
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
                (format!("world {}", khulan::add(2, 2)))
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, thbs, tmaud])
        .mount("/", routes![khulan::robots_txt, khulan::sitemap_xml])
        .mount("/", FileServer::from("./public"))
        .attach(Template::fairing())
        .attach(AdHoc::config::<Config>())
}
