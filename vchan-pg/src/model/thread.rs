use serde::Serialize;

use super::*;

#[derive(Debug, Builder, Default, Insertable)]
#[diesel(table_name = threads)]
#[builder(setter(into))]
pub struct ThreadInsert {
    // id: Option<i32>,
    pub primary_post_id: i64,
    pub board_id: i32
}

#[derive(Debug, Builder, Default, Identifiable, AsChangeset)]
#[diesel(table_name = threads)]
pub struct ThreadUpdate {
    pub id: i32,
    pub primary_post_id: i64,
    pub board_id: i32
}

#[derive(Debug, Builder, Default, Queryable, Serialize)]
#[diesel(table_name = threads)]
pub struct Thread {
    pub id: i32,
    pub primary_post_id: i64,
    pub update_time: NaiveDateTime,
    pub board_id: i32,
}