#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate diesel;
extern crate rocket_contrib;
extern crate rocket;
extern crate bloglib;

// Server
use rocket::request::Form;

// Routing
use rocket::response::Redirect;
use rocket_contrib::Template;

// DB
use diesel::prelude::*;
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
            create_post,
            show_posts
        ])
        .attach(Template::fairing())
        .launch();
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

    let post_list =  posts.load::<Post>(&*conn)
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
fn create_post(form: Form<Posting>, conn: DbConn) -> Redirect {
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
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    // Redirect to index
    Redirect::to("/")
}
