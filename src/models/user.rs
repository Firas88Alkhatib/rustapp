use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a i32,
    pub name: &'a String,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
}
