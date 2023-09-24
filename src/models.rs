use serde::{Deserialize, Serialize};

use crate::schema::links;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = links)]
pub struct Links {
    pub id: String,
    pub url: String,
    pub count: i32,
}
