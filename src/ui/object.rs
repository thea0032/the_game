use std::unimplemented;

use info::information;
use location::get_location;
use quickie::quickie;
use resources::get_resource_filtered;
use select::generic_select;

use crate::{
    component::*,
    resources::*,
    systems::*,
    ui::{component::*, system::*, *},
};
use crate::{
    instr::Directions,
    systems::{object_id::ObjectID, system_id::SystemID},
    ui::io::*,
};
pub fn make_object(
    rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, dir: &mut Directions, system: SystemID, cfg: &mut Config,
) -> Option<ObjectID> {
    let name: String = get_str("What do you want to call your new object?", cfg); //Gets the object's name from input
    let loc = get_location(cfg); //Gets the object's location from input
    if get_from_input("Are you sure?", "Please enter true or false", cfg) {
        //
        println!("Object {} created!", name);
        let res = sys.add_o(rss, cmp, dir, name, loc, system);
        wait_for_input("Press enter to continue.", cfg);
        Some(res)
    } else {
        println!("Object creation aborted.");
        wait_for_input("Press enter to continue.", cfg);
        None
    }
} //Makes an object
pub fn new_at(
    rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, system: SystemID, cfg: &mut Config, dir: &mut Directions, from: ObjectID,
) -> ObjectID {
    let name: String = get_str("What do you want to call your new object?", cfg); //Gets the object's name from input
    let loc = *sys.get_o(from).get_location_stat();
    sys.add_o(rss, cmp, dir, name, loc, system)
}
pub fn object_menu(rss: &ResourceDict, cmp: &mut Components, sys: &mut Systems, obj: ObjectID, dir: &mut Directions, cfg: &mut Config) {
    loop {
        println!("Displaying...");
        println!("Viewing {} ", sys.get_o(obj).name());
        println!("{}", sys.get_o(obj).display(rss, cmp));
        println!("Options: ");
        println!("{}", cfg.display(context::OBJECT_MENU));
        println!("0. Use a recipe ");
        println!("1. Transfer resources to another object: ");
        println!("{}", ansi::BLUE);
        println!("2. Enter instruction menu");
        println!("3. Enter quick instruction menu (all quick instructions are done every turn");
        print!("{}", ansi::RESET); //Displays options
        let len: usize = 4;
        let response: MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x: &MenuRes| x.in_bounds(&len)); //Gets response
        match response {
            MenuRes::Tick => sys.tick(rss, cmp, dir),                                           //Advance 1 tick
            MenuRes::Exit => break,                                                             //Break out of menu
            MenuRes::New => add_component(cmp, sys.get_o(obj), cfg),                            //Add component
            MenuRes::Del => remove_component(cmp, sys.get_o(obj), cfg),                         //Remove component
            MenuRes::Enter(0) => recipe::perform_recipe(cmp, sys.get_o(obj), cfg),              //Perform recipe
            MenuRes::Enter(1) => transfer(rss, sys, obj, cfg),                                  //Transfer resources
            MenuRes::Enter(2) => instrs::instrs_menu(sys, obj, cmp, rss, dir.instrs(obj), cfg), /* Enter instructions menu */
            MenuRes::Enter(3) => quickie(rss, cmp, sys, dir.quickie(obj), obj, cfg),            /* Enter quick */
            // instructions
            // menu
            MenuRes::Copy(val) => *cfg.clipboard(val) = Clipboard::Template(sys.get_o(obj).to_template(cmp, rss, "pasted template".to_string())),
            MenuRes::Paste(val) => match cfg.clipboard(val) {
                Clipboard::Template(_) => unimplemented!(),
                Clipboard::Object(_) => unimplemented!(),
                Clipboard::Instrs(val) => {
                    for line in val.get_queues() {
                        dir.instrs(obj).add(line.clone(), "pasted queue".to_string());
                    }
                    wait_for_input(&format!("{}Queue pasted!", ansi::GREEN), cfg);
                }
                Clipboard::Queue(val) => {
                    dir.instrs(obj).add(val.clone(), "pasted queue".to_string());
                    wait_for_input(&format!("{}Queue pasted!", ansi::GREEN), cfg);
                }
                Clipboard::Instr(val, del) => {
                    dir.quickie(obj).ins(0, val.clone(), *del);
                    wait_for_input(&format!("{}Queue pasted!", ansi::GREEN), cfg);
                }
                Clipboard::Quickie(val) => {
                    for (i, line) in val.get_dirs().iter().enumerate() {
                        dir.quickie(obj).ins(0, line.clone(), val.get_del()[i]);
                    }
                    wait_for_input(&format!("{}Queue pasted!", ansi::GREEN), cfg);
                }
                Clipboard::Resources(_) => {}
                _ => {
                    wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg);
                }
            },
            MenuRes::Info => {
                information(rss, cmp, cfg);
            }
            MenuRes::Enter(_) => {
                get_str("Something went horribly wrong!", cfg);
            }
        };
    }
}
pub fn object_menu_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context(Config::QUIT, Some("exit to system menu".to_string()), ctx, dis);
    cfg.update_context(Config::NEW, Some("install a component".to_string()), ctx, dis);
    cfg.update_context(Config::DELETE, Some("remove a component".to_string()), ctx, dis);
}
pub fn transfer(rss: &ResourceDict, sys: &mut Systems, obj: ObjectID, cfg: &mut Config) {
    let temp = sys.get_o(obj).resources().get_currs().clone(); //Gets current resources
    let mut max = temp.iter(); //Gets maximum resources
    let transfer_cap = sys.get_o(obj).resources().get_curr(crate::resources::constants::TRANSFER); //Gets transfer capacity
    let total_cap: Vec<u64> = resources::get_transfer_max(rss, transfer_cap)
        .into_iter()
        .map(|x| {
            let y = *max.next().unwrap();
            if y > x {
                x
            } else {
                y
            }
        })
        .collect(); //Gets maximum transferrable
    let temp = get_resource_filtered(rss, &total_cap, cfg); //Gets the resource to transfer
    let resource;
    if let Some(val) = temp {
        resource = val;
    } else {
        return; //Returns
    }
    println!("How many?");
    let amt = get_from_input_valid(
        &format!(
            "Enter the amount of {} you want (0 to {}):",
            &rss.get(resource),
            total_cap[resource.get()]
        ),
        "Please enter a valid number!",
        cfg,
        |x| x <= &total_cap[resource.get()],
    );
    let other = if let Some(val) = select_object_docked(sys, obj, cfg) {
        val
    } else {
        return;
    };
    if !sys.get_o(obj).resources_mut().rmv_res(resource, amt) {
        println!("The transfer failed somehow!");
        wait_for_input("Press enter to continue:", cfg);
        return;
    }
    if !sys
        .get_o(obj)
        .resources_mut()
        .rmv_res(crate::resources::constants::TRANSFER, amt * rss.get_transfer_costs()[resource.get()])
    {
        sys.get_o(obj).resources_mut().add_res(resource, amt); //Undoes it
        println!("The transfer failed somehow!");
        wait_for_input("Press enter to continue:", cfg);
        return;
    }
    sys.get_o(other).resources_mut().add_res(resource, amt); //Succeeds; adds the resources to the other object
    println!("{} resources were successfully transferred!", amt); //Helpful message
    wait_for_input("Press enter to continue:", cfg); //Waits
}
pub fn get_object(sys: &Systems, system: SystemID, cfg: &mut Config) -> Option<ObjectID> {
    generic_select(
        &sys.get_s_stat(system).display(sys.get_o_names(), sys),
        sys.get_s_stat(system).get_objs().len(),
        |x| Some(ObjectID::new(x)),
        cfg,
        |x| if let Clipboard::Object(val) = x { Some(*val) } else { None },
    )
}
