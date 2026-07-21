use serde::Serialize;

use crate::infrastructure::persistence::{DatabaseManager, PersistenceResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseDiagnostics {
    pub reachable: bool,
    pub database_path: String,
    pub schema_version: i64,
    pub pending_migrations: usize,
    pub foreign_keys_enabled: bool,
    pub journal_mode: String,
}

pub struct DatabaseDiagnosticsService;

impl DatabaseDiagnosticsService {
    pub fn execute(database_manager: &DatabaseManager) -> PersistenceResult<DatabaseDiagnostics> {
        let snapshot = database_manager.health_snapshot()?;

        Ok(DatabaseDiagnostics {
            reachable: true,
            database_path: snapshot.database_path.to_string_lossy().into_owned(),
            schema_version: snapshot.schema_version,
            pending_migrations: snapshot.pending_migrations,
            foreign_keys_enabled: snapshot.foreign_keys_enabled,
            journal_mode: snapshot.journal_mode,
        })
    }
}
