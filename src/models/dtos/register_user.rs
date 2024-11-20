use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Deserialize, Serialize, Validate, Default)] 
pub struct RegisterUserDto {
    #[validate(length(min= 1, message= "Name is required"))]
    pub name: String,
    #[validate(
        length(min= 1, message= "Email is required"),
        email(message= "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min= 6, message= "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(
        length(min= 1, message= "Confirmation password is required"),
        must_match(other="password", message= "Passwords do not match")
    )]
    #[serde(rename= "passwordConfirm")]
    pub password_confirm: String,
}