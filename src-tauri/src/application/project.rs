use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize};

use crate::domain::{
    project::{
        NewProject, Project, ProjectChanges, ProjectError, ProjectStatus, ProjectType,
        DEFAULT_PROJECT_PRIORITY,
    },
    project_repository::{ProjectRepository, ProjectRepositoryError},
};

pub trait ProjectIdGenerator: Send + Sync {
    fn generate(&self) -> String;
}

pub trait ProjectClock: Send + Sync {
    fn now(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub priority: Option<u8>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub objective: Option<String>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
    pub name: Option<String>,

    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub description: Option<Option<String>>,

    pub project_type: Option<ProjectType>,

    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub start_date: Option<Option<String>>,

    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub target_date: Option<Option<String>>,

    pub priority: Option<u8>,

    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub color: Option<Option<String>>,

    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub icon: Option<Option<String>>,

    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub objective: Option<Option<String>>,

    pub settings_json: Option<String>,
}

fn deserialize_optional_field<'de, D, T>(
    deserializer: D,
) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

impl From<UpdateProjectRequest> for ProjectChanges {
    fn from(request: UpdateProjectRequest) -> Self {
        Self {
            name: request.name,
            description: request.description,
            project_type: request.project_type,
            start_date: request.start_date,
            target_date: request.target_date,
            priority: request.priority,
            color: request.color,
            icon: request.icon,
            objective: request.objective,
            settings_json: request.settings_json,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
    pub status: ProjectStatus,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub priority: u8,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub objective: Option<String>,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

impl From<&Project> for ProjectResponse {
    fn from(project: &Project) -> Self {
        Self {
            id: project.id().to_owned(),
            name: project.name().to_owned(),
            description: project.description().map(str::to_owned),
            project_type: project.project_type(),
            status: project.status(),
            start_date: project.start_date().map(str::to_owned),
            target_date: project.target_date().map(str::to_owned),
            priority: project.priority(),
            color: project.color().map(str::to_owned),
            icon: project.icon().map(str::to_owned),
            objective: project.objective().map(str::to_owned),
            settings_json: project.settings_json().to_owned(),
            created_at: project.created_at().to_owned(),
            updated_at: project.updated_at().to_owned(),
            archived_at: project.archived_at().map(str::to_owned),
        }
    }
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self::from(&project)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectServiceError {
    Validation { message: String },
    NotFound { id: String },
    Conflict { id: String },
    StorageUnavailable,
    InvalidStoredData,
    Unexpected,
}

impl ProjectServiceError {
    fn validation(error: ProjectError) -> Self {
        Self::Validation {
            message: error.to_string(),
        }
    }
}

impl Display for ProjectServiceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation { message } => formatter.write_str(message),
            Self::NotFound { id } => write!(formatter, "project `{id}` was not found"),
            Self::Conflict { id } => write!(formatter, "project `{id}` already exists"),
            Self::StorageUnavailable => formatter.write_str("project storage is unavailable"),
            Self::InvalidStoredData => formatter.write_str("stored project data is invalid"),
            Self::Unexpected => formatter.write_str("an unexpected project error occurred"),
        }
    }
}

impl Error for ProjectServiceError {}

impl From<ProjectError> for ProjectServiceError {
    fn from(error: ProjectError) -> Self {
        Self::validation(error)
    }
}

impl From<ProjectRepositoryError> for ProjectServiceError {
    fn from(error: ProjectRepositoryError) -> Self {
        match error {
            ProjectRepositoryError::NotFound { id } => Self::NotFound { id },
            ProjectRepositoryError::Conflict { id } => Self::Conflict { id },
            ProjectRepositoryError::Unavailable => Self::StorageUnavailable,
            ProjectRepositoryError::CorruptedData { .. } => Self::InvalidStoredData,
            ProjectRepositoryError::Unexpected { .. } => Self::Unexpected,
        }
    }
}

pub struct ProjectService<R, I, C> {
    repository: R,
    id_generator: I,
    clock: C,
}

impl<R, I, C> ProjectService<R, I, C>
where
    R: ProjectRepository,
    I: ProjectIdGenerator,
    C: ProjectClock,
{
    pub const fn new(repository: R, id_generator: I, clock: C) -> Self {
        Self {
            repository,
            id_generator,
            clock,
        }
    }

    pub fn create(
        &self,
        request: CreateProjectRequest,
    ) -> Result<ProjectResponse, ProjectServiceError> {
        let created_at = self.clock.now();

        let project = Project::create(NewProject {
            id: self.id_generator.generate(),
            name: request.name,
            description: request.description,
            project_type: request.project_type,
            start_date: request.start_date,
            target_date: request.target_date,
            priority: request.priority.unwrap_or(DEFAULT_PROJECT_PRIORITY),
            color: request.color,
            icon: request.icon,
            objective: request.objective,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_owned()),
            created_at,
        })?;

        self.repository.insert(&project)?;

        Ok(project.into())
    }

    pub fn find_by_id(&self, id: &str) -> Result<ProjectResponse, ProjectServiceError> {
        let project = self.find_project(id)?;

        Ok(project.into())
    }

    pub fn list_active(&self) -> Result<Vec<ProjectResponse>, ProjectServiceError> {
        Ok(self
            .repository
            .list_active()?
            .into_iter()
            .map(ProjectResponse::from)
            .collect())
    }

    pub fn list_archived(&self) -> Result<Vec<ProjectResponse>, ProjectServiceError> {
        Ok(self
            .repository
            .list_archived()?
            .into_iter()
            .map(ProjectResponse::from)
            .collect())
    }

    pub fn update(
        &self,
        id: &str,
        request: UpdateProjectRequest,
    ) -> Result<ProjectResponse, ProjectServiceError> {
        let mut project = self.find_project(id)?;

        project.update(request.into(), self.clock.now())?;
        self.repository.update(&project)?;

        Ok(project.into())
    }

    pub fn archive(&self, id: &str) -> Result<ProjectResponse, ProjectServiceError> {
        let mut project = self.find_project(id)?;

        project.archive(self.clock.now())?;
        self.repository.update(&project)?;

        Ok(project.into())
    }

    pub fn restore(&self, id: &str) -> Result<ProjectResponse, ProjectServiceError> {
        let mut project = self.find_project(id)?;

        project.restore(self.clock.now())?;
        self.repository.update(&project)?;

        Ok(project.into())
    }

    fn find_project(&self, id: &str) -> Result<Project, ProjectServiceError> {
        self.repository
            .find_by_id(id)?
            .ok_or_else(|| ProjectServiceError::NotFound { id: id.to_owned() })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use super::*;

    const PROJECT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
    const CREATED_AT: &str = "2026-07-19T12:00:00.000Z";
    const UPDATED_AT: &str = "2026-07-19T13:00:00.000Z";

    struct FixedIdGenerator;

    impl ProjectIdGenerator for FixedIdGenerator {
        fn generate(&self) -> String {
            PROJECT_ID.to_owned()
        }
    }

    struct SequenceClock {
        values: Mutex<Vec<String>>,
    }

    impl SequenceClock {
        fn new(values: &[&str]) -> Self {
            Self {
                values: Mutex::new(
                    values
                        .iter()
                        .rev()
                        .map(|value| (*value).to_owned())
                        .collect(),
                ),
            }
        }
    }

    impl ProjectClock for SequenceClock {
        fn now(&self) -> String {
            self.values
                .lock()
                .expect("clock lock should not be poisoned")
                .pop()
                .expect("the test clock should have another value")
        }
    }

    #[derive(Default)]
    struct InMemoryProjectRepository {
        projects: Mutex<HashMap<String, Project>>,
        next_error: Mutex<Option<ProjectRepositoryError>>,
    }

    impl InMemoryProjectRepository {
        fn fail_next(&self, error: ProjectRepositoryError) {
            *self
                .next_error
                .lock()
                .expect("error lock should not be poisoned") = Some(error);
        }

        fn take_error(&self) -> Result<(), ProjectRepositoryError> {
            match self
                .next_error
                .lock()
                .expect("error lock should not be poisoned")
                .take()
            {
                Some(error) => Err(error),
                None => Ok(()),
            }
        }
    }

    impl ProjectRepository for InMemoryProjectRepository {
        fn find_by_id(&self, id: &str) -> Result<Option<Project>, ProjectRepositoryError> {
            self.take_error()?;

            Ok(self
                .projects
                .lock()
                .expect("project lock should not be poisoned")
                .get(id)
                .filter(|project| !project.is_deleted())
                .cloned())
        }

        fn list_active(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
            self.take_error()?;

            let mut projects = self
                .projects
                .lock()
                .expect("project lock should not be poisoned")
                .values()
                .filter(|project| !project.is_deleted() && !project.is_archived())
                .cloned()
                .collect::<Vec<_>>();

            projects.sort_by(|left, right| {
                left.created_at()
                    .cmp(right.created_at())
                    .then_with(|| left.id().cmp(right.id()))
            });

            Ok(projects)
        }

        fn list_archived(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
            self.take_error()?;

            let mut projects = self
                .projects
                .lock()
                .expect("project lock should not be poisoned")
                .values()
                .filter(|project| !project.is_deleted() && project.is_archived())
                .cloned()
                .collect::<Vec<_>>();

            projects.sort_by(|left, right| {
                right
                    .archived_at()
                    .cmp(&left.archived_at())
                    .then_with(|| left.created_at().cmp(right.created_at()))
                    .then_with(|| left.id().cmp(right.id()))
            });

            Ok(projects)
        }

        fn insert(&self, project: &Project) -> Result<(), ProjectRepositoryError> {
            self.take_error()?;

            let mut projects = self
                .projects
                .lock()
                .expect("project lock should not be poisoned");

            if projects.contains_key(project.id()) {
                return Err(ProjectRepositoryError::conflict(project.id()));
            }

            projects.insert(project.id().to_owned(), project.clone());

            Ok(())
        }

        fn update(&self, project: &Project) -> Result<(), ProjectRepositoryError> {
            self.take_error()?;

            let mut projects = self
                .projects
                .lock()
                .expect("project lock should not be poisoned");

            if !projects.contains_key(project.id()) {
                return Err(ProjectRepositoryError::not_found(project.id()));
            }

            projects.insert(project.id().to_owned(), project.clone());

            Ok(())
        }
    }

    fn request(name: &str) -> CreateProjectRequest {
        CreateProjectRequest {
            name: name.to_owned(),
            description: None,
            project_type: ProjectType::Software,
            start_date: None,
            target_date: None,
            priority: None,
            color: None,
            icon: None,
            objective: None,
            settings_json: None,
        }
    }

    fn service(
        timestamps: &[&str],
    ) -> ProjectService<InMemoryProjectRepository, FixedIdGenerator, SequenceClock> {
        ProjectService::new(
            InMemoryProjectRepository::default(),
            FixedIdGenerator,
            SequenceClock::new(timestamps),
        )
    }

    #[test]
    fn creates_and_persists_a_project_with_application_generated_values() {
        let service = service(&[CREATED_AT]);

        let project = service
            .create(request("  Vertica  "))
            .expect("project should be created");

        assert_eq!(project.id, PROJECT_ID);
        assert_eq!(project.name, "Vertica");
        assert_eq!(project.status, ProjectStatus::Active);
        assert_eq!(project.priority, DEFAULT_PROJECT_PRIORITY);
        assert_eq!(project.settings_json, "{}");
        assert_eq!(project.created_at, CREATED_AT);
        assert_eq!(project.updated_at, CREATED_AT);

        assert_eq!(
            service
                .find_by_id(PROJECT_ID)
                .expect("project should be persisted"),
            project
        );
    }

    #[test]
    fn rejects_invalid_creation_without_persisting_it() {
        let service = service(&[CREATED_AT]);

        let error = service
            .create(request("   "))
            .expect_err("empty names should be rejected");

        assert_eq!(
            error,
            ProjectServiceError::Validation {
                message: "field `name` is required".to_owned(),
            }
        );

        assert!(service
            .list_active()
            .expect("listing should succeed")
            .is_empty());
    }

    #[test]
    fn returns_not_found_for_an_unknown_identifier() {
        let service = service(&[]);

        assert_eq!(
            service.find_by_id(PROJECT_ID),
            Err(ProjectServiceError::NotFound {
                id: PROJECT_ID.to_owned(),
            })
        );
    }

    #[test]
    fn updates_an_existing_project() {
        let service = service(&[CREATED_AT, UPDATED_AT]);

        service
            .create(request("Vertica"))
            .expect("project should be created");

        let updated = service
            .update(
                PROJECT_ID,
                UpdateProjectRequest {
                    name: Some("Vertica Desktop".to_owned()),
                    priority: Some(1),
                    ..UpdateProjectRequest::default()
                },
            )
            .expect("project should be updated");

        assert_eq!(updated.name, "Vertica Desktop");
        assert_eq!(updated.priority, 1);
        assert_eq!(updated.created_at, CREATED_AT);
        assert_eq!(updated.updated_at, UPDATED_AT);
    }

    #[test]
    fn archives_and_restores_a_project() {
        let service = service(&[
            CREATED_AT,
            "2026-07-19T13:00:00.000Z",
            "2026-07-19T14:00:00.000Z",
        ]);

        service
            .create(request("Vertica"))
            .expect("project should be created");

        let archived = service
            .archive(PROJECT_ID)
            .expect("project should be archived");

        assert_eq!(archived.status, ProjectStatus::Archived);
        assert_eq!(
            archived.archived_at.as_deref(),
            Some("2026-07-19T13:00:00.000Z")
        );

        assert!(service
            .list_active()
            .expect("active listing should succeed")
            .is_empty());

        assert_eq!(
            service
                .list_archived()
                .expect("archived listing should succeed"),
            vec![archived]
        );

        let restored = service
            .restore(PROJECT_ID)
            .expect("project should be restored");

        assert_eq!(restored.status, ProjectStatus::Active);
        assert_eq!(restored.archived_at, None);
        assert_eq!(restored.updated_at, "2026-07-19T14:00:00.000Z");
    }

    #[test]
    fn maps_repository_failures_to_safe_application_errors() {
        let service = service(&[]);

        service
            .repository
            .fail_next(ProjectRepositoryError::unexpected(
                "sensitive database details",
            ));

        assert_eq!(service.list_active(), Err(ProjectServiceError::Unexpected));

        assert_eq!(
            ProjectServiceError::Unexpected.to_string(),
            "an unexpected project error occurred"
        );
    }

    #[test]
    fn maps_repository_conflicts() {
        let service = service(&[CREATED_AT, UPDATED_AT]);

        service
            .create(request("First"))
            .expect("first project should be created");

        assert_eq!(
            service.create(request("Duplicate")),
            Err(ProjectServiceError::Conflict {
                id: PROJECT_ID.to_owned(),
            })
        );
    }

    #[test]
    fn deserializes_missing_update_field_as_unchanged() {
        let request: UpdateProjectRequest =
            serde_json::from_str(r#"{"name":"Updated"}"#)
                .expect("request should deserialize");

        assert_eq!(request.name.as_deref(), Some("Updated"));
        assert_eq!(request.description, None);
    }

    #[test]
    fn deserializes_null_update_field_as_clear_value() {
        let request: UpdateProjectRequest =
            serde_json::from_str(r#"{"description":null}"#)
                .expect("request should deserialize");

        assert_eq!(request.description, Some(None));
    }

    #[test]
    fn deserializes_update_field_value_as_replacement() {
        let request: UpdateProjectRequest =
            serde_json::from_str(r#"{"description":"Updated description"}"#)
                .expect("request should deserialize");

        assert_eq!(
            request.description,
            Some(Some("Updated description".to_owned()))
        );
    }
}

