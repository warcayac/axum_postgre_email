use serde::{Deserialize, Serialize};

use super::user_data::UserData;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}
