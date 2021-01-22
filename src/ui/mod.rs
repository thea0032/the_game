#[allow(dead_code)]
pub mod ansi;
pub mod component;
pub mod condition;
pub mod instr;
pub mod io;
pub mod location;
pub mod object;
pub mod quickie;
pub mod rand;
pub mod recipe;
pub mod resources;
pub mod system;
pub mod from_str;
pub mod defaults;
use std::str::FromStr;
use from_str::{BooleanDefNo, FromString, InBounds};

use crate::{component::*, resources::*, systems::*, ui::system::*};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};

pub fn menu(rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, dir: &mut Directions, cfg:&mut Config) {
    loop {
        println!("{}", ansi::RESET); //Resets the coloring
        println!("{}. End turn; wait a tick", cfg.tick());
        println!("{}. Quit (the game)", cfg.quit());
        println!("{}", sys.display()); //Displays options
        let response:MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x:&MenuRes| x.in_bounds(&sys.len())); //Gets input
        match response {
            MenuRes::Tick => sys.tick(rss, cmp, dir), //Ticks
            MenuRes::System(val) => system_menu(rss, cmp, sys, SystemID::new(val), dir, cfg), /* Enters system menu */
            MenuRes::Exit => if get_from_input::<BooleanDefNo>("Are you sure? y/N", "You shouldn't see this message at all.", cfg).b{
                    break;
                }
            ,
        };
    } //As long as we can...
} //Basic menu function
pub enum MenuRes{
    System(usize),
    Tick,
    Exit,
}
impl FromString for MenuRes{
    fn from_string(s: &str, cfg:&mut Config) -> Option<Self> {
        if s == cfg.tick() {
            return Some(MenuRes::Tick);
        } else if s == cfg.quit() {
            return Some(MenuRes::Exit);
        } else if let Ok(val) = usize::from_str(s){
            return Some(MenuRes::System(val));
        } else {
            return None;
        }
    }
    fn from_string_s(s:&str) -> Option<Self>
    where Self:Sized {
        None
    }
}
impl InBounds for MenuRes{
    fn in_bounds(&self, bounds:&usize) -> bool {
        if let MenuRes::System(val) = self{
            return val < bounds;
        } else {
            return true;
        }
    }
}