use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;


pub mod schema;
pub mod model;
pub mod transaction;
pub mod utils;
pub mod query;
pub mod repository;

pub fn create_pool(url: &str) -> Pool<ConnectionManager<PgConnection>>{
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder().max_size(32).build(manager).unwrap()
}