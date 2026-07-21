use std::{error::Error, fmt};

use super::settings::{ApplicationSettings, UpdateApplicationSettings};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsRepositoryError {
    message: String,
}

impl SettingsRepositoryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SettingsRepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for SettingsRepositoryError {}

pub trait SettingsRepository {
    fn get(&self) -> Result<ApplicationSettings, SettingsRepositoryError>;

    fn update(
        &self,
        settings: &UpdateApplicationSettings,
    ) -> Result<ApplicationSettings, SettingsRepositoryError>;
}
