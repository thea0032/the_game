use info::information;
use object_id::ObjectID;

use crate::{component::*, object::template::Template, resources::*, systems::*, ui::{object::*, *}};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};

use super::object::get_object;
pub fn system_menu(rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, system: SystemID, dir: &mut Directions, cfg: &mut Config) {
    loop {
        println!("Viewing {}", sys.get_s_name(system));
        let mut ctx = cfg.generate_context();
        let mut dis = cfg.generate_display();
        cfg.update_context(Config::QUIT, Some("exit to systems menu".to_string()), &mut ctx, &mut dis);
        cfg.update_context(Config::DELETE, None, &mut ctx, &mut dis);
        cfg.update_context(Config::NEW, Some("new object".to_string()), &mut ctx, &mut dis);
        println!("{}", cfg.display(ctx, dis));
        println!("{}", sys.get_s_stat(system).display(sys.get_o_names(), sys));
        print!("{}", ansi::RESET); //Print statements are self-explanatory
        let response: MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x: &MenuRes| {
            x.in_bounds(&sys.get_s_stat(system).len())
        }); //Gets your response
        match response {
            MenuRes::Tick => sys.tick(rss, cmp, dir), //If it's zero, we tick
            MenuRes::Exit => break,                   //If it's 1, goes back to the system menu
            MenuRes::New => {make_object(rss, cmp, sys, dir, system, cfg);},
            MenuRes::Enter(val) => {
                let actual_id = sys.get_s(system).get_objs()[val];
                object_menu(rss, cmp, sys, actual_id, dir, cfg)
            }
            MenuRes::Copy => {
                cfg.cpb = Clipboard::SystemID(system);
                wait_for_input(&format!("Copied system {} to the clipboard!", sys.get_s_name(system)), cfg);
            }
            MenuRes::Paste => if let Clipboard::Template(val) = &cfg.cpb.clone() {
                paste_object(rss, cmp, sys, system, dir, cfg, val);
            } else {
                wait_for_input(&format!("{}You cannot paste that here!", ansi::RED), cfg);
            },
            MenuRes::Del => {

            },
            MenuRes::Info => information(rss, cmp, cfg)
        };
    }
}
pub fn paste_object(rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, system:SystemID, dir: &mut Directions, cfg: &mut Config, t:&Template, ) {
    println!("Enter the source object:");
    let o = get_object(sys, system, cfg);
    if let Some(source) = o{
        println!("Creating the destination object...");
        let val = sys.add_o(rss, cmp, dir, get_str("What do you want to call your object?", cfg), *sys.get_o_stat(source).get_location_stat(), system);
        println!("Creation done!");
        if t.install(val, sys){
            println!("Template installed!");
        } else {
            println!("Trying to install from other origin...");
            if t.grab(source, val, sys){
                println!("Successfully installed template!");
            } else {
                println!("Failed to install template!");
            }
        }
    }
}
pub fn select_object_filtered(sys: &Systems, id: SystemID, filter: Vec<bool>, cfg: &mut Config) -> ObjectID {
    println!("{}", sys.get_s_stat(id).display_filtered(0, &filter, sys.get_o_names()));
    let len = filter.iter().filter(|x| **x).count();
    let input: usize = get_from_input_valid("Enter the object you want: ", "Please enter a valid id", cfg, |x| x < &len);
    sys.get_s_stat(id).get_objs()[crate::extra_bits::filter(input, &filter)]
}
pub fn select_object_docked(sys: &Systems, id: ObjectID, cfg: &mut Config) -> ObjectID {
    let curr_system_id = sys.get_o_sys(id);
    let curr_location = *sys.get_o_stat(id).get_location_stat();
    let filter: Vec<bool> = sys
        .get_os(sys.get_s_stat(curr_system_id).get_objs())
        .iter()
        .map(|x| x.get_location_stat().eq(&curr_location))
        .collect();
    select_object_filtered(sys, curr_system_id, filter, cfg)
}
pub fn get_system(sys: &Systems, cfg: &mut Config) -> SystemID {
    println!("{}", sys.display());
    SystemID::new(get_from_input_valid(
        "Enter the system you want",
        "Please enter a valid number",
        cfg,
        |x| x < &sys.len(),
    ))
}