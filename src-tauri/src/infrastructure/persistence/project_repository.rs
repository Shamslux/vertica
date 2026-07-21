use rusqlite::{params, types::Type, Connection, ErrorCode, OptionalExtension, Row};

use crate::domain::{
    project::{Project, ProjectStatus, ProjectType},
    project_repository::{ProjectRepository, ProjectRepositoryError},
};

use super::{DatabaseManager, PersistenceError, PersistenceResult};

const PROJECT_COLUMNS: &str = "
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
";

pub struct SqliteProjectRepository<'a> {
    database_manager: &'a DatabaseManager,
}

impl<'a> SqliteProjectRepository<'a> {
    pub const fn new(database_manager: &'a DatabaseManager) -> Self {
        Self { database_manager }
    }
}

impl ProjectRepository for SqliteProjectRepository<'_> {
    fn find_by_id(&self, id: &str) -> Result<Option<Project>, ProjectRepositoryError> {
        self.database_manager
            .with_connection(|connection| {
                let sql = format!(
                    "SELECT {PROJECT_COLUMNS}
FROM projects
WHERE id = ?1
AND deleted_at IS NULL"
                );

                connection
                    .query_row(&sql, params![id], project_from_row)
                    .optional()
                    .map_err(PersistenceError::from)
            })
            .map_err(map_persistence_error)
    }

    fn list_active(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
        self.database_manager
            .with_connection(|connection| {
                let sql = format!(
                    "SELECT {PROJECT_COLUMNS}
FROM projects
WHERE deleted_at IS NULL
  AND status != 'archived'
ORDER BY created_at ASC, id ASC"
                );

                read_projects(connection, &sql)
            })
            .map_err(map_persistence_error)
    }

    fn list_archived(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
        self.database_manager
            .with_connection(|connection| {
                let sql = format!(
                    "SELECT {PROJECT_COLUMNS}
FROM projects
WHERE deleted_at IS NULL
  AND status = 'archived'
ORDER BY archived_at DESC, created_at ASC, id ASC"
                );

                read_projects(connection, &sql)
            })
            .map_err(map_persistence_error)
    }

    fn insert(&self, project: &Project) -> Result<(), ProjectRepositoryError> {
        self.database_manager
            .with_connection(|connection| {
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
                    )
                    VALUES (
                        ?1,
                        ?2,
                        ?3,
                        ?4,
                        ?5,
                        ?6,
                        ?7,
                        ?8,
                        ?9,
                        ?10,
                        ?11,
                        ?12,
                        ?13,
                        ?14,
                        ?15,
                        ?16
                    )",
                    params![
                        project.id(),
                        project.name(),
                        project.description(),
                        project.project_type().as_str(),
                        project.status().as_str(),
                        project.start_date(),
                        project.target_date(),
                        project.priority(),
                        project.color(),
                        project.icon(),
                        project.objective(),
                        project.settings_json(),
                        project.created_at(),
                        project.updated_at(),
                        project.archived_at(),
                        project.deleted_at(),
                    ],
                )?;

                Ok(())
            })
            .map_err(|error| map_insert_error(error, project.id()))
    }

    fn update(&self, project: &Project) -> Result<(), ProjectRepositoryError> {
        self.database_manager
            .with_connection(|connection| {
                let affected_rows = connection.execute(
                    "UPDATE projects
SET name = ?2,
    description = ?3,
    project_type = ?4,
    status = ?5,
    start_date = ?6,
    target_date = ?7,
    priority = ?8,
    color = ?9,
    icon = ?10,
    objective = ?11,
    settings_json = ?12,
    created_at = ?13,
    updated_at = ?14,
    archived_at = ?15,
    deleted_at = ?16
WHERE id = ?1",
                    params![
                        project.id(),
                        project.name(),
                        project.description(),
                        project.project_type().as_str(),
                        project.status().as_str(),
                        project.start_date(),
                        project.target_date(),
                        project.priority(),
                        project.color(),
                        project.icon(),
                        project.objective(),
                        project.settings_json(),
                        project.created_at(),
                        project.updated_at(),
                        project.archived_at(),
                        project.deleted_at(),
                    ],
                )?;

                Ok(affected_rows)
            })
            .map_err(map_persistence_error)
            .and_then(|affected_rows| {
                if affected_rows == 0 {
                    Err(ProjectRepositoryError::not_found(project.id()))
                } else {
                    Ok(())
                }
            })
    }
}

fn read_projects(connection: &Connection, sql: &str) -> PersistenceResult<Vec<Project>> {
    let mut statement = connection.prepare(sql)?;

    let projects = statement
        .query_map([], project_from_row)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

fn project_from_row(row: &Row<'_>) -> rusqlite::Result<Project> {
    let project_type_value: String = row.get(3)?;
    let status_value: String = row.get(4)?;
    let priority_value: i64 = row.get(7)?;

    let project_type = project_type_value.parse::<ProjectType>().map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(3, Type::Text, Box::new(error))
    })?;

    let status = status_value.parse::<ProjectStatus>().map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(4, Type::Text, Box::new(error))
    })?;

    let priority = u8::try_from(priority_value).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(7, Type::Integer, Box::new(error))
    })?;

    Project::reconstitute(
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        project_type,
        status,
        row.get(5)?,
        row.get(6)?,
        priority,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
        row.get(12)?,
        row.get(13)?,
        row.get(14)?,
        row.get(15)?,
    )
    .map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(error)))
}

fn map_insert_error(error: PersistenceError, project_id: &str) -> ProjectRepositoryError {
    if is_constraint_violation(&error) {
        ProjectRepositoryError::conflict(project_id)
    } else {
        map_persistence_error(error)
    }
}

fn map_persistence_error(error: PersistenceError) -> ProjectRepositoryError {
    if is_database_unavailable(&error) {
        return ProjectRepositoryError::unavailable();
    }

    if is_corrupted_data_error(&error) {
        return ProjectRepositoryError::corrupted_data(error.to_string());
    }

    ProjectRepositoryError::unexpected(error.to_string())
}

fn is_constraint_violation(error: &PersistenceError) -> bool {
    matches!(
        error,
        PersistenceError::Database(
            rusqlite::Error::SqliteFailure(sqlite_error, _)
        ) if sqlite_error.code == ErrorCode::ConstraintViolation
    )
}

fn is_database_unavailable(error: &PersistenceError) -> bool {
    match error {
        PersistenceError::LockPoisoned => true,
        PersistenceError::Database(rusqlite::Error::SqliteFailure(sqlite_error, _)) => matches!(
            sqlite_error.code,
            ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked | ErrorCode::CannotOpen
        ),
        _ => false,
    }
}

fn is_corrupted_data_error(error: &PersistenceError) -> bool {
    matches!(
        error,
        PersistenceError::Database(rusqlite::Error::FromSqlConversionFailure(_, _, _))
    )
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        project::{NewProject, Project, ProjectChanges, ProjectStatus, ProjectType},
        project_repository::{ProjectRepository, ProjectRepositoryError},
    };
    use crate::infrastructure::persistence::DatabaseManager;

    use super::SqliteProjectRepository;

    const FIRST_PROJECT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
    const SECOND_PROJECT_ID: &str = "550e8400-e29b-41d4-a716-446655440001";
    const UNKNOWN_PROJECT_ID: &str = "550e8400-e29b-41d4-a716-446655440099";

    const CREATED_AT: &str = "2026-07-19T12:00:00.000Z";
    const UPDATED_AT: &str = "2026-07-19T13:00:00.000Z";

    fn create_project(id: &str, name: &str) -> Project {
        Project::create(NewProject::minimal(
            id,
            name,
            ProjectType::Software,
            CREATED_AT,
        ))
        .expect("test project should be valid")
    }

    fn repository_fixture() -> (DatabaseManager, Project, Project) {
        let database_manager =
            DatabaseManager::open_in_memory("test").expect("database should initialize");

        let first = create_project(FIRST_PROJECT_ID, "First");
        let second = create_project(SECOND_PROJECT_ID, "Second");

        (database_manager, first, second)
    }

    #[test]
    fn inserts_and_finds_a_project() -> Result<(), ProjectRepositoryError> {
        let (database_manager, project, _) = repository_fixture();

        let repository = SqliteProjectRepository::new(&database_manager);

        repository.insert(&project)?;

        let stored = repository
            .find_by_id(project.id())?
            .expect("project should exist");

        assert_eq!(stored, project);

        Ok(())
    }

    #[test]
    fn find_by_id_returns_none_for_an_unknown_project() -> Result<(), ProjectRepositoryError> {
        let (database_manager, _, _) = repository_fixture();

        let repository = SqliteProjectRepository::new(&database_manager);

        assert_eq!(repository.find_by_id(UNKNOWN_PROJECT_ID)?, None);

        Ok(())
    }

    #[test]
    fn inserting_the_same_identifier_twice_returns_conflict() -> Result<(), ProjectRepositoryError>
    {
        let (database_manager, project, _) = repository_fixture();

        let repository = SqliteProjectRepository::new(&database_manager);

        repository.insert(&project)?;

        assert_eq!(
            repository.insert(&project),
            Err(ProjectRepositoryError::conflict(project.id()))
        );

        Ok(())
    }

    #[test]
    fn lists_active_and_archived_projects_separately() -> Result<(), ProjectRepositoryError> {
        let (database_manager, active_project, mut archived_project) = repository_fixture();

        archived_project
            .archive(UPDATED_AT)
            .expect("project should archive");

        let repository = SqliteProjectRepository::new(&database_manager);

        repository.insert(&active_project)?;
        repository.insert(&archived_project)?;

        let active = repository.list_active()?;
        let archived = repository.list_archived()?;

        assert_eq!(active, vec![active_project]);
        assert_eq!(archived, vec![archived_project]);

        Ok(())
    }

    #[test]
    fn updates_an_existing_project() -> Result<(), ProjectRepositoryError> {
        let (database_manager, mut project, _) = repository_fixture();

        let repository = SqliteProjectRepository::new(&database_manager);

        repository.insert(&project)?;

        project
            .update(
                ProjectChanges {
                    name: Some("Updated project".to_owned()),
                    priority: Some(1),
                    ..ProjectChanges::default()
                },
                UPDATED_AT,
            )
            .expect("domain update should succeed");

        repository.update(&project)?;

        let stored = repository
            .find_by_id(project.id())?
            .expect("project should exist");

        assert_eq!(stored.name(), "Updated project");
        assert_eq!(stored.priority(), 1);
        assert_eq!(stored.updated_at(), UPDATED_AT);

        Ok(())
    }

    #[test]
    fn updating_an_unknown_project_returns_not_found() {
        let (database_manager, _, _) = repository_fixture();

        let repository = SqliteProjectRepository::new(&database_manager);

        let project = create_project(UNKNOWN_PROJECT_ID, "Unknown");

        assert_eq!(
            repository.update(&project),
            Err(ProjectRepositoryError::not_found(UNKNOWN_PROJECT_ID))
        );
    }

    #[test]
    fn soft_deleted_projects_are_not_returned() -> Result<(), ProjectRepositoryError> {
        let (database_manager, mut project, _) = repository_fixture();

        let repository = SqliteProjectRepository::new(&database_manager);

        repository.insert(&project)?;

        project
            .soft_delete(UPDATED_AT)
            .expect("project should be deleted");

        repository.update(&project)?;

        assert_eq!(repository.find_by_id(project.id())?, None);
        assert!(repository.list_active()?.is_empty());
        assert!(repository.list_archived()?.is_empty());

        Ok(())
    }

    #[test]
    fn archived_projects_are_reconstituted_correctly() -> Result<(), ProjectRepositoryError> {
        let (database_manager, mut project, _) = repository_fixture();

        project.archive(UPDATED_AT).expect("project should archive");

        let repository = SqliteProjectRepository::new(&database_manager);

        repository.insert(&project)?;

        let stored = repository
            .find_by_id(project.id())?
            .expect("project should exist");

        assert_eq!(stored.status(), ProjectStatus::Archived);
        assert_eq!(stored.archived_at(), Some(UPDATED_AT));

        Ok(())
    }
}

