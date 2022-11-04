use crate::models::role::Role;
use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Deserialize, Serialize, Debug, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub password: &'a String,
    pub first_name: &'a String,
    pub last_name: &'a String,
}

#[derive(Deserialize, Serialize)]
pub struct UserDto {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<Role>,
}

#[derive(Deserialize, Serialize)]
pub struct UserOutDto {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<Role>,
    #[serde(skip_serializing)]
    pub password: String,
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
