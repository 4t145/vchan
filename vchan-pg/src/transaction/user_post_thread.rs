use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

use super::get_user_auth::UserPermission;
use super::Transaction;
use super::TransactionError;
use super::TransactionResult;

#[derive(Debug)]
pub struct UserPostThread<'a> {
    pub post: PostBody<'a>,
    pub permission: UserPermission,
    pub board_id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserPostResult {
    pub post_id: i64,
    pub thread_id: i32,
}

impl<'a> Transaction<Thread> for UserPostThread<'a> {
    fn execute(&self, conn: &mut PgConnection) -> TransactionResult<Thread> {
        if !self.permission.is_not_banned() {
            return Err(TransactionError::UserDisabled);
        }
        conn.build_transaction()
            .read_write()
            .run(|conn| {
                let post_insert = PostInsert {
                    user_id: &self.permission.user_id,
                    content: self.post.content,
                    flag: self.post.flag,
                    thread_id: self.post.thread_id,
                    thread_path: self.post.thread_path,
                    email: self.post.email,
                    title: self.post.title,
                    author: self.post.author,
                };
                let inserted_posts_id: i64 = diesel::insert_into(posts::table)
                    .values(post_insert)
                    .returning(posts::id)
                    .get_result(conn)?;
                let thread_insert = ThreadInsert {
                    primary_post_id: inserted_posts_id,
                    board_id: self.board_id,
                };
                let thread: Thread = diesel::insert_into(threads::table)
                    .values(thread_insert)
                    .get_result::<Thread>(conn)?;
                let _post = diesel::update(posts::table.filter(posts::id.eq(inserted_posts_id)))
                    .set(posts::thread_id.eq(thread.id))
                    .get_result::<Post>(conn)?;
                return Ok(thread)
            })
            .map_err(TransactionError::Diesel)
    }
}
