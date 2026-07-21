use tauri::State;

use crate::{
    application::database_diagnostics::{DatabaseDiagnostics, DatabaseDiagnosticsService},
    infrastructure::persistence::DatabaseManager,
};

#[tauri::command]
pub fn get_database_diagnostics(
    database_manager: State<'_, DatabaseManager>,
) -> Result<DatabaseDiagnostics, String> {
    DatabaseDiagnosticsService::execute(&database_manager).map_err(|error| error.to_string())
}
