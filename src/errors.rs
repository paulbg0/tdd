pub struct UnauthorizedError {
    name: String,
    message: String,
    code: u16,
    status: u16,
}

pub struct NotFound {
    name: String,
    message: String,
    code: u16,
    status: u16,
}

pub struct BadRequest {
    name: String,
    message: String,
    code: u16,
    status: u16,
}
