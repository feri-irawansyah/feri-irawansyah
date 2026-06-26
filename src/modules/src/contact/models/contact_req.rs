use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactReq {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}
