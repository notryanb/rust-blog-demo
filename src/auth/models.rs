use rocket::outcome::Outcome::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};
// use serde_json;
// use serde::ser;
// use serde::ser::SerializeStruct;
use schema::users;

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a>{
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct UpdateUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str
}

// impl<'a, 'r> FromRequest<'a, 'r> for User{
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {
//         // Grab cookies
//         let request = request.clone();
//         let mut cookies = request.cookies();

//         if let Some(cookie) = cookies.get_private("session") {
//             if let user: Result<User, _> = serde_json::from_str(cookie.value());
//             if user.is_ok() {
//                 return Success(user.unwrap());
//             }
//         }

//         Success(User {
//             id:
//         })
//     }
// }

