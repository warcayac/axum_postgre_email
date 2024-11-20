use serde::{Deserialize, Serialize};

use super::filter_user::FilterUserDto;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}