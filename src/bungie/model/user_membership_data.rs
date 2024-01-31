use serde::{Deserialize, Serialize};

use crate::bungie::EndPoint;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserMembershipData {
    pub primary_membership_id: String,
    pub bungie_net_user: BungieNetUser,
}

impl EndPoint for UserMembershipData {
    fn get_url() -> String {
        "GetMembershipsForCurrentUser".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungieNetUser {
    pub membership_id: String,
}
