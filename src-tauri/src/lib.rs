use tauri::Manager;

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
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
