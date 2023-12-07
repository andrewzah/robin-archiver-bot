use thiserror::Error;

#[derive(Error, Debug)]
pub enum RobinError {
    #[error("General Error: {0}")]
    Generic(String)
}

impl RobinError {
    pub fn from_str(msg: &str) -> Self {
        RobinError::Generic(msg.into())
    }
}
