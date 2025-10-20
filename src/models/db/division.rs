use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::divisions;

#[derive(Queryable, Selectable)]
#[diesel(table_name = divisions)]
pub struct Division {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
