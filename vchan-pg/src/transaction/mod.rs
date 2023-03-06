mod user_post_thread;
mod create_user;
mod create_board;
mod user_reply_thread;
mod get_user_auth;
mod find_user;

use diesel::prelude::*;
pub type TransactionResult<T> = Result<T, TransactionError>;
pub trait Transaction<T=()> {
    fn execute(&self, conn: &mut PgConnection) -> TransactionResult<T>;
}

#[non_exhaustive]
#[derive(Debug)]
pub enum TransactionError {
    UserDisabled,
    Diesel(diesel::result::Error)
}

impl Serialize for TransactionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            TransactionError::UserDisabled => {
                serializer.serialize_unit_variant("TransactionError", 0, "UserDisabled")
            },
            TransactionError::Diesel(err) => {
                let mut tv = serializer.serialize_tuple_variant("TransactionError", 1, "Diesel", 1)?;
                tv.serialize_field(&err.to_string())?;
                tv.end()
            },
        }
    }
}

pub use create_user::*;
pub use create_board::*;
pub use get_user_auth::*;
use serde::{Serialize, ser::SerializeTupleVariant};
pub use user_post_thread::*;
pub use user_reply_thread::*;
pub use find_user::*;