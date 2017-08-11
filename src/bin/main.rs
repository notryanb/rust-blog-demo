#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate diesel;
extern crate rocket_contrib;
extern crate rocket;
extern crate bloglib;

//STD
use std::path::{Path, PathBuf};

// Server
use rocket::request::Form;

// Routing
use rocket::response::{Redirect, NamedFile};
use rocket_contrib::Template;

// DB
use diesel::prelude::*;
use bloglib::models::{Post, NewPost, UpdatePost};
use bloglib::*;
use bloglib::schema::posts;

#[derive(Serialize)]
struct TemplateContext {
    data: String
}

#[derive(Serialize)]
struct PostList {
    posts: Vec<Post>
}

#[derive(FromForm)]
struct UpdatedPost {
    id: i32,
    title: String,
    body: String,
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
            files,
            index,
            new_post,
            create_post,
            edit_post,
            show_posts,
            update_post
        ])
        .attach(Template::fairing())
        .launch();
}

// Routing
#[get("/")]
fn index() -> Template {

    Template::render("index", TemplateContext { data: String::from("Empty")})
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/stylesheets/").join(file)).ok()
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

#[get("/edit_post/<post_id>")]
fn edit_post(post_id: i32, conn: DbConn) -> Template {
    use bloglib::schema::posts::dsl::*;

    let post = posts.find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    Template::render("edit_post", &post)
}

#[post("/create_post", data = "<form>")]
fn create_post(form: Form<Posting>, conn: DbConn) -> Redirect {
    let post = form.get();
    let t: &str = &*post.title;
    let b: &str = &*post.body;

    let new_post = NewPost {
        title: t,
        body: b,
        published_at: None
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    Redirect::to("/")
}

#[post("/update_post", data = "<form>")]
fn update_post(form: Form<UpdatedPost>, conn: DbConn) -> Redirect {
    use bloglib::schema::posts::dsl::*;

    let data = form.get();

    diesel::update(posts.find(data.id))
        .set(&UpdatePost {
            id: data.id,
            title: &data.title[..],
            body: &data.body[..],
            published: false,
            published_at: None
        })
        .get_result::<Post>(&*conn)
        .expect("Error updating Post");

    Redirect::to("/")
}

