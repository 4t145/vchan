use std::num::NonZeroUsize;

use actix_web::web;
use vchan_pg::{
    model::Thread,
    query::{Pagination, PaginationQuery, PaginationView, Query, ThreadPageView},
    transaction::*,
};

use super::post::PostPostRequest;

pub use super::*;

#[derive(Debug, Deserialize)]
pub struct PostThreadRequest {
    pub post: PostPostRequest,
    pub board_id: i32,
}

#[derive(Debug, Serialize)]
pub struct PostThreadResonse {
    primary_post_id: i64,
    thread_id: i32,
}

impl VchanRequest for PostThreadRequest {
    type Response = PostThreadResonse;

    const PERMISSION_GROUP: u64 = PMS_GRP::POST;

    fn execute(
        req: VchanApiRequest<Self>,
        permission: UserPermission,
    ) -> Result<Self::Response, VchanError>
    where
        Self: std::marker::Sized,
    {
        let post_insert = req.data.post.into_body();
        match req.app_state.pg_pool.get() {
            Ok(mut conn) => {
                let thread = UserPostThread {
                    post: post_insert,
                    permission,
                    board_id: req.data.board_id,
                }
                .execute(&mut conn)
                .map_err(VchanError::TransactionError)?;
                Ok(PostThreadResonse {
                    primary_post_id: thread.primary_post_id,
                    thread_id: thread.id,
                })
            }
            Err(err) => Err(VchanError::DatabaseUnreachble {
                err: err.to_string(),
            }),
        }
    }
}

#[actix_web::post("/thread")]
pub async fn post(
    data: Data<AppData>,
    req: Json<PostThreadRequest>,
) -> VchanApiRequest<PostThreadRequest> {
    VchanApiRequest::new(data, req.0)
}

#[derive(Debug, Deserialize)]
pub struct ThreadGetRequest(PaginationQuery<ThreadPageView>);

#[derive(Debug, Deserialize)]
pub struct ThreadGetParam {
    pub pagesize: String,
    pub page: String,
    pub board_id: String,
}

#[derive(Debug, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ThreadGetResponse(PaginationView<Thread>);

impl VchanRequest for ThreadGetRequest {
    type Response = ThreadGetResponse;

    const PERMISSION_GROUP: u64 = PMS_GRP::ANY;

    fn execute(
        req: VchanApiRequest<Self>,
        _permission: UserPermission,
    ) -> Result<Self::Response, VchanError>
    where
        Self: std::marker::Sized,
    {
        match req.app_state.pg_pool.get() {
            Ok(mut conn) => {
                let result = req
                    .data
                    .0
                    .query(&mut conn)
                    .map_err(VchanError::QueryError)?;
                Ok(ThreadGetResponse(result))
            }
            Err(err) => Err(VchanError::DatabaseUnreachble {
                err: err.to_string(),
            }),
        }
    }
}

#[actix_web::get("/thread")]
pub async fn get(
    data: Data<AppData>,
    query: web::Query<ThreadGetParam>,
) -> VchanApiRequest<ThreadGetRequest> {
    let one = unsafe { NonZeroUsize::new_unchecked(1) };
    let page = query.0.page.parse::<NonZeroUsize>().unwrap_or(one);
    let pagesize = query.0.pagesize.parse::<NonZeroUsize>().unwrap_or(one);
    let board_id = query.0.board_id.parse::<i32>().unwrap();
    let req = ThreadGetRequest(PaginationQuery {
        view: ThreadPageView { board_id },
        pagination: Pagination { page, pagesize },
    });
    VchanApiRequest::new(data, req)
}
