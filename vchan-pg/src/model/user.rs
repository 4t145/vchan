use super::*;
/// flag
/// _
/// 0 是否是内容管理员：权限组删po
/// 1 是否是用户管理员：权限组封禁
/// 2 是否是板块管理员：权限组板块
/// ... 

#[derive(Debug, Builder, Default, Insertable)]
#[diesel(table_name = users)]
#[builder(setter(into))]
pub struct UserInsert<'a> {
    // id: Option<i32>,
    pub id: &'a str,
    pub flag: i64,
    pub cookiehash: &'a [i32],
    pub create_time: NaiveDateTime,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub flag: i64,
    pub cookieseed: [u8; 64],
}


#[derive(Debug)]
pub struct UserCookie {
    pub name: String,
    pub cookieseed: [u8; 64],
}

