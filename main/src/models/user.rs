use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub roles: Vec<String>,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub password: &'a String,
    pub roles: &'a Vec<String>,
    pub first_name: &'a String,
    pub last_name: &'a String,
}

#[derive(Deserialize, Serialize)]
pub struct UserDto {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserDto {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub roles: Vec<String>,
    pub first_name: String,
    pub last_name: String,
}
