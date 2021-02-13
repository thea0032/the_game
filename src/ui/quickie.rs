use crate::{
    component::Components,
    instr::Quickie,
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
};

use super::{ansi, config::Config, context::QUICK_MENU, from_str::{InBounds, MenuRes}, instr, io::{get_from_input_valid, wait_for_input}};

pub fn quickie(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, dir: &mut Quickie, obj: ObjectID, cfg: &mut Config) {
    loop {
        print!("{}", ansi::GREEN);
        println!("{}", cfg.display(QUICK_MENU));
        println!("{}", dir.display(obj, sys, rss, cmp)); //Displays options
        let input: MenuRes = get_from_input_valid("", "Please enter a valid input", cfg, |x:&MenuRes| x.in_bounds(&dir.len())); //Gets option
        match input {
            MenuRes::Exit => break, //Breaks out of menu
            MenuRes::New => {
                println!("{}", dir.display(obj, sys, rss, cmp));
                println!("{}. Add on end", dir.len()); //Displays options
                let pos = get_from_input_valid(
                    "Enter the place where you want to add an instruction",
                    "Please enter a valid number!",
                    cfg,
                    |x| *x <= dir.len(),
                ); //Gets position
                let instr = instr::make_instr(rss, cmp, sys, obj, dir.len() + 1, cfg); //Gets instruction
                if let Some(val) = instr {
                    //If the instruction creation wasn't aborted
                    dir.ins(pos, val, false); //Inserts it
                    println!("{}Instruction insertion successful!", ansi::GREEN);
                } else {
                    println!("{}Instruction insertion aborted!", ansi::RED); //Aborts
                }
                wait_for_input("Press enter to continue: ", cfg); //Waits for input
            }
            MenuRes::Tick => {
                println!("{}", dir.display(obj, sys, rss, cmp));
                println!("{}. Add on end", dir.len());
                let pos = get_from_input_valid(
                    "Enter the place where you want to add an instruction",
                    "Please enter a valid number!",
                    cfg,
                    |x| *x <= dir.len(),
                );
                let instr = instr::make_instr(rss, cmp, sys, obj, dir.len() + 1, cfg);
                if let Some(val) = instr {
                    dir.ins(pos, val, true);
                    println!("{}Instruction insertion successful!", ansi::GREEN);
                } else {
                    println!("{}Instruction insertion aborted!", ansi::RED);
                }
                wait_for_input("Press enter to continue: ", cfg); //Same as before,
                                                                  // but the instruction
                                                                  // is temporary
            }
            MenuRes::Del => {
                println!("{}", dir.display(obj, sys, rss, cmp));
                println!("{}{}. abort", ansi::RED, dir.len()); //Displays options
                let pos = get_from_input_valid(
                    "Enter the place where you want to remove an instruction: ",
                    "Please enter a valid number!",
                    cfg,
                    |x| *x <= dir.len(),
                ); //Gets input
                if pos < dir.len() {
                    //If it wasn't aborted...
                    dir.rmv(pos); //removes
                    println!("{}Instruction removal successful!", ansi::GREEN); //text
                } else {
                    //aborts
                    println!("{}Instruction removal aborted!", ansi::RED); //text
                }
                wait_for_input("Press enter to continue: ", cfg); //waits for input
            }
            MenuRes::Enter(val) => {
                let len = dir.len(); //Gets length
                instr::instr_menu(rss, cmp, sys, obj, &mut dir.get(val), len, cfg);
                //Enters instruction menu
            }
            _ => {
                wait_for_input(&format!("{}Please enter a valid input", ansi::RED), cfg);
            }
        } //Responds to options
    }
}
pub fn quickie_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context_all(dis);
    cfg.update_context(Config::QUIT, Some("exit".to_string()), ctx, dis);
    cfg.update_context(Config::NEW, Some("add a permanent instruction".to_string()), ctx, dis);
    cfg.update_context(Config::DELETE, Some("remove an instruction".to_string()), ctx, dis);
    cfg.update_context(Config::TICK, Some("add a temporary instruction".to_string()), ctx, dis);
}