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
use crate::{component::*, resources::*, systems::*, ui::system::*};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};
const EXTRA_OPTIONS: usize = 1;
pub fn menu(rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, dir: &mut Directions) {
    loop {
        println!("{}", ansi::RESET); //Resets the coloring
        println!("0. End turn; wait a tick");
        println!("{}", sys.display(EXTRA_OPTIONS)); //Displays options
        let len = sys.len() + EXTRA_OPTIONS;
        let response = get_from_input_valid("", "Please enter a valid input.", |x| x < &len); //Gets input
        match response {
            0 => sys.tick(rss, cmp, dir), //Ticks
            _ => system_menu(rss, cmp, sys, SystemID::new(response - EXTRA_OPTIONS), dir), /* Enters system menu */
        };
    } //As long as we can...
} //Basic menu function
