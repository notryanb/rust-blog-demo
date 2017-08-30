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
use bloglib::models::*;
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
struct UpdatePostForm {
    id: i32,
    title: String,
    content: String,
}

#[derive(FromForm)]
struct Posting {
    title: String,
    content: String,
}

fn main() {
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![
            files,
            index,
            new_post,
            create_post,
            show_post,
            edit_post,
            update_post
        ])
        .attach(Template::fairing())
        .launch();
}

// Routing
#[get("/")]
fn index(conn: DbConn) -> Template {
    use bloglib::schema::posts::dsl::*;

    let post_list =  posts.order(id.desc())
        .load::<Post>(&*conn)
        .expect("Error loading posts");

    let context = PostList {
        posts: post_list
    };

    Template::render("index", &context)
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/stylesheets/").join(file)).ok()
}

#[get("/show_post/<post_id>")]
fn show_post(post_id: i32, conn: DbConn) -> Template {
    use bloglib::schema::posts::dsl::*;

    let post = posts.find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    Template::render("show_post", &post)
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
    let b: &str = &*post.content;

    let new_post = NewPost {
        user_id: 1, // Hard code user Id
        title: t,
        content: b,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    Redirect::to("/")
}

#[post("/update_post", data = "<form>")]
fn update_post(form: Form<UpdatePostForm>, conn: DbConn) -> Redirect {
    use bloglib::schema::posts::dsl::*;

    let data = form.get();

    let update_post = UpdatePost {
        user_id: None,
        title: &data.title[..],
        content: &data.content[..],
        published: false,
    };

    diesel::update(posts.find(data.id))
        .set(&update_post)
        .get_result::<Post>(&*conn)
        .expect("Error updating Post");

    Redirect::to("/")
}
