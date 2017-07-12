#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate rocket;
extern crate diesel;
extern crate bloglib;
extern crate r2d2;
extern crate r2d2_diesel;
// extern crate serde_json;

// Server
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// Routing
use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;

// Std


// DB
use diesel::prelude::*;
use diesel::update;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;
use bloglib::models::*;
use bloglib::*;

#[derive(Serialize)]
struct TemplateContext {
    data: String
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index
        ])
        .launch();
}

// DB Setup
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}


pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}

// Routing
#[get("/")]
fn index() -> Template {
    //Need TemplateContext Struct!
    let context = TemplateContext {
        data: String::from("A String")
    };

    Template::render("index", &context)
}
