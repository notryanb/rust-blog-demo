use serde::ser;
use serde::ser::SerializeStruct;

use auth::models::User;
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

#[derive(Debug, Queryable)]
pub struct PostWithAuthor{
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub first_name: String,
    pub last_name: String,
}

impl ser::Serialize for PostWithAuthor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut s = serializer.serialize_struct("PostWithAuthor", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("user_id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("content", &self.content)?;
        s.serialize_field("published", &self.published)?;
        s.serialize_field("display_name", &self.display_name())?;
        s.end()
    }
}

impl PostWithAuthor {
    pub fn display_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost<'a> {
    pub user_id: Option<i32>,
    pub title: &'a str,
    pub content: &'a str,
    pub published: bool,
}
