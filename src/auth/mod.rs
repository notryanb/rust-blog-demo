use bcrypt;
use diesel::prelude::*;
use rocket;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};

use rocket::response::{Flash, Redirect};
use rocket::request::{FlashMessage, Form};
use serde_json;
use tera::Context;

use super::DbConn;
use self::models::*;
use self::forms::*;

mod forms;
pub mod models;

#[derive(Serialize)]
struct InvalidFormMessage<'a> {
    name: &'a str,
    msg: &'a str
}

#[get("/login")]
fn login(user: AnonymousUser, flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();
    context.add("user", &user);

    if flash.is_some() {
        let flash_val = flash.unwrap();
        let message = InvalidFormMessage {
            name: &flash_val.name(),
            msg: &flash_val.msg()
        };

        context.add("flash", &message);
    }

    Template::render("auth/login", &context)
}

#[post("/login", data = "<form>")]
fn authenticate(
    user: AnonymousUser,
    form: Form<LoginForm>,
    mut cookies: Cookies,
    conn: DbConn,
) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::users::dsl::*;

    if cookies.get("sessions_auth").is_none() {
        let form = form.get();
        let found_users = users
            .filter(email.eq(&form.email))
            .limit(1)
            .load::<User>(&*conn)
            .expect("Didn't find any users");

        if found_users.len() == 0 {
            return Err(Flash::error(Redirect::to("/auth/login"), "Invalid credentials"))
        }

        let found_user = &found_users[0];

        if bcrypt::verify(&form.password, &found_user.password).unwrap() {
            let sessions = serde_json::to_string(&found_user);

            if sessions.is_ok() {
                let cookie = Cookie::build("sessions_auth".to_owned(), sessions.unwrap())
                    .path("/")
                    .finish();

                cookies.add_private(cookie);
            }
        } else {
            return Err(Flash::error(Redirect::to("/auth/login"), "Invalid credentials"))
        }


    }

    Ok(Redirect::to("/"))
}

#[get("/logout")]
fn logout(user: AuthenticatedUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("sessions_auth"));
    Redirect::to("/")
}

#[get("/register")]
fn register(user: AnonymousUser, flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();
    context.add("user", &user);

    if flash.is_some() {
        let flash_val = flash.unwrap();
        let message = InvalidFormMessage {
            name: &flash_val.name(),
            msg: &flash_val.msg()
        };

        context.add("flash", &message);
    }

    Template::render("auth/register", &context)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![authenticate, login, logout]
}
