use std::time::SystemTimeError;

pub type PersistResult<T, E = PersistError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum PersistError {
    #[error("{0}")]
    Generic(String),

    #[error("{0}")]
    Migration(String),

    #[error("Sql persistence: {0}")]
    Sql(String),
}

impl PersistError {
    pub(crate) fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<anyhow::Error> for PersistError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<hex::FromHexError> for PersistError {
    fn from(err: hex::FromHexError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<rusqlite::Error> for PersistError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sql(err.to_string())
    }
}

impl From<rusqlite_migration::Error> for PersistError {
    fn from(err: rusqlite_migration::Error) -> Self {
        Self::Migration(err.to_string())
    }
}

impl From<serde_json::Error> for PersistError {
    fn from(err: serde_json::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<SystemTimeError> for PersistError {
    fn from(_value: SystemTimeError) -> Self {
        Self::generic("invalid system time")
    }
}
