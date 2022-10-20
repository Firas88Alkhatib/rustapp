use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a String,
    pub last_name: &'a String,
}

#[derive(Deserialize, Serialize)]
pub struct UserDto {
    pub first_name: String,
    pub last_name: String,
}
