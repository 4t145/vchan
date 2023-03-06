use vchan_pg::{
    model::{Post, PostInsert, PostBody},
    query::*,
    transaction::*,
};

pub use super::*;

#[derive(Debug, Deserialize)]
pub struct PostPostRequest {
    pub thread_id: i32,
    pub thread_path: Vec<i64>,
    pub content: String,
    pub flag: i64,
    pub email: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

impl PostPostRequest {
    pub fn into_body<'a>(&'a self) -> PostBody<'a> {
        PostBody {
            content: &self.content,
            flag: self.flag,
            thread_id: self.thread_id,
            thread_path: &self.thread_path,
            email: self.email.as_ref().map(|x| x.as_str()),
            title: self.title.as_ref().map(|x| x.as_str()),
            author: self.author.as_ref().map(|x| x.as_str()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PostPostResonse {
    post_id: i64,
}

impl VchanRequest for PostPostRequest {
    type Response = PostPostResonse;

    const PERMISSION_GROUP: u64 = PMS_GRP::POST;

    fn execute(
        req: VchanApiRequest<Self>,
        permission: UserPermission,
    ) -> Result<Self::Response, VchanError>
    where
        Self: std::marker::Sized,
    {
        let post_insert = req.data.into_body();
        match req.app_state.pg_pool.get() {
            Ok(mut conn) => {
                let post_id = UserReplyThread {
                    post: post_insert,
                    permission,
                }
                .execute(&mut conn)
                .map_err(VchanError::TransactionError)?;
                Ok(PostPostResonse { post_id })
            }
            Err(err) => Err(VchanError::DatabaseUnreachble {
                err: err.to_string(),
            }),
        }
    }
}

#[actix_web::post("/post")]
pub async fn post(
    data: Data<AppData>,
    req: Json<PostPostRequest>,
) -> VchanApiRequest<PostPostRequest> {
    VchanApiRequest::new(data, req.0)
}

#[derive(Debug, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct PostGetRequest(PaginationQuery<PostPageView>);

#[derive(Debug, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct PostGetResponse(PaginationView<Post>);

impl VchanRequest for PostGetRequest {
    type Response = PostGetResponse;

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
                Ok(PostGetResponse(result))
            }
            Err(err) => Err(VchanError::DatabaseUnreachble {
                err: err.to_string(),
            }),
        }
    }
}

impl PostGetRequest {}

#[actix_web::get("/post")]
pub async fn get(
    data: Data<AppData>,
    req: Json<PostGetRequest>,
) -> VchanApiRequest<PostGetRequest> {
    VchanApiRequest::new(data, req.0)
}

#[derive(Debug, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct PostGetBatchRequest(Vec<i64>);

#[derive(Debug, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct PostGetBatchResponse(Vec<Post>);

impl VchanRequest for PostGetBatchRequest {
    type Response = PostGetBatchResponse;

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
                use vchan_pg::query::*;
                let result = Batch::<BatchQueryPost>(req.data.0)
                    .query(&mut conn)
                    .map_err(VchanError::QueryError)?;
                Ok(PostGetBatchResponse(result))
            }
            Err(err) => Err(VchanError::DatabaseUnreachble {
                err: err.to_string(),
            }),
        }
    }
}

impl PostGetRequest {}

#[actix_web::post("/post/batch")]
pub async fn batch(
    data: Data<AppData>,
    req: Json<PostGetBatchRequest>,
) -> VchanApiRequest<PostGetBatchRequest> {
    VchanApiRequest::new(data, req.0)
}
