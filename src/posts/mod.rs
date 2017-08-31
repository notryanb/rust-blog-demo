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

#[derive(Serialize)]
pub struct TemplateContext {
        pub data: String
}

#[derive(Serialize)]
pub struct PostList {
        pub posts: Vec<Post>
}

#[get("/")]
fn index(conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let post_list =  posts.order(id.desc())
        .load::<Post>(&*conn)
        .expect("Error loading posts");

    let context = PostList {
        posts: post_list
    };

    Template::render("index", &context)
}

#[get("/show/<post_id>")]
fn show(post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let post = posts.find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    Template::render("show_post", &post)
}

#[get("/new")]
fn new() -> Template {
    let context = TemplateContext {
        data: String::from("Figure out how to not need this arg")
    };

    Template::render("new_post", &context)
}

#[get("/edit/<post_id>")]
fn edit(post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let post = posts.find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    Template::render("edit_post", &post)
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

