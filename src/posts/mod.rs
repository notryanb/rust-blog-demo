use self::models::*;
use self::forms::*;
use auth::models::*;
use DbConn;

use diesel;
use diesel::prelude::*;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use tera::Context;

pub mod models;
mod forms;

#[get("/")]
fn index(user: User, conn: DbConn) -> Template {
    let mut context = Context::new();

    let post_list = PostWithAuthor::load_all(conn);

    context.add("posts", &post_list);
    context.add("user", &user);

    Template::render("posts/index", &context)
}

#[get("/show/<post_id>")]
fn show(user: User, flash: Option<FlashMessage>, post_id: i32, conn: DbConn) -> Template {
    let mut context = Context::new();

    if flash.is_some() {
        let flash_val = flash.unwrap();
        let message = InvalidFormMessage {
            name: &flash_val.name(),
            msg: &flash_val.msg()
        };

        context.add("flash", &message);
    }
   
    let post_with_author = PostWithAuthor::find(post_id, conn);

    context.add("post", &post_with_author);
    context.add("user", &user);

    Template::render("posts/show", &context)
}

#[get("/new")]
fn new(user: AuthenticatedUser) -> Template {
    let mut context = Context::new();

    context.add("user", &user);

    Template::render("posts/new", &context)
}

#[get("/edit/<post_id>")]
fn edit(user: AuthenticatedUser, post_id: i32, conn: DbConn) -> Result<Template, Flash<Redirect>>{
    use super::schema::posts::dsl::*;

    let mut context = Context::new();
    context.add("user", &user);

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading posts");

    if user.0.id != post.user_id {
        let url = &format!("/posts/show/{}", post_id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }

    context.add("post", &post);

    Ok(Template::render("posts/edit", &context))
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
fn update(user: AuthenticatedUser, form: Form<UpdatePostForm>, conn: DbConn) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::posts::dsl::*;

    let form = form.get();

    if user.0.id != form.user_id {
        let url = &format!("/posts/show/{}", form.id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }

    let update_post = UpdatePost {
        user_id: None,
        title: &form.title[..],
        content: &form.content[..],
        published: false,
    };

    diesel::update(posts.find(form.id))
        .set(&update_post)
        .get_result::<Post>(&*conn)
        .expect("Error updating Post");

    // TODO: Flash success to main page that the post was edited
    // Need to change return type
    Ok(Redirect::to("/"))
}

#[get("/delete/<post_id>")]
fn confirm_delete(user: AuthenticatedUser, post_id: i32, conn: DbConn) -> Result<Template, Flash<Redirect>> {
    use super::schema::posts::dsl::*;
    
    let mut context = Context::new();

    let post = posts
        .find(post_id)
        .get_result::<Post>(&*conn)
        .expect("Error loading post");
    
    if user.0.id != post.user_id {
        let url = &format!("/posts/show/{}", post_id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }
    
    context.add("user", &user);
    context.add("post", &post);

    Ok(Template::render("posts/delete", &context))
}

#[post("/destroy", data = "<form>")]
fn destroy(user: AuthenticatedUser, form: Form<DeletePostForm>, conn: DbConn) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::posts::dsl::*;
    
    let form = form.get();
    
    let post = posts.find(&form.id)
        .get_result::<Post>(&*conn)
        .expect("Error loading post");
    
    if user.0.id != post.user_id {
        let url = &format!("/posts/show/{}", post.id)[..];
        return Err(Flash::error(Redirect::to(url), "Unauthorized Action"))
    }

    diesel::delete(&post)
        .execute(&*conn)
        .expect("Error deleting post");

    // TODO: Flash success to main page that the post was deleted
    // Need to change return type
    Ok(Redirect::to("/"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, create, confirm_delete, destroy, edit, new, show, update]
}
