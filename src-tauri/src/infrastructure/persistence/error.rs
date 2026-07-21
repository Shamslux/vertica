use std::path::PathBuf;

use thiserror::Error;

pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("failed to resolve the application data directory: {0}")]
    PathResolution(#[from] tauri::Error),

    #[error("failed to create the database directory at {path}: {source}")]
    DirectoryCreation {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("database operation failed: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("database state lock is unavailable")]
    LockPoisoned,

    #[error("migration {version} has a different checksum than the applied migration")]
    MigrationChecksumMismatch { version: i64 },

    #[error("migration {version} failed: {source}")]
    MigrationFailed {
        version: i64,
        #[source]
        source: rusqlite::Error,
    },
}
