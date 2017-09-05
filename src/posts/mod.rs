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
fn show(post_id: i32, conn: DbConn) -> Template {
    use super::schema::posts::dsl::*;

    let mut context = Context::new();

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    context.add("post", &post);

    Template::render("posts/show", &context.as_json().unwrap())
}

// TODO: Authenticate
#[get("/new")]
fn new(_user: AuthenticatedUser) -> Template {
    let context = Context::new();

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

    Template::render("posts/edit", &context)
}

// TODO: Authenticate
#[post("/create", data = "<form>")]
fn create(user: AuthenticatedUser, form: Form<CreatePostForm>, conn: DbConn) -> Redirect {
    use schema::posts;

    let post = form.get();

    let new_post = NewPost {
        user_id: 1, // Hard code user Id
        title: &post.title,
        content: &post.content,
    };

    diesel::insert(&new_post)
        .into(posts::table)
        .get_result::<Post>(&*conn)
        .expect("Error saving new post");

    Redirect::to("/")
}


// TODO: Authenticate
#[post("/update", data = "<form>")]
fn update(user: AuthenticatedUser, form: Form<UpdatePostForm>, conn: DbConn) -> Redirect {
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

// TODO: Delete Post
// TODO: Authenticate


pub fn routes() -> Vec<rocket::Route> {
    routes![index, create, edit, new, show, update]
}
