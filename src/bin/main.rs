#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate bloglib;
extern crate rocket;
extern crate rocket_contrib;

use bloglib::*;
use bloglib::{posts};

use rocket_contrib::Template;
use rocket::response::{Redirect, NamedFile};

use std::path::{Path, PathBuf};


fn main() {
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![index, files])
        .mount("/posts", posts::routes())
        .attach(Template::fairing())
        .launch();
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/stylesheets/").join(file)).ok()
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/posts")
}

