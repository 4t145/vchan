use crate::model::*;
use crate::schema::*;
use chrono::Utc;
use diesel::prelude::*;

use super::get_user_auth::UserPermission;
use super::Transaction;
use super::TransactionError;
use super::TransactionResult;

pub struct UserReplyThread<'a> {
    pub permission: UserPermission,
    pub post: PostBody<'a>,
}

impl<'a> Transaction<i64> for UserReplyThread<'a> {
    fn execute(&self, conn: &mut PgConnection) -> TransactionResult<i64> {
        if !self.permission.is_not_banned() {
            return Err(TransactionError::UserDisabled);
        }
        conn.build_transaction()
            .read_write()
            .run(|conn| {
                let thread_id = self.post.thread_id;
                let mut thread_path = Vec::new();
                if let Some(reply_to) = self.post.thread_path.last() {
                    let parent_path = posts::table
                        .filter(
                            posts::id
                                .eq(reply_to)
                                .and(posts::thread_id.eq(self.post.thread_id)),
                        )
                        .select(posts::thread_path)
                        .get_result::<Vec<Option<i64>>>(conn)?;
                    thread_path = parent_path.iter().map(|x| x.unwrap_or_default()).collect();
                    thread_path.push(*reply_to)
                }
                let post_insert = PostInsert {
                    user_id: &self.permission.user_id,
                    content: self.post.content,
                    flag: self.post.flag,
                    thread_id,
                    thread_path: &thread_path,
                    email: self.post.email,
                    title: self.post.title,
                    author: self.post.author,
                };
                diesel::insert_into(posts::table)
                    .values(post_insert)
                    .execute(conn)?;
                let inserted_posts_id = posts::table.select(posts::id).get_result::<i64>(conn)?;
                let current_time = Utc::now().naive_utc();
                diesel::update(threads::table)
                    .filter(threads::id.eq(thread_id))
                    .set(threads::update_time.eq(current_time))
                    .execute(conn)?;
                return Ok(inserted_posts_id);
            })
            .map_err(TransactionError::Diesel)
    }
}
