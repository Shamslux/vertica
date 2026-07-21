use crate::{
    application::application_info::ApplicationInfoService,
    domain::application_info::ApplicationInfo,
};

#[tauri::command]
pub fn get_application_info() -> Result<ApplicationInfo, String> {
    Ok(ApplicationInfoService::get())
}
