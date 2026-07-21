use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub const DEFAULT_PROJECT_PRIORITY: u8 = 3;
pub const MIN_PROJECT_PRIORITY: u8 = 1;
pub const MAX_PROJECT_PRIORITY: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Active,
    Paused,
    Completed,
    Archived,
    Cancelled,
}

impl ProjectStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Archived => "archived",
            Self::Cancelled => "cancelled",
        }
    }
}

impl Display for ProjectStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ProjectStatus {
    type Err = ProjectError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "completed" => Ok(Self::Completed),
            "archived" => Ok(Self::Archived),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(ProjectError::InvalidStatus(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Custom,
    Learning,
    Certification,
    Reading,
    Software,
    Fitness,
    Administrative,
}

impl ProjectType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Learning => "learning",
            Self::Certification => "certification",
            Self::Reading => "reading",
            Self::Software => "software",
            Self::Fitness => "fitness",
            Self::Administrative => "administrative",
        }
    }
}

impl Display for ProjectType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ProjectType {
    type Err = ProjectError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "custom" => Ok(Self::Custom),
            "learning" => Ok(Self::Learning),
            "certification" => Ok(Self::Certification),
            "reading" => Ok(Self::Reading),
            "software" => Ok(Self::Software),
            "fitness" => Ok(Self::Fitness),
            "administrative" => Ok(Self::Administrative),
            _ => Err(ProjectError::InvalidType(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub priority: u8,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub objective: Option<String>,
    pub settings_json: String,
    pub created_at: String,
}

impl NewProject {
    pub fn minimal(
        id: impl Into<String>,
        name: impl Into<String>,
        project_type: ProjectType,
        created_at: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            project_type,
            start_date: None,
            target_date: None,
            priority: DEFAULT_PROJECT_PRIORITY,
            color: None,
            icon: None,
            objective: None,
            settings_json: "{}".to_owned(),
            created_at: created_at.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectChanges {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub project_type: Option<ProjectType>,
    pub start_date: Option<Option<String>>,
    pub target_date: Option<Option<String>>,
    pub priority: Option<u8>,
    pub color: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    pub objective: Option<Option<String>>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    id: String,
    name: String,
    description: Option<String>,
    project_type: ProjectType,
    status: ProjectStatus,
    start_date: Option<String>,
    target_date: Option<String>,
    priority: u8,
    color: Option<String>,
    icon: Option<String>,
    objective: Option<String>,
    settings_json: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
    deleted_at: Option<String>,
}

impl Project {
    pub fn create(input: NewProject) -> Result<Self, ProjectError> {
        validate_uuid(&input.id)?;

        let name = required_text("name", input.name)?;
        validate_priority(input.priority)?;

        let start_date = optional_text(input.start_date);
        let target_date = optional_text(input.target_date);
        validate_date_range(start_date.as_deref(), target_date.as_deref())?;

        let created_at = required_text("created_at", input.created_at)?;
        let settings_json = required_text("settings_json", input.settings_json)?;

        Ok(Self {
            id: input.id,
            name,
            description: optional_text(input.description),
            project_type: input.project_type,
            status: ProjectStatus::Active,
            start_date,
            target_date,
            priority: input.priority,
            color: optional_text(input.color),
            icon: optional_text(input.icon),
            objective: optional_text(input.objective),
            settings_json,
            created_at: created_at.clone(),
            updated_at: created_at,
            archived_at: None,
            deleted_at: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: String,
        name: String,
        description: Option<String>,
        project_type: ProjectType,
        status: ProjectStatus,
        start_date: Option<String>,
        target_date: Option<String>,
        priority: u8,
        color: Option<String>,
        icon: Option<String>,
        objective: Option<String>,
        settings_json: String,
        created_at: String,
        updated_at: String,
        archived_at: Option<String>,
        deleted_at: Option<String>,
    ) -> Result<Self, ProjectError> {
        validate_uuid(&id)?;

        let name = required_text("name", name)?;
        validate_priority(priority)?;

        let start_date = optional_text(start_date);
        let target_date = optional_text(target_date);
        validate_date_range(start_date.as_deref(), target_date.as_deref())?;

        let created_at = required_text("created_at", created_at)?;
        let updated_at = required_text("updated_at", updated_at)?;
        let settings_json = required_text("settings_json", settings_json)?;
        let archived_at = optional_text(archived_at);
        let deleted_at = optional_text(deleted_at);

        if status == ProjectStatus::Archived && archived_at.is_none() {
            return Err(ProjectError::ArchivedAtRequired);
        }

        if status != ProjectStatus::Archived && archived_at.is_some() {
            return Err(ProjectError::ArchivedAtNotAllowed(status));
        }

        Ok(Self {
            id,
            name,
            description: optional_text(description),
            project_type,
            status,
            start_date,
            target_date,
            priority,
            color: optional_text(color),
            icon: optional_text(icon),
            objective: optional_text(objective),
            settings_json,
            created_at,
            updated_at,
            archived_at,
            deleted_at,
        })
    }

    pub fn update(
        &mut self,
        changes: ProjectChanges,
        updated_at: impl Into<String>,
    ) -> Result<(), ProjectError> {
        self.ensure_not_deleted()?;

        let updated_at = required_text("updated_at", updated_at.into())?;

        let next_name = match changes.name {
            Some(name) => required_text("name", name)?,
            None => self.name.clone(),
        };

        let next_description = changes
            .description
            .map(optional_text)
            .unwrap_or_else(|| self.description.clone());

        let next_project_type = changes.project_type.unwrap_or(self.project_type);

        let next_start_date = changes
            .start_date
            .map(optional_text)
            .unwrap_or_else(|| self.start_date.clone());

        let next_target_date = changes
            .target_date
            .map(optional_text)
            .unwrap_or_else(|| self.target_date.clone());

        let next_priority = changes.priority.unwrap_or(self.priority);

        let next_color = changes
            .color
            .map(optional_text)
            .unwrap_or_else(|| self.color.clone());

        let next_icon = changes
            .icon
            .map(optional_text)
            .unwrap_or_else(|| self.icon.clone());

        let next_objective = changes
            .objective
            .map(optional_text)
            .unwrap_or_else(|| self.objective.clone());

        let next_settings_json = match changes.settings_json {
            Some(settings_json) => required_text("settings_json", settings_json)?,
            None => self.settings_json.clone(),
        };

        validate_priority(next_priority)?;
        validate_date_range(next_start_date.as_deref(), next_target_date.as_deref())?;

        self.name = next_name;
        self.description = next_description;
        self.project_type = next_project_type;
        self.start_date = next_start_date;
        self.target_date = next_target_date;
        self.priority = next_priority;
        self.color = next_color;
        self.icon = next_icon;
        self.objective = next_objective;
        self.settings_json = next_settings_json;
        self.updated_at = updated_at;

        Ok(())
    }

    pub fn pause(&mut self, changed_at: impl Into<String>) -> Result<(), ProjectError> {
        self.transition(
            ProjectStatus::Paused,
            &[ProjectStatus::Active],
            changed_at,
        )
    }

    pub fn resume(&mut self, changed_at: impl Into<String>) -> Result<(), ProjectError> {
        self.transition(
            ProjectStatus::Active,
            &[ProjectStatus::Paused],
            changed_at,
        )
    }

    pub fn complete(&mut self, changed_at: impl Into<String>) -> Result<(), ProjectError> {
        self.transition(
            ProjectStatus::Completed,
            &[ProjectStatus::Active, ProjectStatus::Paused],
            changed_at,
        )
    }

    pub fn cancel(&mut self, changed_at: impl Into<String>) -> Result<(), ProjectError> {
        self.transition(
            ProjectStatus::Cancelled,
            &[ProjectStatus::Active, ProjectStatus::Paused],
            changed_at,
        )
    }

    pub fn archive(&mut self, changed_at: impl Into<String>) -> Result<(), ProjectError> {
        self.ensure_not_deleted()?;

        if self.status == ProjectStatus::Archived {
            return Err(ProjectError::AlreadyInStatus(ProjectStatus::Archived));
        }

        let changed_at = required_text("changed_at", changed_at.into())?;

        self.status = ProjectStatus::Archived;
        self.archived_at = Some(changed_at.clone());
        self.updated_at = changed_at;

        Ok(())
    }

    /// Restoring an archived project returns it to `active`.
    pub fn restore(&mut self, changed_at: impl Into<String>) -> Result<(), ProjectError> {
        self.ensure_not_deleted()?;

        if self.status != ProjectStatus::Archived {
            return Err(ProjectError::InvalidTransition {
                from: self.status,
                to: ProjectStatus::Active,
            });
        }

        let changed_at = required_text("changed_at", changed_at.into())?;

        self.status = ProjectStatus::Active;
        self.archived_at = None;
        self.updated_at = changed_at;

        Ok(())
    }

    pub fn soft_delete(&mut self, deleted_at: impl Into<String>) -> Result<(), ProjectError> {
        self.ensure_not_deleted()?;

        let deleted_at = required_text("deleted_at", deleted_at.into())?;

        self.deleted_at = Some(deleted_at.clone());
        self.updated_at = deleted_at;

        Ok(())
    }

    fn transition(
        &mut self,
        target: ProjectStatus,
        allowed_sources: &[ProjectStatus],
        changed_at: impl Into<String>,
    ) -> Result<(), ProjectError> {
        self.ensure_not_deleted()?;

        if self.status == target {
            return Err(ProjectError::AlreadyInStatus(target));
        }

        if !allowed_sources.contains(&self.status) {
            return Err(ProjectError::InvalidTransition {
                from: self.status,
                to: target,
            });
        }

        let changed_at = required_text("changed_at", changed_at.into())?;

        self.status = target;
        self.archived_at = None;
        self.updated_at = changed_at;

        Ok(())
    }

    fn ensure_not_deleted(&self) -> Result<(), ProjectError> {
        if self.is_deleted() {
            Err(ProjectError::ProjectDeleted)
        } else {
            Ok(())
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub const fn project_type(&self) -> ProjectType {
        self.project_type
    }

    pub const fn status(&self) -> ProjectStatus {
        self.status
    }

    pub fn start_date(&self) -> Option<&str> {
        self.start_date.as_deref()
    }

    pub fn target_date(&self) -> Option<&str> {
        self.target_date.as_deref()
    }

    pub const fn priority(&self) -> u8 {
        self.priority
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn objective(&self) -> Option<&str> {
        self.objective.as_deref()
    }

    pub fn settings_json(&self) -> &str {
        &self.settings_json
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn archived_at(&self) -> Option<&str> {
        self.archived_at.as_deref()
    }

    pub fn deleted_at(&self) -> Option<&str> {
        self.deleted_at.as_deref()
    }

    pub const fn is_archived(&self) -> bool {
        matches!(self.status, ProjectStatus::Archived)
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectError {
    RequiredField(&'static str),
    InvalidId(String),
    InvalidPriority(u8),
    InvalidDate {
        field: &'static str,
        value: String,
    },
    InvalidDateRange {
        start_date: String,
        target_date: String,
    },
    InvalidStatus(String),
    InvalidType(String),
    InvalidTransition {
        from: ProjectStatus,
        to: ProjectStatus,
    },
    AlreadyInStatus(ProjectStatus),
    ArchivedAtRequired,
    ArchivedAtNotAllowed(ProjectStatus),
    ProjectDeleted,
}

impl Display for ProjectError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequiredField(field) => {
                write!(formatter, "field `{field}` is required")
            }
            Self::InvalidId(id) => {
                write!(formatter, "`{id}` is not a canonical UUID")
            }
            Self::InvalidPriority(priority) => write!(
                formatter,
                "priority must be between {MIN_PROJECT_PRIORITY} and \
                 {MAX_PROJECT_PRIORITY}; received {priority}"
            ),
            Self::InvalidDate { field, value } => write!(
                formatter,
                "field `{field}` must use YYYY-MM-DD; received `{value}`"
            ),
            Self::InvalidDateRange {
                start_date,
                target_date,
            } => write!(
                formatter,
                "target date `{target_date}` cannot be earlier than \
                 start date `{start_date}`"
            ),
            Self::InvalidStatus(status) => {
                write!(formatter, "unknown project status `{status}`")
            }
            Self::InvalidType(project_type) => {
                write!(formatter, "unknown project type `{project_type}`")
            }
            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "cannot transition project from `{from}` to `{to}`"
                )
            }
            Self::AlreadyInStatus(status) => {
                write!(formatter, "project is already in status `{status}`")
            }
            Self::ArchivedAtRequired => {
                formatter.write_str("an archived project requires `archived_at`")
            }
            Self::ArchivedAtNotAllowed(status) => write!(
                formatter,
                "`archived_at` is not allowed when project status is `{status}`"
            ),
            Self::ProjectDeleted => {
                formatter.write_str("a deleted project cannot be changed")
            }
        }
    }
}

impl Error for ProjectError {}

fn required_text(field: &'static str, value: String) -> Result<String, ProjectError> {
    let normalized = value.trim();

    if normalized.is_empty() {
        Err(ProjectError::RequiredField(field))
    } else {
        Ok(normalized.to_owned())
    }
}

fn optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let normalized = value.trim();
        (!normalized.is_empty()).then(|| normalized.to_owned())
    })
}

fn validate_priority(priority: u8) -> Result<(), ProjectError> {
    if (MIN_PROJECT_PRIORITY..=MAX_PROJECT_PRIORITY).contains(&priority) {
        Ok(())
    } else {
        Err(ProjectError::InvalidPriority(priority))
    }
}

fn validate_date_range(
    start_date: Option<&str>,
    target_date: Option<&str>,
) -> Result<(), ProjectError> {
    if let Some(start_date) = start_date {
        validate_iso_date("start_date", start_date)?;
    }

    if let Some(target_date) = target_date {
        validate_iso_date("target_date", target_date)?;
    }

    if let (Some(start_date), Some(target_date)) = (start_date, target_date) {
        if target_date < start_date {
            return Err(ProjectError::InvalidDateRange {
                start_date: start_date.to_owned(),
                target_date: target_date.to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_iso_date(field: &'static str, value: &str) -> Result<(), ProjectError> {
    let bytes = value.as_bytes();

    let valid_shape = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit());

    if !valid_shape {
        return Err(ProjectError::InvalidDate {
            field,
            value: value.to_owned(),
        });
    }

    let year = value[0..4].parse::<u16>().unwrap_or_default();
    let month = value[5..7].parse::<u8>().unwrap_or_default();
    let day = value[8..10].parse::<u8>().unwrap_or_default();

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    };

    if year == 0 || day == 0 || day > max_day {
        return Err(ProjectError::InvalidDate {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

const fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn validate_uuid(value: &str) -> Result<(), ProjectError> {
    let bytes = value.as_bytes();

    let valid = bytes.len() == 36
        && bytes[8] == b'-'
        && bytes[13] == b'-'
        && bytes[18] == b'-'
        && bytes[23] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| {
                matches!(index, 8 | 13 | 18 | 23) || byte.is_ascii_hexdigit()
            });

    if valid {
        Ok(())
    } else {
        Err(ProjectError::InvalidId(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROJECT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
    const CREATED_AT: &str = "2026-07-19T12:00:00.000Z";

    fn project() -> Project {
        Project::create(NewProject::minimal(
            PROJECT_ID,
            "Vertica",
            ProjectType::Software,
            CREATED_AT,
        ))
        .expect("the fixture must be valid")
    }

    #[test]
    fn creates_an_active_project_with_normalized_name_and_defaults() {
        let input = NewProject::minimal(
            PROJECT_ID,
            "  Vertica  ",
            ProjectType::Software,
            CREATED_AT,
        );

        let project = Project::create(input).expect("project should be created");

        assert_eq!(project.name(), "Vertica");
        assert_eq!(project.status(), ProjectStatus::Active);
        assert_eq!(project.priority(), DEFAULT_PROJECT_PRIORITY);
        assert_eq!(project.settings_json(), "{}");
        assert_eq!(project.created_at(), CREATED_AT);
        assert_eq!(project.updated_at(), CREATED_AT);
    }

    #[test]
    fn rejects_an_empty_name() {
        let input =
            NewProject::minimal(PROJECT_ID, "   ", ProjectType::Custom, CREATED_AT);

        assert_eq!(
            Project::create(input),
            Err(ProjectError::RequiredField("name"))
        );
    }

    #[test]
    fn rejects_a_non_canonical_uuid() {
        let input =
            NewProject::minimal("not-an-id", "Vertica", ProjectType::Custom, CREATED_AT);

        assert_eq!(
            Project::create(input),
            Err(ProjectError::InvalidId("not-an-id".to_owned()))
        );
    }

    #[test]
    fn rejects_priority_outside_the_domain_range() {
        let mut input =
            NewProject::minimal(PROJECT_ID, "Vertica", ProjectType::Custom, CREATED_AT);

        input.priority = 6;

        assert_eq!(
            Project::create(input),
            Err(ProjectError::InvalidPriority(6))
        );
    }

    #[test]
    fn rejects_an_invalid_calendar_date() {
        let mut input =
            NewProject::minimal(PROJECT_ID, "Vertica", ProjectType::Custom, CREATED_AT);

        input.start_date = Some("2026-02-30".to_owned());

        assert_eq!(
            Project::create(input),
            Err(ProjectError::InvalidDate {
                field: "start_date",
                value: "2026-02-30".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_a_target_date_before_the_start_date() {
        let mut input =
            NewProject::minimal(PROJECT_ID, "Vertica", ProjectType::Custom, CREATED_AT);

        input.start_date = Some("2026-07-20".to_owned());
        input.target_date = Some("2026-07-19".to_owned());

        assert!(matches!(
            Project::create(input),
            Err(ProjectError::InvalidDateRange { .. })
        ));
    }

    #[test]
    fn updates_project_data_atomically() {
        let mut project = project();

        let changes = ProjectChanges {
            name: Some("  Vertica Desktop  ".to_owned()),
            description: Some(Some("  Project workspace  ".to_owned())),
            priority: Some(1),
            ..ProjectChanges::default()
        };

        project
            .update(changes, "2026-07-19T13:00:00.000Z")
            .expect("update should succeed");

        assert_eq!(project.name(), "Vertica Desktop");
        assert_eq!(project.description(), Some("Project workspace"));
        assert_eq!(project.priority(), 1);
        assert_eq!(project.updated_at(), "2026-07-19T13:00:00.000Z");
        assert_eq!(project.created_at(), CREATED_AT);
    }

    #[test]
    fn failed_update_does_not_partially_mutate_the_project() {
        let mut project = project();
        let before = project.clone();

        let changes = ProjectChanges {
            name: Some("Changed".to_owned()),
            priority: Some(9),
            ..ProjectChanges::default()
        };

        assert_eq!(
            project.update(changes, "2026-07-19T13:00:00.000Z"),
            Err(ProjectError::InvalidPriority(9))
        );

        assert_eq!(project, before);
    }

    #[test]
    fn supports_pause_and_resume() {
        let mut project = project();

        project
            .pause("2026-07-19T13:00:00.000Z")
            .expect("active projects can be paused");

        assert_eq!(project.status(), ProjectStatus::Paused);

        project
            .resume("2026-07-19T14:00:00.000Z")
            .expect("paused projects can be resumed");

        assert_eq!(project.status(), ProjectStatus::Active);
    }

    #[test]
    fn rejects_invalid_lifecycle_transitions() {
        let mut project = project();

        project
            .complete("2026-07-19T13:00:00.000Z")
            .expect("active projects can be completed");

        assert_eq!(
            project.pause("2026-07-19T14:00:00.000Z"),
            Err(ProjectError::InvalidTransition {
                from: ProjectStatus::Completed,
                to: ProjectStatus::Paused,
            })
        );
    }

    #[test]
    fn archives_and_restores_to_active() {
        let mut project = project();

        project
            .archive("2026-07-19T13:00:00.000Z")
            .expect("project should be archived");

        assert!(project.is_archived());
        assert_eq!(
            project.archived_at(),
            Some("2026-07-19T13:00:00.000Z")
        );

        project
            .restore("2026-07-19T14:00:00.000Z")
            .expect("project should be restored");

        assert_eq!(project.status(), ProjectStatus::Active);
        assert_eq!(project.archived_at(), None);
    }

    #[test]
    fn soft_deleted_projects_are_immutable() {
        let mut project = project();

        project
            .soft_delete("2026-07-19T13:00:00.000Z")
            .expect("project should be deleted");

        assert!(project.is_deleted());

        assert_eq!(
            project.pause("2026-07-19T14:00:00.000Z"),
            Err(ProjectError::ProjectDeleted)
        );

        assert_eq!(
            project.update(
                ProjectChanges {
                    name: Some("Changed".to_owned()),
                    ..ProjectChanges::default()
                },
                "2026-07-19T14:00:00.000Z",
            ),
            Err(ProjectError::ProjectDeleted)
        );
    }

    #[test]
    fn parses_persistence_values_for_status_and_type() {
        assert_eq!(
            "paused".parse::<ProjectStatus>(),
            Ok(ProjectStatus::Paused)
        );

        assert_eq!(
            "software".parse::<ProjectType>(),
            Ok(ProjectType::Software)
        );

        assert_eq!(
            "unknown".parse::<ProjectStatus>(),
            Err(ProjectError::InvalidStatus("unknown".to_owned()))
        );
    }

    #[test]
    fn serializes_project_status_as_snake_case() {
        let json = serde_json::to_string(&ProjectStatus::Archived)
            .expect("status should serialize");

        assert_eq!(json, r#""archived""#);
    }

    #[test]
    fn deserializes_project_status_from_snake_case() {
        let status: ProjectStatus =
            serde_json::from_str(r#""completed""#).expect("status should deserialize");

        assert_eq!(status, ProjectStatus::Completed);
    }

    #[test]
    fn serializes_project_type_as_snake_case() {
        let json =
            serde_json::to_string(&ProjectType::Administrative)
                .expect("project type should serialize");

        assert_eq!(json, r#""administrative""#);
    }

    #[test]
    fn deserializes_project_type_from_snake_case() {
        let project_type: ProjectType =
            serde_json::from_str(r#""software""#)
                .expect("project type should deserialize");

        assert_eq!(project_type, ProjectType::Software);
    }
}

