#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ErrorMessage {
    // #[error("Server Error. Please try again later")]
    // ServerError,

    #[error("Email or password is wrong")]
    WrongCredentials,

    #[error("A user with this email already exists")]
    EmailExist,

    #[error("User belonging to this token no longer exists")]
    UserNoLongerExist,

    #[error("Password cannot be empty")]
    EmptyPassword,

    #[error("Error while hashing password")]
    HashingError,

    #[error("Invalid password hash format")]
    InvalidHashFormat,

    #[error("Password must not be more than {0} characters")]
    ExceededMaxPasswordLength(usize),

    #[error("Authentication token is invalid or expired")]
    InvalidToken,

    #[error("You are not logged in, please provide a token")]
    TokenNotProvided,

    #[error("You are not allowed to perform this action")]
    PermissionDenied,

    #[error("Authentication required. Please log in.")]
    UserNotAuthenticated,
}
