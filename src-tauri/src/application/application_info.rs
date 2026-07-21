use crate::domain::application_info::{
    ApplicationEnvironment, ApplicationInfo, ApplicationStatus, DatabaseInfo, DatabaseStatus,
};

pub struct ApplicationInfoService;

impl ApplicationInfoService {
    pub fn get() -> ApplicationInfo {
        ApplicationInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: current_environment(),
            status: ApplicationStatus::Ready,
            database: DatabaseInfo {
                status: DatabaseStatus::NotConfigured,
                version: None,
            },
        }
    }
}

fn current_environment() -> ApplicationEnvironment {
    if cfg!(debug_assertions) {
        ApplicationEnvironment::Development
    } else {
        ApplicationEnvironment::Production
    }
}
