use std::sync::OnceLock;

#[derive(Debug, Default, Clone)]
pub struct Tray {}

impl Tray {
    /// 获取单例
    pub fn instance() -> &'static Self {
        static ONCE: OnceLock<Tray> = OnceLock::new();
        ONCE.get_or_init(Tray::new)
    }

    /// 创建一个新的实例
    fn new() -> Self {
        Self {}
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
