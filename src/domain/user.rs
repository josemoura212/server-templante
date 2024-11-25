use uuid::Uuid;

use super::{password::PasswordUser, EmailUser};

#[derive(Debug, Clone)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub email: EmailUser,
    pub password: PasswordUser,
    pub crm: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub admin: bool,
    pub crm: Option<String>,
    pub photo: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
