use schema::users;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a>{
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct UpdateUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str
}

