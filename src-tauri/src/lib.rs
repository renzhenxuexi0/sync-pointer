use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::MacosLauncher;

pub mod api;
pub mod config;
pub mod constant;
pub mod core;
pub mod service;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "zh");

#[allow(deprecated)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 当重复打开应用时，激活已有窗口
            show_window(app);
        }))
        .setup(|app| {
            config::log::init(app.path().app_log_dir()?)?;
            core::handle::Handle::instance().init(app.handle());
            core::tray::Tray::instance().init()?;
            Ok(())
        });

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }
    builder
        .invoke_handler(tauri::generate_handler![
            // log
            api::log::trace,
            api::log::debug,
            api::log::info,
            api::log::warn,
            api::log::error,
            // mdns
            api::mdns::start_mdns_server,
            api::mdns::stop_mdns_server,
            api::mdns::start_mdns_discovery,
            api::mdns::stop_mdns_discovery,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
