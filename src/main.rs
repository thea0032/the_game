#![allow(clippy::ptr_arg)]

use std::io::{Read, stdin};

use component::Components;
use file::{FileObject, FilePresets};
use instr::Directions;
use systems::Systems;
use ui::config::Config;
mod component;
mod extra_bits;
mod file;
mod init;
mod instr;
mod location;
mod object;
mod resources;
#[allow(dead_code)]
mod shape;
mod system;
mod systems;
mod ui;
pub fn main() {
    let presets: FilePresets = FilePresets::new("assets\\".to_string());
    let file_res: FileObject = init::load(presets.clone(), vec!["base"]);
    let rss = init::rss(&file_res);
    let mut cmp: Components = init::cmp(&rss, &file_res);
    let mut dir: Directions = init::dir();
    let mut sys: Systems = init::sys_new(&rss, &mut cmp, &mut dir, &file_res);
    let mut cfg: Config = init::config(presets);
    ui::menu(&rss, &mut cmp, &mut sys, &mut dir, &mut cfg);
}
