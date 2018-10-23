#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;

use rocket::response::NamedFile;

#[get("/")]
fn home() -> io::Result<NamedFile> {
    NamedFile::open("src/views/landing.html")
}

#[catch(404)]
fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("src/views/error.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![home])
    .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
