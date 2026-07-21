use std::{path::PathBuf, sync::Mutex, time::Duration};

use rusqlite::{Connection, OpenFlags};

use super::{
    migration::{current_schema_version, pending_migration_count, run_pending_migrations},
    PersistenceError, PersistenceResult,
};

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct DatabaseHealthSnapshot {
    pub database_path: PathBuf,
    pub schema_version: i64,
    pub pending_migrations: usize,
    pub foreign_keys_enabled: bool,
    pub journal_mode: String,
}

pub struct DatabaseManager {
    database_path: PathBuf,
    connection: Mutex<Connection>,
}

impl DatabaseManager {
    pub fn open(database_path: PathBuf, application_version: &str) -> PersistenceResult<Self> {
        let mut connection = Connection::open_with_flags(
            &database_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        configure_connection(&connection)?;
        run_pending_migrations(&mut connection, application_version)?;
        verify_integrity(&connection)?;

        Ok(Self {
            database_path,
            connection: Mutex::new(connection),
        })
    }

    #[cfg(test)]
    pub(crate) fn open_in_memory(application_version: &str) -> PersistenceResult<Self> {
        let mut connection = Connection::open_in_memory()?;

        configure_connection(&connection)?;
        run_pending_migrations(&mut connection, application_version)?;
        verify_integrity(&connection)?;

        Ok(Self {
            database_path: PathBuf::from(":memory:"),
            connection: Mutex::new(connection),
        })
    }

    pub(crate) fn with_connection<T>(
        &self,
        operation: impl FnOnce(&Connection) -> PersistenceResult<T>,
    ) -> PersistenceResult<T> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| PersistenceError::LockPoisoned)?;

        operation(&connection)
    }

    pub fn health_snapshot(&self) -> PersistenceResult<DatabaseHealthSnapshot> {
        self.with_connection(|connection| {
            let foreign_keys_enabled = connection.query_row("PRAGMA foreign_keys", [], |row| {
                let value: i64 = row.get(0)?;
                Ok(value == 1)
            })?;

            let journal_mode = connection.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;

            Ok(DatabaseHealthSnapshot {
                database_path: self.database_path.clone(),
                schema_version: current_schema_version(connection)?,
                pending_migrations: pending_migration_count(connection)?,
                foreign_keys_enabled,
                journal_mode,
            })
        })
    }
}

fn configure_connection(connection: &Connection) -> PersistenceResult<()> {
    connection.busy_timeout(BUSY_TIMEOUT)?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    connection.pragma_update(None, "synchronous", "NORMAL")?;
    connection.pragma_update(None, "recursive_triggers", "OFF")?;

    Ok(())
}

fn verify_integrity(connection: &Connection) -> PersistenceResult<()> {
    connection.query_row("PRAGMA quick_check", [], |row| {
        let result: String = row.get(0)?;

        if result == "ok" {
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    })?;

    let foreign_key_violation_count: i64 =
        connection.query_row("SELECT COUNT(*) FROM pragma_foreign_key_check", [], |row| {
            row.get(0)
        })?;

    if foreign_key_violation_count > 0 {
        return Err(PersistenceError::Database(rusqlite::Error::InvalidQuery));
    }

    Ok(())
}
