use actix_web::HttpRequest;
use rand::Rng;
use vchan_pg::model::UserCookie;

use crate::{consts::COOKIE_TOKEN_KEY, models::CookieToken};

#[inline]
pub fn get_random_uname() -> String {
    let mut out = hex::encode(rand::thread_rng().gen::<[u8; 5]>());
    out.truncate(9);
    out
}

pub fn extract_cookie(req: &HttpRequest) -> Option<UserCookie> {
    if let Some(ct) = req
        .cookie(COOKIE_TOKEN_KEY).and_then(|t| t.value().parse::<CookieToken>().ok()) {
            Some(UserCookie {
                name: ct.id,
                cookieseed: ct.seed
            })
        }
    else {
        None
    }
}