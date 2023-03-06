use super::*;
// use diesel::dsl::not;

#[derive(Debug, Clone, Deserialize)]
#[serde_as]
pub struct ThreadPageView {
    pub board_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "view", content = "data")]
pub enum FilterMode {
    WhiteList,
    BlackList,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThreadPageFilter {
    pub list: Vec<Board>,
    pub filter_mode: FilterMode,
}

impl Query<PaginationView<Thread>> for PaginationQuery<ThreadPageView> {
    fn query(&self, conn: &mut diesel::PgConnection) -> super::QueryResult<PaginationView<Thread>> {
        let (limit, skip) = self.pagination.limit_and_skip();
        let items = threads::table
            .filter(threads::board_id.eq(self.view.board_id))
            .offset(skip)
            .limit(limit)
            .get_results(conn)
            .map_err(QueryError::Diesel)?;
        Ok(PaginationView {
            items,
            pagination: self.pagination,
        })
    }
}
