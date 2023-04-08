use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedError {
    name: String,
    message: String,
    code: u16,
    status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotFound {
    name: String,
    pub message: String,
    code: u16,
    status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadRequest {
    name: String,
    message: String,
    code: u16,
    status: u16,
}
