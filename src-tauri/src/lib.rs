use tauri::Manager;

pub mod api;
pub mod client;
pub mod config;
pub mod constant;
pub mod core;
pub mod server;

#[macro_use]
extern crate rust_i18n;
// 初始化国际化
i18n!("locales", fallback = "zh");

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[allow(deprecated)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            core::tray::Tray::instance().init(app.handle())?;
            config::log::init(app.path().app_log_dir()?)?;
            Ok(())
        });

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }
    builder
        .invoke_handler(tauri::generate_handler![
            greet,
            api::sys::get_sys_locale
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
