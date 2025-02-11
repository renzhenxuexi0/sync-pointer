use log::info;
use sys_locale::get_locale;
use tauri::Runtime;

#[tauri::command]
pub async fn get_sys_locale<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<String, String> {
    let locale = get_locale().unwrap_or_else(|| String::from("zh-CN"));
    info!("get sys locale: {}", locale);
    Ok(locale)
}
