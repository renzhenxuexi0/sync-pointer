use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use spdlog::info;
use tauri::{
    AppHandle, Manager, Wry,
    menu::{Menu, MenuBuilder, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::{
        MouseButton, MouseButtonState, TrayIcon, TrayIconEvent, TrayIconId,
    },
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
        let app_handle_opt = Handle::instance().app_handle();
        if app_handle_opt.is_none() {
            return Err(anyhow::anyhow!("app handle not found"));
        }
        let app_handle = app_handle_opt.unwrap();
        let tray_icon_id = TrayIconId::new(constant::TRAY_ICON_ID);
        let tray_opt: Option<TrayIcon> = app_handle.tray_by_id(&tray_icon_id);
        if tray_opt.is_none() {
            return Err(anyhow::anyhow!("tray icon not found"));
        }
        let tray = tray_opt.unwrap();
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
        let menu = Self::create_menu(&app_handle)?;
        tray.set_menu(Some(menu))?;
        tray.on_menu_event(Self::on_menu_event);
        Ok(())
    }

    /// 创建菜单
    fn create_menu(app_handle: &AppHandle) -> Result<Menu<Wry>> {
        // 退出按钮
        let quit = &MenuItem::with_id(
            app_handle,
            constant::MENU_ITEM_ID_QUIT,
            t!("menu.quit"),
            true,
            Some("CmdOrControl+Q"),
        )?;

        // 重启按钮
        let restart = &MenuItem::with_id(
            app_handle,
            constant::MENU_ITEM_ID_RESTART,
            t!("menu.restart"),
            true,
            Some("CmdOrControl+R"),
        )?;

        // 屏幕布局设置
        let screen_layout = &MenuItem::with_id(
            app_handle,
            constant::MENU_ITEM_ID_SCREEN_LAYOUT,
            t!("menu.screen-layout"),
            true,
            None::<&str>,
        )?;

        // 设置按钮
        let settings = &MenuItem::with_id(
            app_handle,
            constant::MENU_ITEM_ID_SETTINGS,
            t!("menu.settings"),
            true,
            None::<&str>,
        )?;

        // 分割线
        let separator = &PredefinedMenuItem::separator(app_handle)?;

        MenuBuilder::new(app_handle)
            .items(&[screen_layout, settings, separator, restart, quit])
            .build()
            .map_err(|e| anyhow!("Create Menu Error {:?}", e))
    }

    /// 配置菜单按键事件
    fn on_menu_event(app_handle: &AppHandle, event: MenuEvent) {
        match event.id().as_ref() {
            constant::MENU_ITEM_ID_QUIT => {
                // 退出
                app_handle.exit(0);
            }
            constant::MENU_ITEM_ID_RESTART => {
                // 重启
                app_handle.restart();
            }
            constant::MENU_ITEM_ID_SCREEN_LAYOUT => {
                // 屏幕布局设置
                todo!();
            }
            constant::MENU_ITEM_ID_SETTINGS => {
                // 设置
                todo!();
            }
            _ => {}
        }
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
