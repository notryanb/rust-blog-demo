use models::Post;

#[derive(Serialize)]
pub struct TemplateContext {
    pub data: String
}

#[derive(Serialize)]
pub struct PostList {
    pub posts: Vec<Post>
}

