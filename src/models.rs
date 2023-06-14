use diesel::prelude::*;

use crate::schema::events;

#[derive(Queryable, Selectable)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub description: &'a str,
}
