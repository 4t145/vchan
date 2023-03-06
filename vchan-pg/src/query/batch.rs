use super::*;

pub struct BatchQueryPost {}
impl BatchQuery for BatchQueryPost {
    type Key = i64;

    type Value = Post;

    fn query(keys: &[Self::Key], conn: &mut PgConnection) -> QueryResult<Vec<Self::Value>> {
        posts::table
            .filter(posts::id.eq_any(keys))
            .get_results(conn)
            .map_err(QueryError::Diesel)
    }
}
