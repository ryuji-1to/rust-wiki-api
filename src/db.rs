use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct PageRecord {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}
