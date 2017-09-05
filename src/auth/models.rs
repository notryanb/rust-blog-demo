// Influenced heavily from
// https://git.bananium.fr/eijebong/bananium.rs/blob/master/src/auth/models.rs
// Look into refactoring...

use rocket::outcome::Outcome::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde_json;
use serde::ser;
use serde::ser::SerializeStruct;
use schema::users;

// Custom User serialization? We don't want the password being exposed anywhere in the code that it
// doesn't need to be.
// Also, We can encode another field that checks if the id == AnonymousUser's ID, which will
// give us the state of an authenticated user or anonymous user

#[derive(Debug, Identifiable, Queryable, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing)] pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
}

impl ser::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut s = serializer.serialize_struct("User", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("first_name", &self.first_name)?;
        s.serialize_field("last_name", &self.last_name)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("is_anonymous", &(self.id == -1))?;
        s.end()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {
        let request = request.clone();
        let mut cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("sessions_auth") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_ok() {
                return Success(user.unwrap());
            }
        }

        Success(User {
            id: -1,
            first_name: "".to_owned(),
            last_name: "".to_owned(),
            email: "".to_owned(),
            password: "".to_owned(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, ()> {
        let request = request.clone();
        let mut cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("sessions_auth") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_err() {
                return Failure((Status::raw(600), ()));
            }

            return Success(AuthenticatedUser(user.unwrap()));
        }

        Failure((Status::raw(600), ()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymousUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AnonymousUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AnonymousUser, ()> {
        let request = request.clone();
        let mut cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("sessions_auth") {
            let user: Result<User, _> = serde_json::from_str(cookie.value());
            if user.is_ok() {
                return Failure((Status::raw(601), ()));
            }
        }

        Success(AnonymousUser(User {
            id: -1,
            first_name: "".to_owned(),
            last_name: "".to_owned(),
            email: "".to_owned(),
            password: "".to_owned(),
        }))
    }
}
