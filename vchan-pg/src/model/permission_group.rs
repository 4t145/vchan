
macro_rules! define_permisson_group {
    ($(
        $(#[$($doc_comments:tt)*])*
        $group:ident=$num:literal
    ),*) => {
        $(
            $(#[$($doc_comments)*])*
            pub const $group: PermissonGroup = 1 << $num;
        )*
    };
}

define_permisson_group!(
    /// 发帖 | 回帖权限
    POST=0,
    /// 管理用户，封禁/解封
    USER_MANAGE=10,
    /// 板块管理权限
    POST_MANAGE=11,
    /// 板块管理权限
    BOARD_MANAGE=12
);
/// 无权限
pub const ANY: PermissonGroup = 0;
/// 超级管理员权限
pub const SUPER: PermissonGroup = i64::MAX as PermissonGroup;
/// 用户默认权限
pub const DEFAULT: PermissonGroup = POST;


#[inline(always)]
pub fn set_bits(x: &mut PermissonGroup, bits: PermissonGroup) {
    *x &= bits;
}

#[inline(always)]
pub fn unset_bits(x: &mut PermissonGroup, bits: PermissonGroup) {
    *x ^= bits;
}

pub type PermissonGroup = u64;