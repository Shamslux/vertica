use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};

use super::{PersistenceError, PersistenceResult};

pub struct Migration {
    pub version: i64,
    pub description: &'static str,
    pub sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "initialize persistence foundation",
        sql: include_str!("migrations/0001_initialize_persistence_foundation.sql"),
    },
    Migration {
        version: 2,
        description: "create application settings",
        sql: include_str!("migrations/0002_create_application_settings.sql"),
    },
    Migration {
        version: 3,
        description: "create projects",
        sql: include_str!("migrations/0003_create_projects.sql"),
    },
];

pub fn run_pending_migrations(
    connection: &mut Connection,
    application_version: &str,
) -> PersistenceResult<()> {
    ensure_history_table(connection)?;
    verify_applied_migrations(connection)?;

    for migration in MIGRATIONS {
        if is_applied(connection, migration.version)? {
            continue;
        }

        apply_migration(connection, migration, application_version)?;
    }

    Ok(())
}

pub fn current_schema_version(connection: &Connection) -> PersistenceResult<i64> {
    ensure_history_table(connection)?;

    connection
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .map_err(Into::into)
}

pub fn pending_migration_count(connection: &Connection) -> PersistenceResult<usize> {
    ensure_history_table(connection)?;

    let mut count = 0;

    for migration in MIGRATIONS {
        if !is_applied(connection, migration.version)? {
            count += 1;
        }
    }

    Ok(count)
}

fn ensure_history_table(connection: &Connection) -> PersistenceResult<()> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            checksum TEXT NOT NULL,
            applied_at TEXT NOT NULL,
            application_version TEXT NOT NULL,
            execution_time_ms INTEGER NOT NULL CHECK (execution_time_ms >= 0)
        ) STRICT;",
    )?;

    Ok(())
}

fn verify_applied_migrations(connection: &Connection) -> PersistenceResult<()> {
    for migration in MIGRATIONS {
        let stored_checksum: Option<String> = connection
            .query_row(
                "SELECT checksum
                 FROM schema_migrations
                 WHERE version = ?1",
                [migration.version],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(stored_checksum) = stored_checksum {
            let expected_checksum = checksum(migration.sql);

            if stored_checksum != expected_checksum {
                return Err(PersistenceError::MigrationChecksumMismatch {
                    version: migration.version,
                });
            }
        }
    }

    Ok(())
}

fn is_applied(connection: &Connection, version: i64) -> PersistenceResult<bool> {
    let exists: i64 = connection.query_row(
        "SELECT EXISTS(
            SELECT 1
            FROM schema_migrations
            WHERE version = ?1
        )",
        [version],
        |row| row.get(0),
    )?;

    Ok(exists == 1)
}

fn apply_migration(
    connection: &mut Connection,
    migration: &Migration,
    application_version: &str,
) -> PersistenceResult<()> {
    let started_at = std::time::Instant::now();

    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

    if let Err(source) = transaction.execute_batch(migration.sql) {
        return Err(PersistenceError::MigrationFailed {
            version: migration.version,
            source,
        });
    }

    let execution_time_ms = i64::try_from(started_at.elapsed().as_millis()).unwrap_or(i64::MAX);

    transaction.execute(
        "INSERT INTO schema_migrations (
            version,
            description,
            checksum,
            applied_at,
            application_version,
            execution_time_ms
        ) VALUES (
            ?1,
            ?2,
            ?3,
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
            ?4,
            ?5
        )",
        params![
            migration.version,
            migration.description,
            checksum(migration.sql),
            application_version,
            execution_time_ms,
        ],
    )?;

    transaction.commit()?;

    Ok(())
}

fn checksum(content: &str) -> String {
    let digest = Sha256::digest(content.as_bytes());

    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use rusqlite::{params, Connection};

    use super::{current_schema_version, pending_migration_count, run_pending_migrations};

    const PROJECT_ID: &str = "4e31e697-bdd1-4de5-b31a-341f36e456f4";
    const CREATED_AT: &str = "2026-07-19T12:00:00.000Z";

    struct TestProject<'a> {
        id: &'a str,
        name: &'a str,
        status: &'a str,
        priority: i64,
        start_date: Option<&'a str>,
        target_date: Option<&'a str>,
        archived_at: Option<&'a str>,
    }

    impl<'a> TestProject<'a> {
        fn active(id: &'a str, name: &'a str) -> Self {
            Self {
                id,
                name,
                status: "active",
                priority: 3,
                start_date: None,
                target_date: None,
                archived_at: None,
            }
        }
    }

    fn migrated_connection() -> Connection {
        let mut connection = Connection::open_in_memory().expect("in-memory database should open");

        run_pending_migrations(&mut connection, "test").expect("all migrations should be applied");

        connection
    }

    fn insert_project(
        connection: &Connection,
        project: TestProject<'_>,
    ) -> rusqlite::Result<usize> {
        connection.execute(
            "INSERT INTO projects (
                id,
                name,
                description,
                project_type,
                status,
                start_date,
                target_date,
                priority,
                color,
                icon,
                objective,
                settings_json,
                created_at,
                updated_at,
                archived_at,
                deleted_at
            ) VALUES (
                ?1,
                ?2,
                NULL,
                'software',
                ?3,
                ?4,
                ?5,
                ?6,
                NULL,
                NULL,
                NULL,
                '{}',
                ?7,
                ?7,
                ?8,
                NULL
            )",
            params![
                project.id,
                project.name,
                project.status,
                project.start_date,
                project.target_date,
                project.priority,
                CREATED_AT,
                project.archived_at,
            ],
        )
    }

    #[test]
    fn migrations_are_idempotent() {
        let mut connection = Connection::open_in_memory().expect("in-memory database should open");

        run_pending_migrations(&mut connection, "test")
            .expect("first migration run should succeed");

        run_pending_migrations(&mut connection, "test")
            .expect("second migration run should succeed");

        assert_eq!(current_schema_version(&connection).unwrap(), 3);
        assert_eq!(pending_migration_count(&connection).unwrap(), 0);

        let applied_migration_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .expect("migration history should be readable");

        assert_eq!(applied_migration_count, 3);
    }

    #[test]
    fn projects_migration_creates_the_projects_table() {
        let connection = migrated_connection();

        let table_exists: i64 = connection
            .query_row(
                "SELECT EXISTS(
                    SELECT 1
                    FROM sqlite_master
                    WHERE type = 'table'
                      AND name = 'projects'
                )",
                [],
                |row| row.get(0),
            )
            .expect("sqlite metadata should be readable");

        assert_eq!(table_exists, 1);
    }

    #[test]
    fn projects_migration_accepts_a_valid_project() {
        let connection = migrated_connection();

        insert_project(
            &connection,
            TestProject {
                start_date: Some("2026-07-19"),
                target_date: Some("2026-12-31"),
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        )
        .expect("valid project should be inserted");

        let stored_name: String = connection
            .query_row(
                "SELECT name
                 FROM projects
                 WHERE id = ?1",
                [PROJECT_ID],
                |row| row.get(0),
            )
            .expect("inserted project should be readable");

        assert_eq!(stored_name, "Vertica");
    }

    #[test]
    fn projects_migration_rejects_an_invalid_status() {
        let connection = migrated_connection();

        let result = insert_project(
            &connection,
            TestProject {
                status: "unknown",
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn projects_migration_rejects_an_invalid_project_type() {
        let connection = migrated_connection();

        let result = connection.execute(
            "INSERT INTO projects (
                id,
                name,
                project_type,
                status,
                priority,
                settings_json,
                created_at,
                updated_at
            ) VALUES (
                ?1,
                'Vertica',
                'unknown',
                'active',
                3,
                '{}',
                ?2,
                ?2
            )",
            params![PROJECT_ID, CREATED_AT],
        );

        assert!(result.is_err());
    }

    #[test]
    fn projects_migration_rejects_priority_outside_the_domain_range() {
        let connection = migrated_connection();

        let below_minimum = insert_project(
            &connection,
            TestProject {
                priority: 0,
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        );

        assert!(below_minimum.is_err());

        let above_maximum = insert_project(
            &connection,
            TestProject {
                id: "b415d822-6823-4408-ac93-8f726259304d",
                name: "Another project",
                priority: 6,
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        );

        assert!(above_maximum.is_err());
    }

    #[test]
    fn projects_migration_rejects_an_empty_name() {
        let connection = migrated_connection();

        let result = insert_project(&connection, TestProject::active(PROJECT_ID, "   "));

        assert!(result.is_err());
    }

    #[test]
    fn projects_migration_rejects_a_target_date_before_the_start_date() {
        let connection = migrated_connection();

        let result = insert_project(
            &connection,
            TestProject {
                start_date: Some("2026-12-31"),
                target_date: Some("2026-07-19"),
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn projects_migration_requires_archived_at_for_archived_projects() {
        let connection = migrated_connection();

        let result = insert_project(
            &connection,
            TestProject {
                status: "archived",
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn projects_migration_rejects_archived_at_for_non_archived_projects() {
        let connection = migrated_connection();

        let result = insert_project(
            &connection,
            TestProject {
                archived_at: Some("2026-07-20T12:00:00.000Z"),
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn projects_migration_accepts_a_consistently_archived_project() {
        let connection = migrated_connection();

        insert_project(
            &connection,
            TestProject {
                status: "archived",
                archived_at: Some("2026-07-20T12:00:00.000Z"),
                ..TestProject::active(PROJECT_ID, "Vertica")
            },
        )
        .expect("consistent archived project should be inserted");
    }

    #[test]
    fn projects_migration_requires_valid_json_settings() {
        let connection = migrated_connection();

        let result = connection.execute(
            "INSERT INTO projects (
                id,
                name,
                project_type,
                status,
                priority,
                settings_json,
                created_at,
                updated_at
            ) VALUES (
                ?1,
                'Vertica',
                'software',
                'active',
                3,
                'invalid json',
                ?2,
                ?2
            )",
            params![PROJECT_ID, CREATED_AT],
        );

        assert!(result.is_err());
    }
}
