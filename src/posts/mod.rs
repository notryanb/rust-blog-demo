pub mod models;
mod forms;

use self::models::*;
use self::forms::*;
use auth::models::*;
use super::DbConn;

use diesel;
use diesel::prelude::*;
use rocket;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use tera::Context;

#[get("/")]
fn index(user: User, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post_list = posts
        .order(id.desc())
        .load::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("posts", &post_list);
    context.add("user", &user);

    Template::render("posts/index", &context)
}

#[get("/show/<post_id>")]
fn show(user: User, post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("post", &post);
    context.add("user", &user);

    Template::render("posts/show", &context.as_json().unwrap())
}

#[get("/new")]
fn new(user: AuthenticatedUser) -> Template {
    let mut context = Context::new();

    context.add("user", &user);

    Template::render("posts/new", &context)
}

// TODO: Authenticate
#[get("/edit/<post_id>")]
fn edit(user: AuthenticatedUser, post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("post", &post);
    context.add("user", &user);

    Template::render("posts/edit", &context)
}

#[post("/create", data = "<form>")]
fn create(user: AuthenticatedUser, form: Form<CreatePostForm>, conn: DbConn) -> Redirect {
    use schema::posts;

    let post = form.get();

    let new_post = NewPost {
        user_id: user.0.id,
        title: &post.title,
        content: &post.content,
    };

    diesel::insert(&new_post)
        .into(posts::table)
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    Redirect::to("/")
}

#[post("/update", data = "<form>")]
fn update(user: AuthenticatedUser, form: Form<UpdatePostForm>, conn: DbConn) -> Redirect {
    use super::schema::posts::dsl::*;

    // TODO: Check auth.id == post.user_id
    // Redirect to root
    // Flash err -> Not Authorized for action

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

#[get("/delete/<post_id>")]
fn confirm_delete(user: AuthenticatedUser, post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;
    
    // TODO: Check auth.id == post.user_id
    // Redirect to root
    // Flash err -> Not Authorized for action
    // Need to change return type

    let mut context = Context::new();

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading post");

    context.add("post", &post);
    context.add("user", &user);

    Template::render("posts/delete", &context)
}

#[post("/destroy", data = "<form>")]
fn destroy(user: AuthenticatedUser, form: Form<DeletePostForm>, conn: DbConn) -> Redirect {
    use super::schema::posts::dsl::*;
    
    // TODO: Check auth.id == post.user_id
    // Redirect to root
    // Flash err -> Not Authorized for action
    // Need to change return type

    let data = form.get();
    let post = posts.find(&data.id)
        .get_result::<Post>(&*conn)
        .expect("Error loading post");

    diesel::delete(&post)
        .execute(&*conn)
        .expect("Error deleting post");

    // TODO: Flash success to main page that the post was deleted
    // Need to change return type
    Redirect::to("/")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, create, confirm_delete, destroy, edit, new, show, update]
}
