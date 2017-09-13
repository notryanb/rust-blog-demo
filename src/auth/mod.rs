use bcrypt;
use bcrypt::{DEFAULT_COST, hash};
use diesel;
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
fn signup(user: AnonymousUser, flash: Option<FlashMessage>) -> Template {
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

#[post("/register", data = "<form>")]
fn register(
    user: AnonymousUser,
    form: Form<RegisterForm>,
    conn: DbConn
) -> Result<Redirect, Flash<Redirect>> {
    use super::schema::users::dsl::*;
    use schema::users;

    let mut context = Context::new();
    context.add("user", &user);
    

    // STEPS
    // 1 - Validate presence of all fields
   
    // Validation should return Result<FormType>
    // see if we can chain the logic right on to execute the redirect
    // or unwrap into the var

    let form = match form.get().validate_fields_presence().as_ref() {
        Ok(ref val) => val,
        Err(e) => Err(Flash::error(Redirect::to("/auth/register"), e.msg))
    };
    
    // 2 - Validate no other User with that email

    let found_user = users.filter(email.eq(&form.email.as_ref().unwrap().0))
        .get_results::<User>(&*conn)
        .expect("Error loading users");

    if found_user.len() > 0 {
        return Err(Flash::error(Redirect::to("/auth/register"), "Email already taken"))
    }

    // 3 - Validate PW == PW_CONFIRM
    
    if &form.password.as_ref().unwrap().0 != &form.password_confirm.as_ref().unwrap().0 {
        return Err(Flash::error(Redirect::to("/auth/register"), "Passwords must match"))
    }

    // 4 - Hash the PW

    let secured_password = match hash (&form.password.as_ref().unwrap().0, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!("Error hashing")
    };

    // 5 - Insert Email & Username into DB

    let new_user = NewUser {
        first_name: &form.first_name.as_ref().unwrap().0,
        last_name: &form.last_name.as_ref().unwrap().0,
        email: &form.email.as_ref().unwrap().0,
        password: &secured_password
    };

    diesel::insert(&new_user).into(users::table)
        .get_result::<User>(&*conn)
        .expect("Error inserting user");

    Ok(Redirect::to("/auth/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![authenticate, login, logout, signup, register]
}
