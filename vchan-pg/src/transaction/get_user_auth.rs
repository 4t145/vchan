use super::Transaction;
use super::TransactionError;
use super::TransactionResult;

use crate::model::*;
use ::serde::Deserialize;
use juniper::serde::Serialize;
use permission_group::PermissonGroup;

use crate::schema::*;
use crate::utils::vchan_hash;
use chrono::*;
use diesel::prelude::*;
pub struct GetUserPermission<'t> {
    pub cookie: &'t UserCookie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermission {
    pub user_id: String,
    pub permission_group: u64,
    pub banned: bool,
    pub valid: bool,
}

impl UserPermission {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.valid
    }
    #[inline]
    pub fn is_not_banned(&self) -> bool {
        self.valid && !self.banned
    }
    #[inline]
    pub fn has_permission(&self, permission_group: PermissonGroup) -> bool {
        self.permission_group & permission_group == permission_group
    }
}

impl Transaction<UserPermission> for GetUserPermission<'_> {
    fn execute(&self, conn: &mut PgConnection) -> TransactionResult<UserPermission> {
        conn.build_transaction()
            .read_write()
            .run(|conn| {
                let mut valid = false;
                let (user_id, flag, hash_db, ban_time): (
                    String,
                    i64,
                    Vec<Option<i32>>,
                    Option<NaiveDateTime>,
                ) = users::table
                    .filter(users::id.eq(&self.cookie.name))
                    .select((
                        users::id,
                        users::flag,
                        users::cookiehash,
                        users::ban_time.nullable(),
                    ))
                    .get_result::<(String, i64, Vec<Option<i32>>, Option<NaiveDateTime>)>(
                        conn,
                    )?;

                let hash_calc = unsafe {
                    std::mem::transmute::<[u8; 32], [i32; 8]>(vchan_hash(
                        self.cookie.cookieseed,
                        &self.cookie.name,
                    ))
                };

                for (db, calc) in hash_db.iter().zip(hash_calc.iter()) {
                    valid = Some(*calc) == *db;
                }

                return Ok(UserPermission {
                    user_id,
                    permission_group: flag as u64,
                    valid,
                    banned: ban_time.is_some(),
                });
            })
            .map_err(TransactionError::Diesel)
    }
}
