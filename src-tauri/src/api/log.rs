use tauri::Runtime;

#[tauri::command]
pub fn trace<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::trace!("{}", message);
}

#[tauri::command]
pub fn debug<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::debug!("{}", message);
}

#[tauri::command]
pub fn info<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::info!("{}", message);
}

#[tauri::command]
pub fn warn<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::warn!("{}", message);
}

#[tauri::command]
pub fn error<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::error!("{}", message);
}
