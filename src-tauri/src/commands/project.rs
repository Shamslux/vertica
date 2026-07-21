use chrono::{SecondsFormat, Utc};
use tauri::State;
use uuid::Uuid;

use crate::{
    application::project::{
        CreateProjectRequest, ProjectClock, ProjectIdGenerator, ProjectResponse, ProjectService,
        UpdateProjectRequest,
    },
    infrastructure::persistence::{DatabaseManager, SqliteProjectRepository},
};

#[derive(Debug, Clone, Copy, Default)]
struct UuidProjectIdGenerator;

impl ProjectIdGenerator for UuidProjectIdGenerator {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct UtcProjectClock;

impl ProjectClock for UtcProjectClock {
    fn now(&self) -> String {
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
    }
}

type SqliteProjectService<'a> =
    ProjectService<SqliteProjectRepository<'a>, UuidProjectIdGenerator, UtcProjectClock>;

fn project_service(database_manager: &DatabaseManager) -> SqliteProjectService<'_> {
    ProjectService::new(
        SqliteProjectRepository::new(database_manager),
        UuidProjectIdGenerator,
        UtcProjectClock,
    )
}

#[tauri::command]
pub fn create_project(
    input: CreateProjectRequest,
    database_manager: State<'_, DatabaseManager>,
) -> Result<ProjectResponse, String> {
    project_service(database_manager.inner())
        .create(input)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_project(
    id: String,
    database_manager: State<'_, DatabaseManager>,
) -> Result<ProjectResponse, String> {
    project_service(database_manager.inner())
        .find_by_id(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_active_projects(
    database_manager: State<'_, DatabaseManager>,
) -> Result<Vec<ProjectResponse>, String> {
    project_service(database_manager.inner())
        .list_active()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_archived_projects(
    database_manager: State<'_, DatabaseManager>,
) -> Result<Vec<ProjectResponse>, String> {
    project_service(database_manager.inner())
        .list_archived()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_project(
    id: String,
    input: UpdateProjectRequest,
    database_manager: State<'_, DatabaseManager>,
) -> Result<ProjectResponse, String> {
    project_service(database_manager.inner())
        .update(&id, input)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn archive_project(
    id: String,
    database_manager: State<'_, DatabaseManager>,
) -> Result<ProjectResponse, String> {
    project_service(database_manager.inner())
        .archive(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn restore_project(
    id: String,
    database_manager: State<'_, DatabaseManager>,
) -> Result<ProjectResponse, String> {
    project_service(database_manager.inner())
        .restore(&id)
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::{
        ProjectClock, ProjectIdGenerator, UtcProjectClock, UuidProjectIdGenerator,
    };

    #[test]
    fn generates_a_valid_uuid_project_id() {
        let generator = UuidProjectIdGenerator;

        let generated_id = generator.generate();

        assert!(Uuid::parse_str(&generated_id).is_ok());
    }

    #[test]
    fn generates_a_utc_timestamp_with_millisecond_precision() {
        let clock = UtcProjectClock;

        let timestamp = clock.now();

        assert!(timestamp.ends_with('Z'));
        assert!(chrono::DateTime::parse_from_rfc3339(&timestamp).is_ok());
    }
}

