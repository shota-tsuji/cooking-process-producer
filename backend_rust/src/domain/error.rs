use std::error::Error;

pub type AsyncDynError = dyn Error + Send + Sync;

#[derive(Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub error: Option<Box<AsyncDynError>>,
}
