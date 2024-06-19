use thiserror::Error;

#[derive(Debug, Error)]
#[error("{err}")]
pub struct ServiceConnectivityError {
    pub err: String,
}
impl ServiceConnectivityError {
    pub fn new(err: &str) -> Self {
        ServiceConnectivityError {
            err: err.to_string(),
        }
    }
}
