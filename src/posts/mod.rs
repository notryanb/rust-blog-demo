pub mod models;
mod forms;

use self::models::*;
use self::forms::*;
use super::DbConn;

use diesel;
use diesel::prelude::*;
use rocket;
use rocket::request::Form;
use rocket::response::{Redirect};
use rocket_contrib::Template;
use tera::Context;

#[get("/")]
fn index(conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post_list =  posts.order(id.desc())
        .load::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("posts", &post_list);

    Template::render("posts/index", &context)
}

#[get("/show/<post_id>")]
fn show(post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post = posts.find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("post", &post);

    Template::render("posts/show", &context.as_json().unwrap())
}

#[get("/new")]
fn new() -> Template {
    let context = Context::new();

    Template::render("posts/new", &context)
}

#[get("/edit/<post_id>")]
fn edit(post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post = posts.find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("post", &post);

    Template::render("posts/edit", &context)
}

#[post("/create", data = "<form>")]
fn create(form: Form<CreatePostForm>, conn: DbConn) -> Redirect {
    use schema::posts;

    let post = form.get();

    let new_post = NewPost {
        user_id: 1, // Hard code user Id
        title: &post.title,
        content: &post.content,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    Redirect::to("/")
}

#[post("/update", data = "<form>")]
fn update(form: Form<UpdatePostForm>, conn: DbConn) -> Redirect {
    use super::schema::posts::dsl::*;

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

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        create,
        edit,
        new,
        show,
        update
    ]
}
