use rocket::request::FromFormValue;
use rocket::http::RawStr;

#[derive(Debug, FromForm, Serialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub struct UserField(pub String);

impl<'v> FromFormValue<'v> for UserField {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<UserField, &'v RawStr> {
        println!("THE VALUE COMING THROUGH: |{}|", &form_value);
        match form_value.percent_decode() {
            Ok(ref val) if val.is_empty() => Err(form_value),
            Ok(ref val) => Ok(UserField(val.to_string())),
            Err(ref val) => Err(form_value)
        }
    }
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub first_name: Option<UserField>,
    pub last_name: Option<UserField>,
    pub email: Option<UserField>,
    pub password: Option<UserField>,
    pub password_confirm: Option<UserField>,
}

