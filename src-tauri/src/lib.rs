pub mod client;
pub mod core;
pub mod server;

use chrono::Local;
use tauri_plugin_log::{Target, TargetKind};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    // devtools
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default()
        .setup(|_app| Ok(()))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                // Perform allocation-free log formatting
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[[{date}]-[{level}]-[{target}]] {message}",
                        date = Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        level = record.level(),
                        target = record.target(),
                        message = message
                    ))
                })
                // Add blanket level filter -
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_valtio::init())
        .invoke_handler(tauri::generate_handler![greet]);

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    let app =
        builder.build(tauri::generate_context!()).expect("error while running tauri application");

    app.run(|_, _e| {});
}
