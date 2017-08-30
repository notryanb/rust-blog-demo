use schema::posts;
use schema::users;

// Posts
#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(AsChangeset)]
#[table_name="posts"]
pub struct UpdatePost<'a> {
    pub user_id: Option<i32>,
    pub title: &'a str,
    pub content: &'a str,
    pub published: bool,
}

// Users
#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String
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
