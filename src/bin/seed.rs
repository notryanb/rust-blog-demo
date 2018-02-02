extern crate bloglib;
extern crate diesel;
extern crate bcrypt;
#[macro_use] extern crate fake;

use bcrypt::{DEFAULT_COST, hash};
use bloglib::*;
use bloglib::posts::models::*;
use bloglib::auth::models::*;
use diesel::prelude::*;

fn main() {
    use bloglib::schema::posts::dsl::*;
    use bloglib::schema::users::dsl::*;

    let connection = create_db_pool().get().unwrap();
    let plain_text_pw = "testing";
    let hashed_password = match hash (plain_text_pw, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => panic!("Error hashing")
    };


    // Remove all users & posts from DB to start fresh
    diesel::delete(posts).execute(&*connection).expect("Error deleteing posts");
    diesel::delete(users).execute(&*connection).expect("Error deleteing users");

    // Randomly generate user info
    fn generate_user_info(pw: &str) -> BulkNewUser {
        BulkNewUser {
           first_name: fake!(Name.name),
           last_name: fake!(Name.name),
           email: fake!(Internet.free_email),
           password: pw.to_string(),
        }
    }

    // Randomly generate post info
    fn generate_post_info(user: User) -> BulkNewPost {
        let _title = &fake!(Lorem.sentence(1, 4))[..];
        let _content = &fake!(Lorem.paragraph(5,5))[..];

        BulkNewPost {
           user_id: user.id,
           title: _title.to_string(),
           content: _content.to_string(),
        }
    }

    // Create personal login
    let me = NewUser {
        first_name: "Ryan",
        last_name: "Blecher",
        email: "notryanb@gmail.com",
        password: &hashed_password[..],
    };
    
    diesel::insert(&me)
        .into(users)
        .execute(&*connection)
        .expect("Error inserting users");

    // Create 10 randomly generated users stored as a vec
    let new_user_list: Vec<BulkNewUser> = (0..10)
        .map( |_| generate_user_info(&hashed_password))
        .collect();

    // Insert that vec of users and get a vec back of the inserts
    let returned_users = diesel::insert(&new_user_list)
        .into(users)
        .get_results::<User>(&*connection)
        .expect("Error inserting users");

    // For each of the new users, create some posts
    let new_post_list: Vec<BulkNewPost> = returned_users
        .into_iter()
        .map(|user| generate_post_info(user))
        .collect();

    // Insert those posts
    diesel::insert(&new_post_list)
        .into(posts)
        .execute(&*connection)
        .expect("Error inserting posts");
}
