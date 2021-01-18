
#![allow(clippy::ptr_arg)]

use component::Components;
use init::init_file;
use instr::Directions;
use resources::ResourceDict;
use systems::Systems;
mod component;
mod constants;
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
    init_file();
    let rss: ResourceDict = init::rss();
    let mut cmp: Components = init::cmp(&rss);
    let mut dir: Directions = init::dir();
    let mut sys: Systems = init::sys(&rss, &mut cmp, &mut dir);
    ui::menu(&rss, &mut cmp, &mut sys, &mut dir);
}
