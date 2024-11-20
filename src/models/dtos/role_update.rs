use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::entities::user_role::UserRole;


#[derive(Debug, Serialize, Deserialize, Validate)] 
pub struct RoleUpdateDto {
    #[validate(custom(function= "validate_user_role"))]
    pub role: UserRole,
}


fn validate_user_role(role: &UserRole) -> Result<(), ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        // _ => Err(ValidationError::new("invalid_role")),
    }
}