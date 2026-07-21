use serde::Deserialize;

use crate::domain::{
    settings::{ApplicationSettings, ThemePreference, UpdateApplicationSettings},
    settings_repository::{SettingsRepository, SettingsRepositoryError},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsInput {
    pub theme: ThemePreference,
    pub language: String,
}

pub struct SettingsService<'a, TRepository>
where
    TRepository: SettingsRepository,
{
    repository: &'a TRepository,
}

impl<'a, TRepository> SettingsService<'a, TRepository>
where
    TRepository: SettingsRepository,
{
    pub const fn new(repository: &'a TRepository) -> Self {
        Self { repository }
    }

    pub fn get(&self) -> Result<ApplicationSettings, SettingsRepositoryError> {
        self.repository.get()
    }

    pub fn update(
        &self,
        input: UpdateSettingsInput,
    ) -> Result<ApplicationSettings, SettingsRepositoryError> {
        let settings = UpdateApplicationSettings::new(input.theme, input.language)?;

        self.repository.update(&settings)
    }
}
