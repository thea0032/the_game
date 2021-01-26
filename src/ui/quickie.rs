use crate::{
    component::Components,
    instr::Quickie,
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
};

use super::{ansi, config::Config, instr, io::{get_from_input_valid, wait_for_input,}};

pub fn quickie(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, dir: &mut Quickie, obj: ObjectID, cfg: &mut Config) {
    loop {
        print!("{}", ansi::GREEN);
        println!("0. Go back");
        println!("1. Add a permanent instruction");
        println!("2. Add a temporary instruction");
        println!("3. Remove an instruction.");
        let len = 4;
        println!("{}", dir.display(len, obj, sys, rss, cmp)); //Displays options
        let input: usize = get_from_input_valid("", "Please enter a valid input", cfg, |x| *x < len + dir.len()); //Gets option
        
        match input {
            0 => break, //Breaks out of menu
            1 => {
                println!("{}", dir.display(0, obj, sys, rss, cmp));
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
            2 => {
                println!("{}", dir.display(0, obj, sys, rss, cmp));
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
            3 => {
                println!("{}", dir.display(0, obj, sys, rss, cmp));
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
            _ => {
                let len = dir.len(); //Gets length
                instr::instr_menu(rss, cmp, sys, obj, &mut dir.get(input - 4), len, cfg);
                //Enters instruction menu
            }
        } //Responds to options
    }
}
