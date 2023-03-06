use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};

use super::*;

#[derive(Debug, Builder, Default, Insertable)]
#[diesel(table_name = boards)]
#[builder(setter(into))]
pub struct BoardInsert<'a> {
    // id: Option<i32>,
    pub name: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Board {
    pub id: i32,
    pub name: String,
}