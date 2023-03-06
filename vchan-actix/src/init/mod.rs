use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use self::config::Config;

mod config;

#[derive(Debug, Clone)]
pub struct AppData {
    pub pg_pool: Pool<ConnectionManager<PgConnection>>,
}

impl AppData {
    pub fn init() -> Self {
        let cfg = Config::load();
        Self {
            pg_pool: init_pg_pool(&cfg),
        }
    }
}
pub fn init_pg_pool(cfg: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(&cfg.r2d2.db_url);
    Pool::builder()
        .max_size(cfg.r2d2.max_connection as u32)
        .build(manager)
        .expect("fail to build r2d2 pool")
}

