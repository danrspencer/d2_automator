use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response<T> {
    pub response: T,
    pub throttle_seconds: i32,
    pub error_status: String,
    pub message: String,
    // pub message_data: MessageData,
}
