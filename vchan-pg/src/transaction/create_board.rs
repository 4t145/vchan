use super::Transaction;
use super::TransactionError;
use super::TransactionResult;
use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;


pub struct CreateBoard {
    pub name: String,
}


impl Transaction<i32> for CreateBoard {
    fn execute(&self, conn: &mut PgConnection) -> TransactionResult<i32> {
        conn.build_transaction().read_write().run(|conn| {
            let board_insert = BoardInsert {
                name: self.name.as_str()
            };

            let inserted_board_id = diesel::insert_into(boards::table)
                .values(board_insert).returning(boards::id).get_result(conn)?;
            return Ok(inserted_board_id);
        }).map_err(TransactionError::Diesel)
    }
}