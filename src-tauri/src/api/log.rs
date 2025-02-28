use tauri::{Runtime, command};

#[command]
pub fn trace<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::trace!("{}", message);
}

#[command]
pub fn debug<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::debug!("{}", message);
}

#[command]
pub fn info<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::info!("{}", message);
}

#[command]
pub fn warn<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::warn!("{}", message);
}

#[command]
pub fn error<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::error!("{}", message);
}
