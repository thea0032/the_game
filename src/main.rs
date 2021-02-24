#![allow(clippy::ptr_arg)]

use std::fs;

use component::Components;
use file::{FilePresets};
use file::file_object::FileObject;
use instr::Directions;
use systems::Systems;
use ui::config::Config;
use ui::io::get_raw;
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
mod save;
pub fn main() {
    let presets: FilePresets = FilePresets::new("assets\\".to_string());
    let file_res: FileObject = init::load(presets.clone(), vec!["base"]);
    let rss = init::rss(&file_res);
    let mut cmp: Components = init::cmp(&rss, &file_res);
    let mut dir: Directions = init::dir();
    let mut sys: Systems = init::sys(&rss, &mut cmp, &mut dir, &file_res);
    let mut cfg: Config = init::config(presets.clone());
    ui::menu(&rss, &mut cmp, &mut sys, &mut dir, &mut cfg);
    let filename = file::get_file(&(presets.path() + "saves\\"));
    if let Ok(filename) = filename {
        while !save::save_game(&filename, &rss, &cmp, &sys, &dir) && get_raw::<bool>("Save failed! Do you want to keep trying to save?"){}
    } else if let Err(val) = filename{
        println!("ERROR: {}", val);
    }
}

