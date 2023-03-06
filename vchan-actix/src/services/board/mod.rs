use actix_web::web::{self, Data};
use serde::{Deserialize, Serialize};
use vchan_pg::{
    model::{permission_group::PermissonGroup, Board},
    query::{BoardQueryPost, Query},
    transaction::{CreateBoard, Transaction},
};

use super::*;
use crate::init::AppData;

#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBoardResponse {
    board_id: i32,
}

impl VchanRequest for CreateBoardRequest {
    type Response = CreateBoardResponse;

    const PERMISSION_GROUP: PermissonGroup = PMS_GRP::BOARD_MANAGE;

    fn execute(
        req: VchanApiRequest<Self>,
        _permission: UserPermission,
    ) -> Result<Self::Response, VchanError> {
        match req.app_state.pg_pool.get() {
            Ok(mut conn) => {
                let board_id = (CreateBoard {
                    name: req.data.name,
                }
                .execute(&mut conn))
                .map_err(|e| VchanError::TransactionError(e))?;
                Ok(CreateBoardResponse { board_id })
            }
            Err(err) => Err(VchanError::DatabaseUnreachble {
                err: err.to_string(),
            }),
        }
    }
}

#[actix_web::post("/board/create/{name}")]
pub async fn create(
    data: Data<AppData>,
    name: web::Path<String>,
) -> VchanApiRequest<CreateBoardRequest> {
    VchanApiRequest {
        data: CreateBoardRequest {
            name: name.to_string(),
        },
        app_state: data,
    }
}

#[derive(Debug, Deserialize)]
pub struct GetBoardRequest {}

#[derive(Debug, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct GetBoardResponse(Vec<Board>);

impl VchanRequest for GetBoardRequest {
    type Response = GetBoardResponse;

    const PERMISSION_GROUP: u64 = PMS_GRP::ANY;

    fn execute(
        req: VchanApiRequest<Self>,
        _permission: UserPermission,
    ) -> Result<Self::Response, VchanError>
    where
        Self: std::marker::Sized,
    {
        let mut conn =
            req.app_state
                .pg_pool
                .get()
                .map_err(|err| VchanError::DatabaseUnreachble {
                    err: err.to_string(),
                })?;
        BoardQueryPost {}
            .query(&mut conn)
            .map(GetBoardResponse)
            .map_err(VchanError::QueryError)
    }
}

#[actix_web::get("/board")]
pub async fn get(data: Data<AppData>) -> VchanApiRequest<GetBoardRequest> {
    VchanApiRequest {
        data: GetBoardRequest {},
        app_state: data,
    }
}
