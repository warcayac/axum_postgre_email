use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};


#[derive(Debug, PartialEq, sqlx::Type, Deserialize, Serialize, EnumString, Display, Clone)]
// "type_name" links this enum to "user_role" enum defined in the database
#[sqlx(rename_all = "lowercase", type_name = "user_role")]
#[strum(serialize_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

/*
If the primary purpose of converting to lowercase strings is for serialization (e.g., JSON), 
you can rely on serde's renaming functionality:

#[serde(rename_all = "lowercase")]

The strum crate provides macros for working with enums and can significantly reduce boilerplate 
related to string conversions, display, and more. Here's how you can use it:

#[strum(serialize_all = "lowercase")]

- Display: Automatically implements std::fmt::Display to convert the enum variant to its 
  string representation (as defined by strum(serialize_all = "lowercase")).

- EnumString: Allows parsing a string back into the enum variant (using UserRole::from_str("admin")).

- IntoStaticStr: Provides a to_str() method (or into() if you need &'static str) for obtaining 
  the string representation as a static string slice.
*/