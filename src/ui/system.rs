use object_id::ObjectID;

use crate::{
    component::*,
    resources::*,
    systems::*,
    ui::{object::*, *},
};
use crate::{instr::Directions, systems::system_id::SystemID, ui::io::*};
pub fn system_menu(
    rss: &ResourceDict,
    cmp: &mut Components,
    sys: &mut Systems,
    system: SystemID,
    dir: &mut Directions,
    cfg: &mut Config,
) {
    loop {
        println!("Viewing {}", sys.get_s_name(system));
        print!("{}", ansi::GREEN);
        println!("{}. End turn; wait a tick", cfg.tick());
        println!("{}", ansi::YELLOW);
        println!("{}. Go back", cfg.back());
        println!("{}. Create a new object", cfg.new());
        println!("{}", ansi::CYAN);
        println!(
            "{}",
            sys.get_s_stat(system)
                .display(sys.get_o_names(), sys)
        );
        print!("{}", ansi::RESET); //Print statements are self-explanatory
        let response:SysRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x:&SysRes| x.in_bounds(&sys.get_s_stat(system).len())); //Gets your response
        match response {
            SysRes::Tick => sys.tick(rss, cmp, dir), //If it's zero, we tick
            SysRes::Exit => break,                   //If it's 1, goes back to the system menu
            SysRes::New => make_object(rss, cmp, sys, dir, system, cfg),
            SysRes::System(val) => object_menu(
                rss,
                cmp,
                sys,
                ObjectID::new(val),
                dir,
                cfg,
            ),
        };
    }
}
pub fn select_object_filtered(sys: &Systems, id: SystemID, filter: Vec<bool>, cfg:&mut Config) -> ObjectID {
    println!(
        "{}",
        sys.get_s_stat(id)
            .display_filtered(0, &filter, sys.get_o_names())
    );
    let len = filter.iter().filter(|x| **x).count();
    let input: usize = get_from_input_valid(
        "Enter the object you want: ",
        "Please enter a valid id",
        cfg,
        |x| x < &len,
    );
    sys.get_s_stat(id).get_objs()[crate::extra_bits::filter(input, &filter)]
}
pub fn select_object_docked(sys: &Systems, id: ObjectID, cfg:&mut Config) -> ObjectID {
    let curr_system_id = sys.get_o_sys(id);
    let curr_location = *sys.get_o_stat(id).get_location_stat();
    let filter: Vec<bool> = sys
        .get_os(sys.get_s_stat(curr_system_id).get_objs())
        .iter()
        .map(|x| x.get_location_stat().eq(&curr_location))
        .collect();
    select_object_filtered(sys, curr_system_id, filter, cfg)
}
pub fn get_system(sys: &Systems, cfg:&mut Config) -> SystemID {
    println!("{}", sys.display());
    SystemID::new(get_from_input_valid(
        "Enter the system you want",
        "Please enter a valid number",
        cfg,
        |x| x < &sys.len(),
    ))
}

pub enum SysRes{
    System(usize),
    Tick,
    Exit,
    New,
}
impl FromString for SysRes{
    fn from_string(s: &str, cfg:&mut Config) -> Option<Self> {
        if s == cfg.tick() {
            return Some(SysRes::Tick);
        } else if s == cfg.back() {
            return Some(SysRes::Exit);
        } else if s == cfg.new() {
            return Some(SysRes::New);
        } else if let Ok(val) = usize::from_str(s){
            return Some(SysRes::System(val));
        } else {
            return None;
        }
    }
    fn from_string_s(_:&str) -> Option<Self>
    where Self:Sized {
        None
    }
}
impl InBounds for SysRes{
    fn in_bounds(&self, bounds:&usize) -> bool {
        if let SysRes::System(val) = self{
            return val < bounds;
        } else {
            return true;
        }
    }
}