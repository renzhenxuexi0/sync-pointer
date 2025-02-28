use tauri::{Runtime, command};

#[command]
pub async fn local_ip<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<String, String> {
    local_ip_address::local_ip()
        .map(|addr| addr.to_string())
        .map_err(|e| e.to_string())
}
