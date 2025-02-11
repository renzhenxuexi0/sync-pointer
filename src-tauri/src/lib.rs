pub mod api;
pub mod client;
pub mod constant;
pub mod core;
pub mod server;

#[macro_use]
extern crate rust_i18n;

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
        .setup(|app| {
            core::tray::Tray::instance().init(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_valtio::init());

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    let app = builder
        .invoke_handler(tauri::generate_handler![greet, api::sys::get_sys_locale])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, _e| {});
}
