use component::Components;
use init::init_file;
use instr::Directions;
use resources::ResourceDict;
use systems::{Systems};

mod component;
mod object;
mod resources;
mod extra_bits;
mod init;
mod location;
mod system;
mod systems;
mod ui;
mod instr;
mod shape;
mod file;
pub fn main(){
    init_file();
    let mut rss:ResourceDict = init::rss();
    let mut cmp:Components = init::cmp(&mut rss);
    let mut dir:Directions = init::dir();
    let mut sys:Systems = init::sys(&mut rss, &mut cmp, &mut dir);
    ui::menu(&mut rss, &mut cmp, &mut sys, &mut dir);
}