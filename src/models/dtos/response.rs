use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)] 
pub struct Response {
    pub status: &'static str,
    pub message: String,
}