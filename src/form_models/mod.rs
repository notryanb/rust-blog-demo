#[derive(FromForm)]
pub struct UpdatePostForm {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(FromForm)]
pub struct CreatePostForm {
    pub title: String,
    pub content: String,
}
