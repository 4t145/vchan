mod thread_page;
mod post_page;
mod batch;
mod board;

use std::num::NonZeroUsize;

use diesel::PgConnection;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

pub(crate) use crate::model::*;
pub(crate) use crate::schema::*;
pub(crate) use diesel::prelude::*;

pub use thread_page::*;
pub use post_page::*;
pub use batch::*;
pub use board::*;

#[derive(Debug)]
pub enum QueryError {
    Diesel(diesel::result::Error),
    Limit(usize)
}

impl Serialize for QueryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            QueryError::Diesel(err) => serializer.serialize_str(&err.to_string()),
            QueryError::Limit(limit) => serializer.serialize_str(format!("limit {limit}").as_str()),
        }
    }
}


type QueryResult<T> = Result<T, QueryError>;

pub trait Query<R> {
    fn query(&self, conn: &mut PgConnection) -> QueryResult<R>;
}


pub trait BatchQuery {
    type Key: for <'a> Deserialize<'a>;
    type Value: Serialize;
    const LIMIT: usize = 50;
    fn query(keys: &[Self::Key], conn: &mut PgConnection) -> QueryResult<Vec<Self::Value>>;
}

pub struct Batch<B: BatchQuery>(pub Vec<B::Key>);

impl<B: BatchQuery> Query<Vec<B::Value>> for Batch<B>
{
    fn query(&self, conn: &mut PgConnection) -> QueryResult<Vec<B::Value>> {
        if self.0.len() > B::LIMIT {
            return Err(QueryError::Limit(B::LIMIT))
        }
        B::query(&self.0, conn)
    }
}


#[derive(Debug, Clone, Copy, Deserialize)]
pub struct PaginationQuery<V>
{
    #[serde(flatten)]
    pub view: V,
    #[serde(flatten)]
    pub pagination: Pagination
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde_as]
pub struct Pagination {
    pub page: NonZeroUsize,
    pub pagesize: NonZeroUsize
}

impl Pagination {
    fn limit_and_skip(&self) -> (i64, i64){
        let limit = self.pagesize.get();
        let skip = (self.page.get()-1)*limit;
        return (limit as i64, skip as i64)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PaginationView<Item> {
    pub items: Vec<Item>,
    pub pagination: Pagination
}


