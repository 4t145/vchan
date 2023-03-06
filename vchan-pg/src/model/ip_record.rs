use serde::{Serialize, Deserialize};

use super::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = ip_records)]
pub struct IpRecordInsert {
    pub ip: ipnetwork::IpNetwork
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct IpRecord {
    pub ip: ipnetwork::IpNetwork,
    pub banned: bool,
    pub fetch_cookie_date: chrono::NaiveDate
}