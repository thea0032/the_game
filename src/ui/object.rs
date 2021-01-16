use location::get_location;
use quickie::quickie;
use resources::{get_amt, get_resource_filtered};

use crate::{instr::Directions, systems::{object_id::ObjectID, system_id::SystemID}, ui::io::*};
use crate::{component::*, resources::*, systems::*, ui::{*, system::*, component::*}};
pub fn make_object(rss:&ResourceDict, cmp:&mut Components, sys:&mut Systems, dir:&mut Directions, system:SystemID){
    refresh();
    let name:String = get_str("What do you want to call your new object?");
    let loc = get_location();
    if get_from_input("Are you sure?", "Please enter true or false"){
        println!("Object {} created!", name);
        sys.add_o(rss, cmp, dir, name, loc, system);
        wait_for_input("Press enter to continue.");
    } else {
        println!("Object creation aborted.");
        wait_for_input("Press enter to continue.");
    }
}
pub fn object_menu(rss:&ResourceDict, cmp:&mut Components, sys:&mut Systems, obj:ObjectID, dir:&mut Directions){
    loop{
        println!("Displaying...");
        refresh();
        println!("Viewing {} ", sys.get_o(obj).name());
        println!("{}", sys.get_o(obj).display(rss, cmp));
        println!("Options: ");
        println!("{}", ansi::GREEN);
        println!("0. End turn; wait a tick");
        println!("{}", ansi::YELLOW);
        println!("1. Break out of object menu");
        println!("{}", ansi::CYAN);
        println!("2. Add a component ");
        println!("3. Remove a component");
        println!("4. Use a recipe ");
        println!("5. Transfer resources to another object: ");
        println!("{}", ansi::MAGENTA);
        println!("6. Get detailed information on components ");
        println!("7. Get detailed information on recipes ");
        println!("{}", ansi::BLUE);
        println!("8. Enter instruction menu");
        println!("9. Enter quick instruction menu (all quick instructions are done every turn");
        print!("{}", ansi::RESET);
        let len:usize = 10;
        let response:usize = get_from_input_valid("", "Please enter a valid input.", |x| x < &len);
        match response{
            0=>sys.tick(rss, cmp, dir),
            1=>break,
            2=>add_component(cmp, sys.get_o(obj)),
            3=>remove_component(cmp, sys.get_o(obj)),
            4=>recipe::recipe(cmp, sys.get_o(obj)),
            5=>transfer(rss, sys, obj),
            6=>details(rss, cmp),
            7=>recipe::r_details(rss, cmp),
            8=>{instr::instrs_menu(sys, obj, cmp, rss, dir.instrs(obj))},
            9=>{quickie(rss, cmp, sys, dir.quickie(obj), obj)}
            _=>{io::get_str("Something went horribly wrong!");},
        };
    }
}
pub fn transfer(rss:&ResourceDict, sys:&mut Systems, obj:ObjectID){
    refresh();
    let temp = sys.get_o(obj).resources().get_currs().clone();
    let mut max = temp.iter();
    let transfer_cap = sys.get_o(obj).resources().get_curr(crate::resources::constants::TRANSFER);
    let total_cap:Vec<u128> = resources::get_transfer_max(rss, transfer_cap).into_iter().map(|x| {let y = *max.next().unwrap(); if y > x {x} else {y}}).collect();
    let temp = get_resource_filtered(rss, &total_cap);
    let resource;
    if let Some(val) = temp{
        resource = val;
    } else {
        return;
    }
    println!("How many?");
    let amt = get_amt(rss, sys.get_o(obj), resource);
    let other = select_object_docked(sys, obj);
    if !sys.get_o(obj).resources_mut().rmv_res(resource, amt){
        println!("The transfer failed somehow!");
        wait_for_input("Press enter to continue.");
        return;
    }
    sys.get_o(other).resources_mut().add_res(resource, amt);
    println!("{} resources were successfully transferred!", amt);
    wait_for_input("Press enter to continue.");
}
pub fn get_object(sys:&Systems, curr_sys:SystemID) -> ObjectID{
    refresh();
    println!("{}", sys.get_s_stat(curr_sys).display(0, sys.get_o_names(), sys));
    let input:usize = get_from_input_valid("Enter the object: ", "Please enter a valid number", |x| x < &sys.get_s_stat(curr_sys).get_objs().len());
    return sys.get_s_stat(curr_sys).get_objs()[input];
}