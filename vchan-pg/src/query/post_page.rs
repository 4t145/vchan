use super::*;

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag="type", content="data")]
pub enum PostPageView {
    Thread(i32),
    Poster(String),
    PosterOfThread {
        poster_id: String,
        thread_id: i32
    },
    All,
}

impl Query<PaginationView<Post>> for PaginationQuery<PostPageView> {
    fn query(&self, conn: &mut diesel::PgConnection) -> super::QueryResult<PaginationView<Post>> {
        let (limit, skip) = self.pagination.limit_and_skip();
        let items: Vec<Post> = match &self.view {
            PostPageView::Thread(id) => posts::table
                .filter(posts::thread_id.eq(id))
                .offset(skip)
                .limit(limit)
                .get_results(conn)
                .map_err(QueryError::Diesel)?,
            PostPageView::Poster(id) => posts::table
                .filter(posts::user_id.eq(id))
                .offset(skip)
                .limit(limit)
                .get_results(conn)
                .map_err(QueryError::Diesel)?,
            PostPageView::PosterOfThread { poster_id, thread_id } => posts::table
                .filter(posts::user_id.eq(poster_id).and(posts::thread_id.eq(thread_id)))
                .offset(skip)
                .limit(limit)
                .get_results(conn)
                .map_err(QueryError::Diesel)?,
            PostPageView::All => posts::table
                .offset(skip)
                .limit(limit)
                .get_results(conn)
                .map_err(QueryError::Diesel)?,
            // 未实现部分，可能会拓展
            _ => vec![],
        };
        Ok(PaginationView {
            items,
            pagination: self.pagination,
        })
    }
}
