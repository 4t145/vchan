use diesel::prelude::*;
pub mod schema;
pub mod model;
pub mod transaction;
pub mod utils;

use crate::transaction::*;
fn main() {
    let mut conn = PgConnection::establish(env!("DATABASE_URL")).unwrap();
    // CreateBoard {
    //     name: "综合".to_string(),
    // }.execute(&mut conn).unwrap();
    let seed = CreateUser {
        flag: i64::MAX as i64,
        name: "SUPERADMIN".to_string(),
    }.execute(&mut conn).unwrap();
    let hexcode = hex::encode(seed);
    let seed = format!("[128]{hexcode}@SUPERADMIN");
    println!("{seed}");
}
// [128]7fc8b5ba52602c9989cc08bb393d896c6d53fe370b7b1b5a14d73c26e8574db031d7b442d94be5db5f2c5a76da684f5455b4e93e411f89407985c7486611d0f2@SUPERADMIN