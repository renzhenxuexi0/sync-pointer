use tauri::{command, Runtime};

#[command]
pub async fn trace<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::trace!("{}", message);
}

#[command]
pub async fn debug<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::debug!("{}", message);
}

#[command]
pub async fn info<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::info!("{}", message);
}

#[command]
pub async fn warn<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::warn!("{}", message);
}

#[command]
pub async fn error<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    message: String,
) {
    spdlog::error!("{}", message);
}
