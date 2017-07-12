#![feature(plugin, custom_derive)]
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
use rocket::request::{Outcome, FromRequest, Form};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// Routing
use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;

// Std

// DB
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;
use bloglib::models::{Post, NewPost};
use bloglib::*;

#[derive(Serialize)]
struct TemplateContext {
    data: String
}

#[derive(Serialize)]
struct PostList {
    posts: Vec<Post>
}

#[derive(FromForm)]
struct Posting {
    title: String,
    body: String,
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            new_post,
            create_post,
            show_posts
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

#[get("/show_posts")]
fn show_posts(db: DB) -> Template {
    use bloglib::schema::posts::dsl::*;

    let post_list =  posts.load::<Post>(db.conn())
        .expect("Error loading posts");

    let context = PostList {
        posts: post_list
    };

    Template::render("show_posts", &context)
}

#[get("/new_post")]
fn new_post() -> Template {
    let context = TemplateContext {
        data: String::from("Figure out how to not need this arg")
    };

    Template::render("new_post", &context)
}

#[post("/create_post", data = "<form>")]
fn create_post(form: Form<Posting>, db: DB) -> Redirect {
    // Take post object and insert into DB
    use bloglib::schema::posts;

    let post = form.get();
    let t: &str = &*post.title;
    let b: &str = &*post.body;

    let new_post = NewPost {
        title: t,
        body: b
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result::<Post>(db.conn())
        .expect("Error saving new post");

    // Redirect to index
    Redirect::to("/")
}
