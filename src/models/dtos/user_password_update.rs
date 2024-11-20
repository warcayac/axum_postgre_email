use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)] 
pub struct UserPasswordUpdateDto {
    #[validate(length(min = 6, message = "new password must be at least 6 characters"))]
    pub new_password: String,

    #[validate(
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(length(min = 6, message = "Old password must be at least 6 characters"))]
    pub old_password: String,
}