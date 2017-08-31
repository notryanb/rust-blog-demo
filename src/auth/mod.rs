use bcrypt;
use bcrypt::{DEFAULT_COST, hash};
use diesel;
use diesel::prelude::*;
use rocket;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use tera::Context;

use super::DbConn;
use self::models::*;
use self::forms::*;

mod forms;
pub mod models;

#[get("/login")]
fn login() -> Template {
    let context = Context::new();

    Template::render("auth/login", &context)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}


