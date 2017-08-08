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

// Std
use std::ops::Deref;

// Server
use rocket::request::{self, FromRequest, Form};
use rocket::http::Status;

// Routing
use rocket::{Request, State, Outcome};
use rocket::response::Redirect;
use rocket_contrib::Template;

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
        .manage(create_db_pool())
        .mount("/", routes![
            index,
            new_post,
            show_posts
        ])
        .attach(Template::fairing())
        .launch();
}


pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
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
fn show_posts(conn: DbConn) -> Template {
    use bloglib::schema::posts::dsl::*;

    let post_list =  posts.load::<Post>(&conn)
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

// #[post("/create_post", data = "<form>")]
// fn create_post(form: Form<Posting>, conn: DbConn) -> Redirect {
//     // Take post object and insert into DB
//     use bloglib::schema::posts;

//     let post = form.get();
//     let t: &str = &*post.title;
//     let b: &str = &*post.body;

//     let new_post = NewPost {
//         title: t,
//         body: b
//     };

//     diesel::insert(&new_post).into(posts::table)
//         .get_result::<Post>(&conn)
//         .expect("Error saving new post");

//     // Redirect to index
//     Redirect::to("/")
// }
