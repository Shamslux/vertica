use tauri::State;

use crate::{
    application::settings::{SettingsService, UpdateSettingsInput},
    domain::settings::ApplicationSettings,
    infrastructure::persistence::{DatabaseManager, SqliteSettingsRepository},
};

#[tauri::command]
pub fn get_settings(
    database_manager: State<'_, DatabaseManager>,
) -> Result<ApplicationSettings, String> {
    let repository = SqliteSettingsRepository::new(database_manager.inner());

    let service = SettingsService::new(&repository);

    service.get().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_settings(
    input: UpdateSettingsInput,
    database_manager: State<'_, DatabaseManager>,
) -> Result<ApplicationSettings, String> {
    let repository = SqliteSettingsRepository::new(database_manager.inner());

    let service = SettingsService::new(&repository);

    service.update(input).map_err(|error| error.to_string())
}
