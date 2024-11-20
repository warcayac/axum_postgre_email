use super::user_role::UserRole;

type TDateTime = Option<chrono::DateTime<chrono::Utc>>;

#[derive(Debug, sqlx::FromRow, sqlx::Type, serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub verified: bool,
    pub verification_token: Option<String>,
    pub token_expires_at: TDateTime,
    #[serde(rename = "createdAt")]
    pub created_at: TDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: TDateTime,
}