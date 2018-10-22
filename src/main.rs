#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;

use rocket::Request;
use rocket::response::NamedFile;

#[get("/")]
fn home() -> io::Result<NamedFile> {
    NamedFile::open("templates/landing.html")
}

#[catch(404)]
fn not_found(_req: &Request) -> String { "Sorry, that url doesnt exist!".to_owned() }

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![home])
    .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
