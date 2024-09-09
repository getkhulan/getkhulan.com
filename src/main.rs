#[macro_use] extern crate rocket;
use khulan;

use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template
{
    let context = context! { hello: format!("world {}", khulan::add(2, 2)) };
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/", FileServer::from("./public"))
        .attach(Template::fairing())
}