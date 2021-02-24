#[allow(dead_code)]
pub mod ansi;
#[allow(dead_code)]
mod clipboard;
mod component;
mod condition;
pub mod config;
mod context;
mod defaults;
mod from_str;
mod info;
mod instr;
mod instrs;
pub mod io;
mod location;
mod object;
mod quickie;
pub mod rand;
mod recipe;
pub mod resources;
mod select;
mod system;

use from_str::{BooleanDefNo, InBounds};

use crate::{component::*, resources::*, systems::*, ui::system::*};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};

use self::{clipboard::Clipboard, config::Config, from_str::MenuRes};

pub fn menu(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, dir: &mut Directions, cfg: &mut Config) {
    loop {
        println!("{}", ansi::RESET); //Resets the coloring
        println!("{}", cfg.display(context::MENU));
        println!("{}", sys.display()); //Displays options
        let response: MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x: &MenuRes| x.in_bounds(&sys.len())); //Gets input
        match response {
            MenuRes::Tick => sys.tick(rss, cmp, dir),                                        //Ticks
            MenuRes::Enter(val) => system_menu(rss, cmp, sys, SystemID::new(val), dir, cfg), /* Enters system menu */
            MenuRes::Exit => {
                if get_from_input::<BooleanDefNo>("Are you sure? y/N", "You shouldn't see this message at all.", cfg).b {
                    //If you're sure...
                    break; //Exits!
                }
            }
            MenuRes::Copy(_) => {
                wait_for_input(&format!("{}You can't copy here!", ansi::RED), cfg); //Nothing to copy here
            }
            MenuRes::Paste(val) => match cfg.clipboard(val) {
                Clipboard::SystemID(val) => {
                    system_menu(rss, cmp, sys, *val, dir, cfg);
                }
                _ => {
                    wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg);
                }
            },
            _ => {
                wait_for_input(&format!("{}Invalid response!", ansi::RED), cfg);
            }
        };
    } //As long as we can...
} //Basic menu function
pub fn menu_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context(Config::QUIT, Some("quit the game".to_string()), ctx, dis);
    cfg.update_context(Config::NEW, None, ctx, dis);
    cfg.update_context(Config::DELETE, None, ctx, dis);
    cfg.update_context(Config::COPY, None, ctx, dis);
    cfg.update_context(Config::PASTE, None, ctx, dis);
}
