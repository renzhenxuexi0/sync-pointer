use std::sync::OnceLock;

use anyhow::Result;
use spdlog::debug;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId},
    Manager,
};

use crate::constant;

use super::handle::Handle;

#[derive(Default)]
pub struct Tray {}

impl Tray {
    /// 获取单例
    pub fn instance() -> &'static Self {
        static ONCE: OnceLock<Tray> = OnceLock::new();
        ONCE.get_or_init(|| Tray {})
    }

    /// 初始化
    pub fn init(&self) -> Result<()> {
        let app_handle = Handle::instance().app_handle().unwrap();
        let tray_icon_id = TrayIconId::new(constant::TRAY_ICON_ID);
        let tray = app_handle.tray_by_id(&tray_icon_id).unwrap();
        tray.on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                debug!("left click pressed and released");
                // in this example, let's show and focus the main window when the tray is clicked
                let app = tray.app_handle();
                if let Some(window) =
                    app.get_webview_window(constant::MAIN_WINDOW_LABEL)
                {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                debug!("unhandled event {event:?}");
            }
        });

        Ok(())
    }

    /// 更新菜单
    pub fn update_menu(&self) {
        unimplemented!()
    }

    /// 更新提示
    pub fn update_tooltip(&self) {
        unimplemented!()
    }

    /// 更新图标
    pub fn update_icon(&self) {
        unimplemented!()
    }
}
