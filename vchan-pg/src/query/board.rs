use super::*;

pub struct BoardQueryPost {}

impl Query<Vec<Board>> for BoardQueryPost {

    fn query(&self, conn: &mut PgConnection) -> QueryResult<Vec<Board>> {
        boards::table
            .get_results(conn)
            .map_err(QueryError::Diesel)
    }
}