use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserMembershipData {
    pub primary_membership_id: String,
    pub bungie_net_user: BungieNetUser,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungieNetUser {
    pub membership_id: String,
}
