use std::{fs, path::PathBuf};

use tauri::{AppHandle, Manager};

use super::{PersistenceError, PersistenceResult};

const PROFILES_DIRECTORY: &str = "profiles";
const DEFAULT_PROFILE_ID: &str = "default";
const DATABASE_FILE_NAME: &str = "vertica.db";

pub struct DatabasePathResolver;

impl DatabasePathResolver {
    pub fn resolve(app_handle: &AppHandle) -> PersistenceResult<PathBuf> {
        let database_directory = app_handle
            .path()
            .app_data_dir()?
            .join(PROFILES_DIRECTORY)
            .join(DEFAULT_PROFILE_ID);

        fs::create_dir_all(&database_directory).map_err(|source| {
            PersistenceError::DirectoryCreation {
                path: database_directory.clone(),
                source,
            }
        })?;

        Ok(database_directory.join(DATABASE_FILE_NAME))
    }
}
