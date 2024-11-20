use serde::{Deserialize, Serialize};

use super::filter_user::FilterUserDto;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}
