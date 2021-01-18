use location::get_location;
use quickie::quickie;
use resources::{get_resource_filtered};

use crate::{instr::Directions, systems::{object_id::ObjectID, system_id::SystemID}, ui::io::*};
use crate::{component::*, resources::*, systems::*, ui::{*, system::*, component::*}};
pub fn make_object(rss:&ResourceDict, cmp:&mut Components, sys:&mut Systems, dir:&mut Directions, system:SystemID){
    let name:String = get_str("What do you want to call your new object?");//Gets the object's name from input
    let loc = get_location();//Gets the object's location from input
    if get_from_input("Are you sure?", "Please enter true or false"){//
        println!("Object {} created!", name);
        sys.add_o(rss, cmp, dir, name, loc, system);
        wait_for_input("Press enter to continue.");
    } else {
        println!("Object creation aborted.");
        wait_for_input("Press enter to continue.");
    }
}//Makes an object
pub fn object_menu(rss:&ResourceDict, cmp:&mut Components, sys:&mut Systems, obj:ObjectID, dir:&mut Directions){
    loop{
        println!("Displaying...");
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
        print!("{}", ansi::RESET);//Displays options
        let len:usize = 10;
        let response:usize = get_from_input_valid("", "Please enter a valid input.", |x| x < &len);//Gets response
        match response{
            0=>sys.tick(rss, cmp, dir),//Advance 1 tick
            1=>break,//Break out of menu
            2=>add_component(cmp, sys.get_o(obj)),//Add component
            3=>remove_component(cmp, sys.get_o(obj)),//Remove component
            4=>recipe::perform_recipe(cmp, sys.get_o(obj)),//Perform recipe
            5=>transfer(rss, sys, obj),//Transfer resources
            6=>details(rss, cmp),//Gather details
            7=>recipe::r_details(rss, cmp),//Gather recipe details
            8=>{instr::instrs_menu(sys, obj, cmp, rss, dir.instrs(obj))},//Enter instructions menu
            9=>{quickie(rss, cmp, sys, dir.quickie(obj), obj)}//Enter quick instructions menu
            _=>{io::get_str("Something went horribly wrong!");},//Something went wrong!
        };
    }
}
pub fn transfer(rss:&ResourceDict, sys:&mut Systems, obj:ObjectID){
    let temp = sys.get_o(obj).resources().get_currs().clone();//Gets current resources
    let mut max = temp.iter();//Gets maximum resources
    let transfer_cap = sys.get_o(obj).resources().get_curr(crate::resources::constants::TRANSFER);//Gets transfer capacity
    let total_cap:Vec<u128> = resources::get_transfer_max(rss, transfer_cap).into_iter().map(|x| {let y = *max.next().unwrap(); if y > x {x} else {y}}).collect();//Gets maximum transferrable
    let temp = get_resource_filtered(rss, &total_cap);//Gets the resource to transfer
    let resource;
    if let Some(val) = temp{
        resource = val;
    } else {
        return;//Returns
    }
    println!("How many?");
    let amt = get_from_input_valid(
        &format!("Enter the amount of {} you want (0 to {}):", &rss.get(resource), total_cap[resource.get()]), 
        "Please enter a valid number!", |x| x <= &total_cap[resource.get()]);
    let other = select_object_docked(sys, obj);
    if !sys.get_o(obj).resources_mut().rmv_res(resource, amt){
        println!("The transfer failed somehow!");
        wait_for_input("Press enter to continue:");
        return;
    }
    if !sys.get_o(obj).resources_mut().rmv_res(crate::resources::constants::TRANSFER, amt * rss.get_transfer_costs()[resource.get()]){
        sys.get_o(obj).resources_mut().add_res(resource, amt);//Undoes it
        println!("The transfer failed somehow!");
        wait_for_input("Press enter to continue:");
        return;
    }
    sys.get_o(other).resources_mut().add_res(resource, amt);//Succeeds; adds the resources to the other object
    println!("{} resources were successfully transferred!", amt);//Helpful message
    wait_for_input("Press enter to continue:");//Waits
}
pub fn get_object(sys:&Systems, curr_sys:SystemID) -> ObjectID{
    println!("{}", sys.get_s_stat(curr_sys).display(0, sys.get_o_names(), sys));//Displays all objects in system
    let input:usize = get_from_input_valid("Enter the object: ", "Please enter a valid number", |x| x < &sys.get_s_stat(curr_sys).get_objs().len());//Gets input
    return sys.get_s_stat(curr_sys).get_objs()[input];//Returns the object based on the input
}