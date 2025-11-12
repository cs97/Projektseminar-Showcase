use diesel::Insertable;
use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::comment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    //pub id: i32,
    pub title: String,
    pub body: String,
}
