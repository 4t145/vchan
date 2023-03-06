use actix_web::web::Data;
use vchan_pg::transaction::UserPermission;

use super::*;

pub struct CheckAuthRequest {}

impl VchanRequest for CheckAuthRequest {
    type Response = UserPermission;

    const PERMISSION_GROUP: u64 = 0;

    fn execute(
        _req: VchanApiRequest<Self>,
        permission: UserPermission,
    ) -> Result<Self::Response, VchanError> {
        return Ok(permission);
    }
}

#[actix_web::get("/user/check-auth")]
pub async fn check_auth(data: Data<AppData>) -> VchanApiRequest<CheckAuthRequest> {
    VchanApiRequest {
        data: CheckAuthRequest {},
        app_state: data,
    }
}

// todo
// - jail
//
