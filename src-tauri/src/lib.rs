mod application;
mod commands;
mod domain;
mod infrastructure;

use commands::{
    application_info::get_application_info,
    database_diagnostics::get_database_diagnostics,
    project::{
        archive_project, create_project, get_project, list_active_projects,
        list_archived_projects, restore_project, update_project,
    },
    settings::{get_settings, update_settings},
};
use infrastructure::persistence::{DatabaseManager, DatabasePathResolver};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let database_path = DatabasePathResolver::resolve(app.handle())?;

            let database_manager = DatabaseManager::open(database_path, env!("CARGO_PKG_VERSION"))?;

            app.manage(database_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_application_info,
            get_database_diagnostics,
            get_settings,
            update_settings,
            create_project,
            get_project,
            list_active_projects,
            list_archived_projects,
            update_project,
            archive_project,
            restore_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
