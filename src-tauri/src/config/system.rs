use serde::{Deserialize, Serialize};
use tauri_plugin_valtio::ManagerExt as _;

use crate::{constant, core};

const KEY: &str = "system";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "zh-CN")]
    ZhCN,
    #[serde(rename = "en-US")]
    EnUS,
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    id: String,
    theme: Theme,
    locale: Locale,
    auto_start: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            theme: Theme::Auto,
            locale: Locale::Auto,
            auto_start: false,
        }
    }
}

impl SystemSettings {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn locale(&self) -> Locale {
        self.locale.clone()
    }

    pub fn auto_start(&self) -> bool {
        self.auto_start
    }
}

// 获取配置
pub fn config() -> Option<SystemSettings> {
    core::handle::Handle::instance()
        .app_handle()
        .and_then(|handle| handle.valtio().get(constant::STORE_ID, KEY))
        .and_then(|value| serde_json::from_value(value).ok())
}
