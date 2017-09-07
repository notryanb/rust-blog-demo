#[derive(Debug, FromForm, Serialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
