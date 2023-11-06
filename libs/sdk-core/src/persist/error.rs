pub type PersistResult<T, E = PersistError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum PersistError {
    #[error("Generic: {0}")]
    Generic(#[from] anyhow::Error),

    #[error("Migration: {0}")]
    Migration(#[from] rusqlite_migration::Error),

    #[error("SQL: {0}")]
    Sql(#[from] rusqlite::Error),
}

impl From<hex::FromHexError> for PersistError {
    fn from(err: hex::FromHexError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<serde_json::Error> for PersistError {
    fn from(err: serde_json::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}
