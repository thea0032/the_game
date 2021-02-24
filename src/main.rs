#![allow(clippy::ptr_arg)]

use std::fs;

use component::Components;
use file::file_object::FileObject;
use file::FilePresets;
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
mod save;
#[allow(dead_code)]
mod shape;
mod system;
mod systems;
mod ui;
mod constants;
pub fn main() {
    let presets: FilePresets = FilePresets::new("assets\\".to_string());
    let mut package = if let Some(val) = ui::init::start_game(&presets) {
        val
    } else {
        
        return;
    };
    let rss = package.unpackage_rss().expect("Safe unwrap");
    let mut cmp = package.unpackage_cmp().expect("Safe unwrap");
    let mut sys = package.unpackage_sys().expect("Safe unwrap");
    let mut dir = package.unpackage_dir().expect("Safe unwrap");
    let mut cfg = Config::new(&presets);
    ui::menu(&rss, &mut cmp, &mut sys, &mut dir, &mut cfg);
    let filename = file::get_file(&(presets.path() + "saves\\"));
    if let Ok(filename) = filename {
        while !save::save_game(&filename, &rss, &cmp, &sys, &dir) && get_raw::<bool>("Save failed! Do you want to keep trying to save?") {}
    } else if let Err(val) = filename {
        println!("ERROR: {}", val);
    }
}
