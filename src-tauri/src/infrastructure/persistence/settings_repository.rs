use rusqlite::{params, types::Type, Connection};

use crate::domain::{
    settings::{ApplicationSettings, ThemePreference, UpdateApplicationSettings},
    settings_repository::{SettingsRepository, SettingsRepositoryError},
};

use super::{DatabaseManager, PersistenceError, PersistenceResult};

pub struct SqliteSettingsRepository<'a> {
    database_manager: &'a DatabaseManager,
}

impl<'a> SqliteSettingsRepository<'a> {
    pub const fn new(database_manager: &'a DatabaseManager) -> Self {
        Self { database_manager }
    }
}

impl SettingsRepository for SqliteSettingsRepository<'_> {
    fn get(&self) -> Result<ApplicationSettings, SettingsRepositoryError> {
        self.database_manager
            .with_connection(read_settings)
            .map_err(map_persistence_error)
    }

    fn update(
        &self,
        settings: &UpdateApplicationSettings,
    ) -> Result<ApplicationSettings, SettingsRepositoryError> {
        self.database_manager
            .with_connection(|connection| {
                let affected_rows = connection.execute(
                    "UPDATE application_settings
                     SET theme = ?1,
                         language = ?2,
                         updated_at = strftime(
                             '%Y-%m-%dT%H:%M:%fZ',
                             'now'
                         )
                     WHERE singleton_id = 1",
                    params![settings.theme.as_str(), settings.language.as_str(),],
                )?;

                if affected_rows != 1 {
                    return Err(PersistenceError::Database(
                        rusqlite::Error::QueryReturnedNoRows,
                    ));
                }

                read_settings(connection)
            })
            .map_err(map_persistence_error)
    }
}

fn read_settings(connection: &Connection) -> PersistenceResult<ApplicationSettings> {
    connection
        .query_row(
            "SELECT theme, language, updated_at
             FROM application_settings
             WHERE singleton_id = 1",
            [],
            |row| {
                let theme: String = row.get(0)?;
                let language: String = row.get(1)?;
                let updated_at: String = row.get(2)?;

                let theme = ThemePreference::try_from(theme.as_str()).map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(error))
                })?;

                Ok(ApplicationSettings {
                    theme,
                    language,
                    updated_at,
                })
            },
        )
        .map_err(PersistenceError::from)
}

fn map_persistence_error(error: PersistenceError) -> SettingsRepositoryError {
    SettingsRepositoryError::new(error.to_string())
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        settings::{ThemePreference, UpdateApplicationSettings},
        settings_repository::{SettingsRepository, SettingsRepositoryError},
    };
    use crate::infrastructure::persistence::DatabaseManager;

    use super::SqliteSettingsRepository;

    #[test]
    fn reads_and_updates_settings() -> Result<(), SettingsRepositoryError> {
        let database_manager =
            DatabaseManager::open_in_memory("test").expect("in-memory database should initialize");

        let repository = SqliteSettingsRepository::new(&database_manager);

        let initial = repository.get()?;

        assert_eq!(initial.theme, ThemePreference::System);
        assert_eq!(initial.language, "en");

        let update = UpdateApplicationSettings::new(ThemePreference::Dark, "pt-BR")?;

        let updated = repository.update(&update)?;

        assert_eq!(updated.theme, ThemePreference::Dark);
        assert_eq!(updated.language, "pt-BR");

        Ok(())
    }
}
