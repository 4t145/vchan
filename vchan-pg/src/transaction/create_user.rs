use super::Transaction;
use super::TransactionError;
use super::TransactionResult;
use crate::model::*;
use crate::schema::*;
use crate::utils::vchan_hash;
use chrono::Utc;
use diesel::prelude::*;
use rand::prelude::*;

pub struct CreateUser {
    pub name: String,
    pub flag: i64,
}


impl Transaction<[u8; 64]> for CreateUser {
    fn execute(&self, conn: &mut PgConnection) -> TransactionResult<[u8; 64]> {
        // let post_insert = self.post.insert();
        conn.build_transaction().read_write().run(|conn| {
            let mut rng = rand::thread_rng();
            let mut seed = [0u8; 64];
            rng.fill(&mut seed);
            // 这里unsafe 可能受到大小端影响
            let cookiehash =
                unsafe { std::mem::transmute::<[u8; 32], [i32; 8]>(vchan_hash(&seed, &self.name)) };

            let user_insert = UserInsert {
                flag: self.flag,
                id: &self.name,
                cookiehash: &cookiehash,
                create_time: Utc::now().naive_utc(),
            };

            diesel::insert_into(users::table)
                .values(user_insert)
                .execute(conn)?;


            return Ok(seed);
        }).map_err(TransactionError::Diesel)
    }
}
