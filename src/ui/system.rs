use object_id::ObjectID;

use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};
use crate::{component::*, resources::*, systems::*, ui::{object::*, *}};
const EXTRA_OPTIONS_SYS:usize = 3;//The amount of extra options
pub fn system_menu(rss:&ResourceDict, cmp:&mut Components, sys:&mut Systems, system:SystemID, dir:&mut Directions){
    loop{
        refresh();
        println!("Viewing {}", sys.get_s_name(system));
        print!("{}", ansi::GREEN);
        println!("0. End turn; wait a tick");
        println!("{}", ansi::YELLOW);
        println!("1. Go back");
        println!("2. Create an object");
        println!("{}", ansi::CYAN);
        println!("{}", sys.get_s_stat(system).display(EXTRA_OPTIONS_SYS, sys.get_o_names(), sys));
        print!("{}", ansi::RESET);//Print statements are self-explanatory
        let len = sys.get_s(system).len() + EXTRA_OPTIONS_SYS;
        let response = get_from_input_valid("", "Please enter a valid input.", |x| x < &len);
        match response{
            0=>sys.tick(rss, cmp, dir),
            1=>break,
            2=>make_object(rss, cmp, sys, dir, system),
            _=>object_menu(rss, cmp, sys, ObjectID::new(response - EXTRA_OPTIONS_SYS), dir),
        };
    }
}
pub fn select_object_filtered(sys:&Systems, id:SystemID, filter:Vec<bool>) -> ObjectID{
    refresh();
    println!("{}", sys.get_s_stat(id).display_filtered(0, &filter, sys.get_o_names()));
    let len = filter.iter().filter(|x| **x).count();
    let input:usize = get_from_input_valid("Enter the object you want: ", "Please enter a valid id", |x| x < &len);
    return sys.get_s_stat(id).get_objs()[crate::extra_bits::filter(input, &filter)];
}
pub fn select_object_docked(sys:&Systems, id:ObjectID) -> ObjectID{
    refresh();
    let curr_system_id = sys.get_o_sys(id);
    let curr_location = *sys.get_o_stat(id).get_location_stat();
    let filter:Vec<bool> = 
        sys.get_os(sys.get_s_stat(curr_system_id).get_objs())
        .iter()
        .map(|x| x.get_location_stat().eq(&curr_location))
        .collect();
    return select_object_filtered(sys, curr_system_id, filter);
}
pub fn get_system(sys:&Systems) -> SystemID{
    refresh();
    println!("{}", sys.display(0));
    return SystemID::new(get_from_input_valid("Enter the system you want", "Please enter a valid number", |x| x < &sys.len()));
}