use bcrypt;
use bcrypt::{DEFAULT_COST, hash};
use diesel;
use diesel::prelude::*;
use rocket;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use serde_json;
use tera::Context;

use super::DbConn;
use self::models::*;
use self::forms::*;

mod forms;
pub mod models;

#[get("/login")]
fn login() -> Template {
    let context = Context::new();

    Template::render("auth/login", &context)
}

#[post("/login", data="<form>")]
fn authenticate(form: Form<LoginForm>, mut cookies: Cookies, conn: DbConn) -> Redirect {
    use super::schema::users::dsl::*;

    // Need to validate a few things
    //      1. Find user || error semantically
    //      2. Invalid pw || error semantically
    //      3. Already logged in, redirect. (also, don't allow in the UI)
    //
    //  * Think about User states. Need AuthenticatedUser?
    //  * Make illegal states unrepresentable....aka, UnAuthenticatedUser..?
    //  * Think about serialization...The password.. even hashed shouldn't be
    //      available to the client at all. Need to custom implement Serialize 
    //      trait for user?
    
    let form = form.get();
    let found_users = users.filter(email.eq(&form.email))
        .limit(1)
        .load::<User>(&*conn)
        .expect("Didn't find any users");

    let found_user = &found_users[0];

    if bcrypt::verify(&form.password, &found_user.password).unwrap() {
        let sessions = serde_json::to_string(&found_user);

        if sessions.is_ok() {
            let cookie = Cookie::build("sessions_auth".to_owned(), sessions.unwrap())
                .path("/")
                .finish();

            cookies.add_private(cookie);
        }
    }

    Redirect::to("/")
}

// TODO: Logout Path.
//
// TODO: Register Path.

pub fn routes() -> Vec<rocket::Route> {
    routes![authenticate, login]
}


