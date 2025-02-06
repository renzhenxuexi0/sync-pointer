use tauri::Runtime;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
async fn greet<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<String, String> {
    Ok("Hello from Rust!".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    // Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_valtio::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
