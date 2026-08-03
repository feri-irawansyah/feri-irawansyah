use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserView {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub client_category: i32,
}
