use std::error::Error;
use std::fmt::{Display, Formatter};

use super::project::Project;

/// Domain-facing persistence contract for [`Project`].
///
/// Implementations live in the infrastructure layer. The domain and application
/// layers depend only on this abstraction and must not know about SQLite,
/// transactions, rows, or SQL statements.
pub trait ProjectRepository: Send + Sync {
    /// Returns a non-deleted project by its identifier.
    ///
    /// Soft-deleted projects must not be returned by this method.
    fn find_by_id(&self, id: &str) -> Result<Option<Project>, ProjectRepositoryError>;

    /// Returns every non-deleted, non-archived project.
    ///
    /// The infrastructure implementation is responsible for applying a stable,
    /// deterministic ordering.
    fn list_active(&self) -> Result<Vec<Project>, ProjectRepositoryError>;

    /// Returns every non-deleted archived project.
    ///
    /// The infrastructure implementation is responsible for applying a stable,
    /// deterministic ordering.
    fn list_archived(&self) -> Result<Vec<Project>, ProjectRepositoryError>;

    /// Persists a newly created project.
    ///
    /// Implementations must return [`ProjectRepositoryError::Conflict`] when a
    /// project with the same identifier already exists.
    fn insert(&self, project: &Project) -> Result<(), ProjectRepositoryError>;

    /// Persists the current state of an existing project.
    ///
    /// This operation is also used for lifecycle changes and soft deletion.
    /// Implementations must return [`ProjectRepositoryError::NotFound`] when no
    /// stored project has the supplied identifier.
    fn update(&self, project: &Project) -> Result<(), ProjectRepositoryError>;
}

/// Stable error vocabulary exposed by project repositories.
///
/// Infrastructure-specific errors should be logged at the infrastructure
/// boundary and converted to one of these variants before crossing into the
/// application layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectRepositoryError {
    NotFound { id: String },
    Conflict { id: String },
    Unavailable,
    CorruptedData { details: String },
    Unexpected { details: String },
}

impl ProjectRepositoryError {
    pub fn not_found(id: impl Into<String>) -> Self {
        Self::NotFound { id: id.into() }
    }

    pub fn conflict(id: impl Into<String>) -> Self {
        Self::Conflict { id: id.into() }
    }

    pub const fn unavailable() -> Self {
        Self::Unavailable
    }

    pub fn corrupted_data(details: impl Into<String>) -> Self {
        Self::CorruptedData {
            details: details.into(),
        }
    }

    pub fn unexpected(details: impl Into<String>) -> Self {
        Self::Unexpected {
            details: details.into(),
        }
    }
}

impl Display for ProjectRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { id } => write!(formatter, "project `{id}` was not found"),
            Self::Conflict { id } => {
                write!(formatter, "project `{id}` already exists")
            }
            Self::Unavailable => formatter.write_str("project storage is unavailable"),
            Self::CorruptedData { details } => {
                write!(formatter, "stored project data is invalid: {details}")
            }
            Self::Unexpected { details } => {
                write!(formatter, "unexpected project storage error: {details}")
            }
        }
    }
}

impl Error for ProjectRepositoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_errors_have_safe_messages() {
        let cases = [
            (
                ProjectRepositoryError::not_found("project-id"),
                "project `project-id` was not found",
            ),
            (
                ProjectRepositoryError::conflict("project-id"),
                "project `project-id` already exists",
            ),
            (
                ProjectRepositoryError::unavailable(),
                "project storage is unavailable",
            ),
            (
                ProjectRepositoryError::corrupted_data("invalid status"),
                "stored project data is invalid: invalid status",
            ),
            (
                ProjectRepositoryError::unexpected("write failed"),
                "unexpected project storage error: write failed",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn helper_constructors_preserve_context() {
        assert_eq!(
            ProjectRepositoryError::not_found("abc"),
            ProjectRepositoryError::NotFound {
                id: "abc".to_owned(),
            }
        );

        assert_eq!(
            ProjectRepositoryError::conflict("abc"),
            ProjectRepositoryError::Conflict {
                id: "abc".to_owned(),
            }
        );
    }
}
