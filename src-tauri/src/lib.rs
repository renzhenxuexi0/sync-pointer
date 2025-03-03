use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_valtio::ManagerExt;

pub mod api;
pub mod config;
pub mod constant;
pub mod core;
pub mod service;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "zh-CN");

#[allow(deprecated)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default().setup(|app| {
        config::log::init(app.path().app_log_dir()?)?;
        core::handle::Handle::instance().init(app.handle());
        core::tray::Tray::instance().init()?;
        // 设置应用数据目录
        let app_data_dir = app.path().app_data_dir()?;
        app.handle().valtio().set_path(app_data_dir.join("store"))?;
        Ok(())
    });

    builder = builder
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_valtio::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 当重复打开应用时，激活已有窗口
            show_window(app);
        }));

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }
    builder
        .invoke_handler(tauri::generate_handler![
            // sys
            api::sys::local_ip,
            // log
            api::log::trace,
            api::log::debug,
            api::log::info,
            api::log::warn,
            api::log::error,
            // mdns
            api::mdns::start_mdns_client,
            api::mdns::stop_mdns_client,
            api::mdns::start_mdns_server,
            api::mdns::stop_mdns_server,
            api::mdns::update_mdns_server_info,
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
