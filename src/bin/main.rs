#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate bloglib;
extern crate rocket;
extern crate rocket_contrib;

use bloglib::*;
use bloglib::{auth, posts};

use rocket::{Catcher, Error};
use rocket_contrib::Template;
use rocket::request::Request;
use rocket::response;
use rocket::response::{NamedFile, Redirect, Responder};

use std::path::{Path, PathBuf};

fn redirect_login<'r>(_: Error, r: &'r Request) -> response::Result<'r> {
    Redirect::to("/auth/login").respond_to(r)
}

fn redirect_root<'r>(_: Error, r: &'r Request) -> response::Result<'r> {
        Redirect::to("/").respond_to(r)
}

fn main() {
    let login = Catcher::new(600, redirect_login);
    let root = Catcher::new(601, redirect_root);

    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![index, files])
        .mount("/auth", auth::routes())
        .mount("/posts", posts::routes())
        .attach(Template::fairing())
        .catch(vec![login, root])
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
