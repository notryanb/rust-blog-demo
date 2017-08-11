use schema::posts;
use std::time::SystemTime;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub published_at: Option<SystemTime>
}

#[derive(AsChangeset, Identifiable, Serialize)]
#[table_name="posts"]
#[changeset_options(treat_none_as_null="true")]
pub struct UpdatePost<'a> {
    pub id: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
    pub published_at: Option<SystemTime>
}

#[derive(AsChangeset, Insertable)]
#[table_name="posts"]
#[changeset_options(treat_none_as_null="true")]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published_at: Option<SystemTime>
}
