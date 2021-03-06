use info::information;
use object_id::ObjectID;

use crate::{
    component::*,
    object::template::Template,
    resources::*,
    systems::*,
    ui::{object::*, *},
};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};

use super::object::get_object;
pub fn system_menu(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, system: SystemID, dir: &mut Directions, cfg: &mut Config) {
    loop {
        println!("Viewing {}", sys.get_system_name(system));
        println!("{}", cfg.display(context::SYSTEM_MENU));
        println!("{}", sys.get_system(system).display(sys.get_object_names(), sys));
        print!("{}", ansi::RESET); //Print statements are self-explanatory
        let response: MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x: &MenuRes| {
            x.in_bounds(&sys.get_system(system).len())
        }); //Gets your response
        match response {
            MenuRes::Tick => sys.tick(rss, cmp, dir), //If it's zero, we tick
            MenuRes::Exit => break,                   //If it's 1, goes back to the system menu
            MenuRes::New => {
                make_object(rss, cmp, sys, dir, system, cfg);
            }
            MenuRes::Enter(val) => {
                let actual_id = sys.get_system_mut(system).get_objs()[val];
                object_menu(rss, cmp, sys, actual_id, dir, cfg)
            }
            MenuRes::Copy(val) => {
                *(cfg.clipboard(val)) = Clipboard::SystemID(system);
                wait_for_input(&format!("Copied system {} to the clipboard!", sys.get_system_name(system)), cfg);
            }
            MenuRes::Paste(val) => {
                if let Clipboard::Template(val) = &cfg.clipboard(val).clone() {
                    paste_object(rss, cmp, sys, system, dir, cfg, val);
                } else if let Clipboard::Object(val) = &cfg.clipboard(val) {
                    object_menu(rss, cmp, sys, *val, dir, cfg);
                } else {
                    wait_for_input(&format!("{}You cannot paste that here!", ansi::RED), cfg);
                }
            }
            MenuRes::Del => {}
            MenuRes::Info => information(rss, cmp, cfg),
        };
    }
}
pub fn system_menu_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context(Config::QUIT, Some("exit to systems menu".to_string()), ctx, dis);
    cfg.update_context(Config::DELETE, None, ctx, dis);
    cfg.update_context(Config::NEW, Some("new object".to_string()), ctx, dis);
}
pub fn paste_object(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, system: SystemID, dir: &mut Directions, cfg: &mut Config, t: &Template) {
    println!("Enter the source object:");
    let o = get_object(sys, system, cfg);
    if let Some(source) = o {
        println!("Creating the destination object...");
        let val = sys.add_object(
            rss,
            cmp,
            dir,
            get_str("What do you want to call your object?", cfg),
            *sys.get_object(source).get_location_stat(),
            system,
        );
        println!("Creation done!");
        if t.install(val, sys) {
            println!("Template installed!");
        } else {
            println!("Trying to install from other origin...");
            if t.grab(source, val, sys, rss) {
                println!("Successfully installed template!");
            } else {
                println!("Failed to install template!");
            }
        }
    }
}
pub fn select_object_filtered(sys: &Systems, id: SystemID, filter: Vec<bool>, cfg: &mut Config) -> Option<ObjectID> {
    loop {
        println!("{}", sys.get_system(id).display_filtered(0, &filter, sys.get_object_names()));
        let len = filter.iter().filter(|x| **x).count();
        let input: MenuRes = get_from_input_valid("Enter the object you want: ", "Please enter a valid id", cfg, |x: &MenuRes| {
            x.in_bounds(&len)
        });
        match input {
            MenuRes::Enter(v) => return Some(sys.get_system(id).get_objs()[crate::extra_bits::filter(v, &filter)]),
            MenuRes::Exit | MenuRes::Del => return None,
            MenuRes::Paste(val) => match &cfg.clipboard(val) {
                Clipboard::Object(val) => {
                    if filter[val.get()] {
                        return Some(*val);
                    } else {
                        wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg);
                    }
                }
                _ => {
                    wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg);
                }
            },
            _ => {
                wait_for_input(&format!("{}Please enter a valid entry", ansi::RED), cfg);
            }
        }
    }
}
pub fn select_object_docked(sys: &Systems, id: ObjectID, cfg: &mut Config) -> Option<ObjectID> {
    let curr_system_id = sys.get_objects_system(id);
    let curr_location = *sys.get_object(id).get_location_stat();
    let filter: Vec<bool> = sys
        .get_objects(sys.get_system(curr_system_id).get_objs())
        .iter()
        .map(|x| x.get_location_stat().eq(&curr_location))
        .collect();
    select_object_filtered(sys, curr_system_id, filter, cfg)
}
pub fn get_system(sys: &Systems, cfg: &mut Config) -> Option<SystemID> {
    loop {
        println!("{}", sys.display());
        let input: MenuRes = get_from_input_valid("Enter the system you want", "Please enter a valid entry", cfg, |x: &MenuRes| {
            x.in_bounds(&sys.len())
        });
        match input {
            MenuRes::Enter(v) => {
                return Some(SystemID::new(v));
            }
            MenuRes::Exit | MenuRes::Del => {
                return None;
            }
            MenuRes::Paste(val) => match &cfg.clipboard(val) {
                Clipboard::SystemID(v) => return Some(*v),
                _ => wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg),
            },
            _ => {
                wait_for_input(&format!("{}Please enter a valid entry", ansi::RED), cfg);
            }
        }
    }
}
