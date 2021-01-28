#[allow(dead_code)]
pub mod ansi;
pub mod component;
pub mod condition;
pub mod defaults;
pub mod from_str;
pub mod instr;
pub mod io;
pub mod location;
pub mod object;
pub mod quickie;
pub mod rand;
pub mod recipe;
pub mod resources;
pub mod system;
pub mod config;
pub mod clipboard;
pub mod info;

use from_str::{BooleanDefNo, InBounds};

use crate::{component::*, resources::*, systems::*, ui::system::*};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};

use self::{clipboard::Clipboard, config::Config, from_str::MenuRes};

pub fn menu(rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, dir: &mut Directions, cfg: &mut Config) {
    loop {
        println!("{}", ansi::RESET); //Resets the coloring
        let mut ctx = cfg.generate_context();
        let mut dis = cfg.generate_display();
        cfg.update_context(Config::QUIT, Some("quit the game".to_string()), &mut ctx, &mut dis);
        cfg.update_context(Config::NEW, None, &mut ctx, &mut dis);
        cfg.update_context(Config::DELETE, None, &mut ctx, &mut dis);
        cfg.update_context(Config::COPY, None, &mut ctx, &mut dis);
        cfg.update_context(Config::PASTE, None, &mut ctx, &mut dis);
        println!("{}", cfg.display(&ctx, &dis));
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
            MenuRes::Copy => {
                println!("You can't copy here!"); //Nothing to copy here
            }
            MenuRes::Paste => match cfg.cpb {
                Clipboard::SystemID(val) => {
                    system_menu(rss, cmp, sys, val, dir, cfg);
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
