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

#[derive(FromForm)]
pub struct DeletePostForm {
    pub id: i32,
}

#[derive(Serialize)]
pub struct InvalidFormMessage<'a> {
    pub name: &'a str,
    pub msg: &'a str
}

