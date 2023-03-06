use crate::{init::AppData, utils::extract_cookie};
use actix_web::{
    body::BoxBody,
    http::{header, StatusCode},
    web::{Data, Json},
    HttpResponse, HttpResponseBuilder, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vchan_pg::{
    query::QueryError,
    transaction::{GetUserPermission, Transaction, TransactionError, UserPermission},
};

use crate::consts::*;
use vchan_pg::model::permission_group as PMS_GRP;
pub mod board;
pub mod post;
pub mod thread;
pub mod user;

#[derive(Debug, Serialize)]
#[serde(tag="type", content="data")]
pub enum VchanError {
    PermissionDenied { group: Option<u64> },
    DatabaseUnreachble { err: String },
    TransactionError(TransactionError),
    QueryError(QueryError),
}

pub trait VchanRequest {
    type Response: Serialize;
    const PERMISSION_GROUP: u64;
    fn check_permission(&self, permission: &UserPermission) -> bool {
        if Self::PERMISSION_GROUP == PMS_GRP::ANY {
            true
        } else {
            permission.is_valid() && permission.has_permission(Self::PERMISSION_GROUP)
        }
    }
    fn execute(
        req: VchanApiRequest<Self>,
        permission: UserPermission,
    ) -> Result<Self::Response, VchanError>
    where
        Self: std::marker::Sized;
}

pub struct VchanApiRequest<T: VchanRequest> {
    data: T,
    app_state: Data<AppData>,
}

impl<T: VchanRequest> VchanApiRequest<T> {
    #[inline]
    pub fn execute(self, permission: UserPermission) -> Result<T::Response, VchanError> {
        if self.data.check_permission(&permission) {
            T::execute(self, permission)
        } else {
            Err(VchanError::PermissionDenied {
                group: Some(T::PERMISSION_GROUP),
            })
        }
    }

    pub fn new(app_state: Data<AppData>, data: T) -> Self {
        Self { data, app_state }
    }
}

impl<T: VchanRequest> Responder for VchanApiRequest<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let mut builder = HttpResponse::InternalServerError();
        let permission: Result<UserPermission, VchanError> = {
            if T::PERMISSION_GROUP == PMS_GRP::ANY {
                Ok(UserPermission {
                    user_id: String::new(),
                    permission_group: PMS_GRP::ANY,
                    banned: true,
                    valid: false,
                })
            } else {
                self.app_state
                    .pg_pool
                    .get()
                    .map_err(|e| VchanError::DatabaseUnreachble { err: e.to_string() })
                    .and_then(|mut conn| {
                        extract_cookie(&req)
                            .ok_or(VchanError::PermissionDenied { group: None })
                            .and_then(|cookie| {
                                GetUserPermission { cookie: &cookie }
                                    .execute(&mut conn)
                                    .map_err(|_|VchanError::PermissionDenied { group: None })
                            })
                    })
            }
        };
        let vchan_resp = VchanResponse(permission.and_then(|permission| self.execute(permission)));
        vchan_resp.wrap(&mut builder);
        return builder.json(Into::<Value>::into(vchan_resp));
    }
}

#[repr(transparent)]
pub struct VchanResponse<T: Serialize = ()>(Result<T, VchanError>);

impl<T: Serialize> VchanResponse<T> {
    pub fn wrap(&self, response_builder: &mut HttpResponseBuilder) {
        match &self.0 {
            Ok(_) => {
                response_builder.status(StatusCode::OK);
            }
            Err(err) => {
                use VchanError::*;
                match err {
                    PermissionDenied { group } => {
                        response_builder.status(StatusCode::UNAUTHORIZED);
                        response_builder.append_header((
                            header::WWW_AUTHENTICATE,
                            group
                                .as_ref()
                                .map(|_| "permission is not enough")
                                .unwrap_or("cookie not valid"),
                        ));
                    }
                    _ => {
                        response_builder.status(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            }
        };
    }
    #[inline]
    pub fn ok(value: T) -> Self {
        Self(Ok(value))
    }
    /*     
    #[inline]
    pub fn err(err: VchanError) -> Self {
        Self(Err(err))
    }
    */
    #[inline]
    pub fn json(self) -> Value {
        self.into()
    }
}

impl<T> Into<serde_json::Value> for VchanResponse<T>
where
    T: Serialize,
{
    fn into(self) -> serde_json::Value {
        match self.0 {
            Ok(data) => {
                serde_json::json! ({
                    "ok": true,
                    "data": data
                })
            }
            Err(err) => {
                serde_json::json! ({
                    "ok": false,
                    "err": err
                })
            }
        }
    }
}

use actix_web::cookie::Cookie;
use std::time::Duration;

#[derive(Debug, Serialize)]
pub struct CreateUserResponse<'a> {
    cookie_token: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {}

#[actix_web::post("/register")]
pub async fn register(data: Data<AppData>) -> HttpResponse {
    use crate::{models::CookieToken, utils::get_random_uname};

    let name = get_random_uname();
    let create_user = vchan_pg::transaction::CreateUser { name, flag: PMS_GRP::POST as i64 };

    let mut conn = data.pg_pool.get_timeout(Duration::from_secs(30)).unwrap();
    let seed = create_user.execute(&mut conn).unwrap();
    let cookie_token = CookieToken {
        id: create_user.name,
        seed,
    };
    let cookie_token_string = cookie_token.to_string();
    let resp = CreateUserResponse {
        cookie_token: &cookie_token_string,
    };
    return HttpResponse::Ok()
        .cookie(
            Cookie::build(COOKIE_TOKEN_KEY, &cookie_token_string)
                .domain(env!("DOMAIN"))
                .expires(None)
                .path("/")
                .finish(),
        )
        .json(VchanResponse::ok(resp).json());
}
