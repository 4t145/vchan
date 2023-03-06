use std::str::FromStr;

use actix_web::cookie::Cookie;

use crate::{consts::COOKIE_TOKEN_KEY};

#[derive(Debug)]
pub struct CookieToken {
    pub id: String,
    pub seed: [u8; 64]
}

impl CookieToken {
    #[allow(dead_code)]
    pub fn generate_cookie(&self) -> Cookie<'_> {
        return Cookie::build(COOKIE_TOKEN_KEY, self.to_string())
            .domain("localhost")
            .permanent()
            .path("/")
            .finish();
    }

}

impl ToString for CookieToken {
    fn to_string(&self) -> String {
        let id = &self.id;
        let seed = self.seed;
        let seedlen = seed.len();
        let seed_hex= hex::encode(&seed);
        format!("[{seedlen}]{seed_hex}@{id}")
    }
}

impl FromStr for CookieToken {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE_COOKIE_TOKEN: regex::Regex = regex::RegexBuilder::new(r"\[\d+\]([[:xdigit:]]*)@(.*)").build().unwrap();
        }
        let caps = RE_COOKIE_TOKEN.captures(s).ok_or(())?;
        let seed = caps.get(1).ok_or(())?.as_str();
        let id = caps.get(2).ok_or(())?.as_str();
        let mut seed_array = [0u8; 64];
        hex::decode_to_slice(seed, &mut seed_array).map_err(|_|())?;
        return Ok(CookieToken {
            id: id.to_string(),
            seed: seed_array,
        })
    }
}