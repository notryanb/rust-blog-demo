use auth::models::{User};
use schema::posts;

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


