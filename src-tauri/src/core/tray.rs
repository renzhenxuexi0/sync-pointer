use std::sync::OnceLock;

use anyhow::Result;
use spdlog::info;
use tauri::{
    Manager,
    tray::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId},
};

use crate::constant;

use super::handle::Handle;

pub struct Tray {}

impl Tray {
    /// 获取单例
    pub fn instance() -> &'static Self {
        static TRAY: OnceLock<Tray> = OnceLock::new();
        TRAY.get_or_init(|| Tray {})
    }

    /// 初始化
    pub fn init(&self) -> Result<()> {
        let app_handle = Handle::instance().app_handle().unwrap();
        let tray_icon_id = TrayIconId::new(constant::TRAY_ICON_ID);
        let tray: tauri::tray::TrayIcon =
            app_handle.tray_by_id(&tray_icon_id).unwrap();
        tray.on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                info!("left click pressed and released");
                // in this example, let's show and focus the main window when the tray is clicked
                let app = tray.app_handle();
                if let Some(window) =
                    app.get_webview_window(constant::MAIN_WINDOW_LABEL)
                {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        });
        Ok(())
    }

    /// 更新菜单
    pub fn update_menu(&self) {}

    /// 更新提示
    pub fn update_tooltip(&self) {
        unimplemented!()
    }

    /// 更新图标
    pub fn update_icon(&self) {
        unimplemented!()
    }
}
