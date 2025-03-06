#[tauri::command]
pub fn trace(message: String) {
    spdlog::trace!("{}", message);
}

#[tauri::command]
pub fn debug(message: String) {
    spdlog::debug!("{}", message);
}

#[tauri::command]
pub fn info(message: String) {
    spdlog::info!("{}", message);
}

#[tauri::command]
pub fn warn(message: String) {
    spdlog::warn!("{}", message);
}

#[tauri::command]
pub fn error(message: String) {
    spdlog::error!("{}", message);
}
