use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfo {
    pub name: String,
    pub version: String,
    pub environment: ApplicationEnvironment,
    pub status: ApplicationStatus,
    pub database: DatabaseInfo,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationEnvironment {
    Development,
    Production,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStatus {
    Ready,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInfo {
    pub status: DatabaseStatus,
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseStatus {
    NotConfigured,
}
