use super::config::Config;

pub const MENU: usize = 0;
pub const SYSTEM_MENU: usize = 1;
pub const OBJECT_MENU: usize = 2;
pub const SELECT: usize = 3;
pub const INSTRS_MENU:usize = 4;
pub const QUEUE_MENU:usize = 5;

pub fn init(cfg: &mut Config) {
    cfg.add_context(super::menu_context); // 0
    cfg.add_context(super::system::system_menu_context); // 1
    cfg.add_context(super::object::object_menu_context); // 2
    cfg.add_context(super::select::menu_context); // 3
    cfg.add_context(super::instrs::instrs_menu_context); // 4
    cfg.add_context(super::instrs::queue_context); // 5
}
