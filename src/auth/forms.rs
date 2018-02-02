use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug, FromForm, Serialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub struct UserField(pub String);
pub struct FieldPresenceError<'a> { 
    pub msg: &'a str,
}

impl<'v> FromFormValue<'v> for UserField {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<UserField, &'v RawStr> {
        match form_value.percent_decode() {
            Ok(ref val) if val.is_empty() => Err(form_value),
            Ok(ref val) => Ok(UserField(val.to_string())),
            Err(_val) => Err(form_value)
        }
    }
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub id: Option<i32>,
    pub first_name: Option<UserField>,
    pub last_name: Option<UserField>,
    pub email: Option<UserField>,
    pub password: Option<UserField>,
    pub password_confirm: Option<UserField>,
}

impl RegisterForm {
    pub fn validate_fields_presence(&self) -> Result<&Self, FieldPresenceError> {
        if self.first_name.is_some() &&
            self.last_name.is_some() &&
            self.email.is_some() &&
            self.password.is_some() &&
            self.password_confirm.is_some()
            {
                return Ok(self);
            }

        Err(FieldPresenceError { msg: "All fields must be filled out" })
    }
}
