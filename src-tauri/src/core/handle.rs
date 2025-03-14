use parking_lot::RwLock;
use spdlog::debug;
use std::sync::{Arc, OnceLock};
use tauri::{AppHandle, Manager, WebviewWindow};

use crate::constant;

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
}

impl Handle {
    pub fn instance() -> &'static Handle {
        static HANDLE: OnceLock<Handle> = OnceLock::new();

        HANDLE
            .get_or_init(|| Handle { app_handle: Arc::new(RwLock::new(None)) })
    }

    pub fn init(&self, app_handle: &AppHandle) {
        let mut handle = self.app_handle.write();
        *handle = Some(app_handle.clone());
    }

    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }

    pub fn get_window(&self) -> Option<WebviewWindow> {
        let app_handle = self.app_handle().unwrap();
        let window: Option<WebviewWindow> =
            app_handle.get_webview_window(constant::MAIN_WINDOW_LABEL);
        if window.is_none() {
            debug!("main window not found");
        }
        window
    }
}
