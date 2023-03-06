use serde::Serialize;

use super::*;

#[derive(Debug,  Clone)]
pub struct PostBody<'a> {
    pub content: &'a str,
    pub flag: i64,
    pub thread_id: i32,
    pub thread_path: &'a [i64],
    pub email: Option<&'a str>,
    pub title: Option<&'a str>,
    pub author: Option<&'a str>,
}

#[derive(Debug, Builder, Default, Insertable, Clone)]
#[diesel(table_name = posts)]
pub struct PostInsert<'a> {
    pub user_id: &'a str,
    pub content: &'a str,
    pub flag: i64,
    pub thread_id: i32,
    pub thread_path: &'a [i64],
    pub email: Option<&'a str>,
    pub title: Option<&'a str>,
    pub author: Option<&'a str>,
}


#[derive(Debug, Selectable)]
#[diesel(table_name = posts)]
pub struct PostSelect {
    pub id: Option<i64>,
    pub user_id: Option<i32>,
    pub content: String,
    pub flag: i64,
    pub create_time: NaiveDateTime,
    pub deleted: bool,
    pub thread_id: i32,
    pub thread_path: Vec<Option<i64>>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i64,
    pub user_id: String,
    pub content: String,
    pub flag: i64,
    pub create_time: NaiveDateTime,
    pub deleted: bool,
    pub thread_id: i32,
    pub thread_path: Vec<Option<i64>>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

