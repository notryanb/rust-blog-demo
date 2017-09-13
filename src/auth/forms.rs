#[derive(Debug, FromForm, Serialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromForm, Serialize)]
pub struct RegisterForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

