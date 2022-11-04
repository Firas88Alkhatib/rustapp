use crate::schema::roles;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Deserialize, Serialize, Debug, Queryable, AsChangeset, Clone)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
}
