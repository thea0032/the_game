use crate::{
    component::Components,
    extra_bits,
    instr::{Instr, InstrID, Instrs, Queue},
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
};

use super::{
    ansi,
    clipboard::Clipboard,
    component::{select_component_unfiltered, select_components_unfiltered},
    condition::make_condition,
    config::Config,
    from_str::{InBounds, MenuRes},
    io::{get_from_input, get_from_input_valid, get_str, wait_for_input},
    location::get_location,
    object::get_object,
    recipe::{select_recipe_unfiltered, select_recipes_unfiltered},
    resources::get_rss,
    select::{generic_select, generic_select_simple},
    system::get_system,
};

pub fn make_queue(sys: &mut Systems, obj: ObjectID, cmp: &Components, rss: &ResourceDict, instrs: &mut Instrs, cfg: &mut Config) {
    let is_kept: bool = get_from_input(
        "Is this queue for tasks that are going to be re-used? ",
        "Please enter true or false. ",
        cfg,
    ); //Gets property from input
    let name: String = get_str("What do you want to call this queue?", cfg); //Gets property from input
    wait_for_input(
        "You will now be prompted to enter the first instruction of the queue. Press enter to continue: ",
        cfg,
    ); //Explains what the next part is.
    if let Some(val) = make_instr(rss, cmp, sys, obj, 1, cfg) {
        //Gets an instruction
        let new_queue = Queue::new(!is_kept, val); //Makes the queue based on the inputs
        instrs.add(new_queue, name); //Adds it
    } else {
        wait_for_input(&format!("{}Queue aborted!", ansi::RED), cfg); //If you didn't make a new instruction, abort
    }
}
pub fn make_instr(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, obj: ObjectID, queue_len: usize, cfg: &mut Config) -> Option<Instr> {
    println!("Enter the type of instruction: ");
    println!("{}", ansi::GREEN);
    println!("0. Move to another location within system. ");
    println!("1. Jump to another system. ");
    println!("2. Move to another object (Includes jumping to another system if needed)");
    println!("{}", ansi::CYAN);
    println!("3. Transfer resources to another object (includes movement to the object)");
    println!("4. Grab resources from another object (includes movement to the object)");
    println!("5. Perform a recipe.");
    println!("6. Install a component.");
    println!("{}", ansi::MAGENTA);
    println!("7. If a certain condition is fulfilled, do something. Otherwise, do something else. ");
    println!("8. Execute as many of the following instructions at once as possible.");
    println!("9. Go to a different position in the queue (default = advance 1).");
    println!("{}", ansi::BLUE);
    println!("10. Do nothing and stay on this until manual intervention (useful to suspend a queue temporarily, or in if statements");
    print!("{}", ansi::GREEN);
    println!("11. Continue immediately (useful as a placeholder)");
    print!("{}", ansi::RED);
    println!("12. fail and cause an instruction error (useful as a warming)");
    println!();
    println!("13. Abort instruction creation");
    print!("{}", ansi::RESET); //Prints stuff out
    const LEN: usize = 13; //Amount of options
    let response: usize = get_from_input_valid("", "Please enter a valid input!", cfg, |x| x <= &LEN); //Gets input
    match response {
        //Gives an instruction based on it
        0 => Some(make_move(cfg)),
        1 => Some(make_jump(sys, cfg)),
        2 => Some(make_moveto(sys, cfg)),
        3 => Some(make_transfer(rss, sys, cfg)),
        4 => Some(make_grab(rss, sys, cfg)),
        5 => Some(make_auto_recipe(cmp, cfg)),
        6 => Some(make_auto_cmp(cmp, cfg)),
        7 => Some(make_if(sys, obj, cmp, rss, queue_len, cfg)),
        8 => Some(make_all()),
        9 => Some(make_goto(queue_len, cfg)),
        10 => Some(make_sticky()),
        11 => Some(make_end()),
        12 => Some(make_fail()),
        13 => None,
        _ => panic!("Input validation has deeply failed!"),
    }
}
pub fn make_move(cfg: &mut Config) -> Instr {
    Instr::Move(get_location(cfg)) //Gets a location
} //Makes a move instruction from input
pub fn make_jump(sys: &Systems, cfg: &mut Config) -> Instr {
    Instr::Jump(if let Some(val) = get_system(sys, cfg) { val } else { return Instr::Fail }) //Gets a system
} //Makes a jump instruction from input
pub fn make_transfer(rss: &ResourceDict, sys: &Systems, cfg: &mut Config) -> Instr {
    let mut input: Vec<u128> = extra_bits::fill(rss.len(), 0);
    get_rss(rss, &mut input, cfg);
    let system = if let Some(system) = get_system(sys, cfg) {
        system
    } else {
        return Instr::Fail;
    };
    let o = get_object(sys, system, cfg);
    if let Some(val) = o {
        Instr::Transfer(input, val)
    } else {
        Instr::Fail
    }
} //Makes a transfer instruction from input
pub fn make_grab(rss: &ResourceDict, sys: &Systems, cfg: &mut Config) -> Instr {
    let mut input: Vec<u128> = extra_bits::fill(rss.len(), 0);
    get_rss(rss, &mut input, cfg);
    let system = if let Some(system) = get_system(sys, cfg) {
        system
    } else {
        return Instr::Fail;
    };
    let o = get_object(sys, system, cfg);
    if let Some(val) = o {
        Instr::Grab(input, val)
    } else {
        Instr::Fail
    }
} //Makes a grab instruction from input
pub fn make_moveto(sys: &Systems, cfg: &mut Config) -> Instr {
    let system = if let Some(system) = get_system(sys, cfg) {
        system
    } else {
        return Instr::Fail;
    };
    let o = get_object(sys, system, cfg);
    if let Some(val) = o {
        Instr::MoveTo(val)
    } else {
        Instr::Fail
    }
} //Makes a move to instruction from input
pub fn make_if(sys: &mut Systems, obj: ObjectID, cmp: &Components, rss: &ResourceDict, len: usize, cfg: &mut Config) -> Instr {
    Instr::If(
        make_condition(),
        Box::new(make_instr(rss, cmp, sys, obj, len, cfg).unwrap_or(Instr::Fail)),
        Box::new(make_instr(rss, cmp, sys, obj, len, cfg).unwrap_or(Instr::Fail)),
    )
} //Makes an if instruction from input
pub fn make_all() -> Instr {
    Instr::All(Vec::new())
} //Makes an empty all instruction
pub fn make_goto(queue_len: usize, cfg: &mut Config) -> Instr {
    Instr::GoTo(InstrID::new(get_from_input_valid(
        &format!("Enter the position you want to go to (from 0 to {}): ", queue_len,),
        "Please enter a valid number!",
        cfg,
        |x| x < &queue_len,
    )))
} //Makes a goto instruction from input
pub fn make_auto_recipe(cmp: &Components, cfg: &mut Config) -> Instr {
    let res = select_recipes_unfiltered(cmp, cfg);
    if let Some(res) = res {
        Instr::PerformRecipe(res.0, res.1)
    } else {
        Instr::Fail
    }
} //Makes an automatic recipe performing instruction from input
pub fn make_auto_cmp(cmp: &Components, cfg: &mut Config) -> Instr {
    let res = select_components_unfiltered(cmp, cfg);
    if let Some(res) = res {
        Instr::InstallComponent(res.0, res.1)
    } else {
        Instr::Fail
    }
} //Makes an automatic component installing instruction from input
pub fn make_sticky() -> Instr {
    Instr::Sticky
} //Makes a sticky instruction
pub fn make_end() -> Instr {
    Instr::End
} //Makes an end instruction
pub fn make_fail() -> Instr {
    Instr::Fail
} //Makes a fail instruction

pub fn instr_menu(rss: &ResourceDict, cmp: &Components, sys: &mut Systems, obj: ObjectID, instr: &mut Instr, queue_len: usize, cfg: &mut Config) {
    loop {
        println!("Viewing {}", instr.display(obj, sys, rss, cmp));
        println!("Options:");
        println!("0. Exit"); //Prints stuff
        let len = 1;
        println!("{}", display_options(instr, obj, sys, rss, cmp, len)); //Displays your options
        let input: usize = get_from_input_valid("", "Please enter a valid input!", cfg, |x| *x < get_len(instr, len)); //Gets input
        match input {
            0 => break,                                                                 //Exit
            _ => parse_options(instr, input - len, obj, sys, rss, cmp, queue_len, cfg), /* Does something based on the instruction viewed. */
        }
    }
}
pub fn display_options(instr: &Instr, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &Components, len: usize) -> String {
    match instr {
        Instr::All(val) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}: Add an instruction to this: \n", len));
            res.push_str(&format!("{}: Remove an instruction from this: \n", len + 1));
            let extra = 2;
            for (i, item) in val.iter().enumerate() {
                res.push_str(&format!("{}. {}\n", len + i + extra, item.display(obj, sys, rss, cmp)));
            }
            res
        }
        Instr::Move(_) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change destination\n", len));
            res
        }
        Instr::Jump(_) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change system to jump to\n", len));
            res
        }
        Instr::Transfer(_, _) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change resources transferred\n", len));
            res.push_str(&format!("{}. Change object destination\n", len + 1));
            res
        }
        Instr::Grab(_, _) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change resources grabbed\n", len));
            res.push_str(&format!("{}. Change object destination\n", len + 1));
            res
        }
        Instr::MoveTo(_) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change object destination\n", len));
            res
        }
        Instr::If(_, _, _) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change condition\n", len));
            res.push_str(&format!("{}. Change instruction if true\n", len + 1));
            res.push_str(&format!("{}. Change instruction if false\n", len + 2));
            res
        }
        Instr::GoTo(_) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Change id that is went to\n", len));
            res
        }
        Instr::PerformRecipe(_, _) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Replace recipe performed\n", len));
            res.push_str(&format!("{}. Replace amount of times recipe is performed\n", len + 1));
            res
        }
        Instr::InstallComponent(_, _) => {
            let mut res: String = "".to_string();
            res.push_str(&format!("{}. Replace component installed\n", len));
            res.push_str(&format!("{}. Replace amount of times component is installed\n", len + 1));
            res
        }
        Instr::Sticky => "".to_string(),
        Instr::End => "".to_string(),
        Instr::Fail => "".to_string(),
    }
} //Displays instruction options based on instruction
pub fn get_len(instr: &Instr, len: usize) -> usize {
    match instr {
        Instr::All(val) => val.len() + 2 + len, //based on options
        Instr::Transfer(_, _) | Instr::Grab(_, _) | Instr::PerformRecipe(_, _) | Instr::InstallComponent(_, _) => len + 2,
        Instr::If(_, _, _) => len + 3,
        Instr::MoveTo(_) | Instr::GoTo(_) | Instr::Move(_) | Instr::Jump(_) => len + 1,
        Instr::Sticky | Instr::End | Instr::Fail => len,
    }
}
pub fn parse_options(
    instr: &mut Instr, input: usize, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &Components, queue_len: usize, cfg: &mut Config,
) {
    match instr {
        Instr::All(val) => {
            match input {
                0 => {
                    let add_num = get_from_input_valid(
                        "Enter the position where you want to insert your position:",
                        "Please enter a valid number!",
                        cfg,
                        |x| *x <= val.len(),
                    ); //Inserts
                    let add_val = make_instr(rss, cmp, sys, obj, queue_len, cfg); //Gets from input
                    if let Some(v) = add_val {
                        val.insert(add_num, v); //Inserts an instruction based
                                                // on the input
                    }
                }
                1 => {
                    let rmv_num: usize = get_from_input_valid("Enter the position you want to remove:", "Please enter a valid number", cfg, |x| {
                        *x < val.len()
                    }); //Gets from input
                    val.remove(rmv_num); //Removes the instruction
                }
                _ => {
                    instr_menu(rss, cmp, sys, obj, &mut val[input - 1], queue_len, cfg);
                    //Enters the instruction displayed
                }
            }
        }
        Instr::Move(val) => {
            *val = get_location(cfg); //Updates location
        }
        Instr::Jump(val) => {
            if let Some(s) = get_system(sys, cfg) {
                *val = s; //Updates system
            }
        }
        Instr::Transfer(val, val2) => {
            match input {
                0 => {
                    get_rss(rss, val, cfg) //Updates resources to be transferred
                }
                1 => {
                    if let Some(val) = get_system(sys, cfg) {
                        if let Some(val) = get_object(sys, val, cfg) {
                            *val2 = val; //Updates object
                        }
                    }
                }
                _ => {}
            }
        }
        Instr::Grab(val, val2) => {
            match input {
                0 => {
                    get_rss(rss, val, cfg) //Updates resources to be transferred
                }
                1 => {
                    if let Some(val) = get_system(sys, cfg) {
                        if let Some(val) = get_object(sys, val, cfg) {
                            *val2 = val; //Updates object
                        }
                    }
                }
                _ => {}
            }
        }
        Instr::MoveTo(val) => {
            if let Some(system) = get_system(sys, cfg) {
                if let Some(obj) = get_object(sys, system, cfg) {
                    *val = obj; //Updates object
                }
            }
        }
        Instr::If(_, _, _) => {
            panic!("Not implemented yet!");
        }
        Instr::GoTo(val) => {
            *val = InstrID::new(get_from_input_valid("Enter the new position", "Please enter a valid input!", cfg, |x| {
                *x < queue_len
            })); //Changes the instruction we go to
        }
        Instr::PerformRecipe(recipe, amt) => {
            match input {
                0 => {
                    if let Some(val) = select_recipe_unfiltered(cmp, cfg) {
                        *recipe = val; //Option 1: Select a new recipe
                    };
                }
                1 => {
                    *amt = get_from_input_valid("Enter the new amount", "Please enter a valid input!", cfg, |_| true); //Option 2: Select a new
                                                                                                                       // amount
                }
                _ => {}
            }
        }
        Instr::InstallComponent(component, amt) => {
            match input {
                0 => {
                    if let Some(val) = select_component_unfiltered(cmp, cfg) {
                        *component = val; //Option 1: select a new component
                    };
                }
                1 => {
                    *amt = get_from_input_valid("Enter the new amount", "Please enter a valid input!", cfg, |_| true); //Option 2: Select a new
                                                                                                                       // amount
                }
                _ => {}
            }
        }
        Instr::Sticky => {} //No extra options
        Instr::End => {}    //No extra options
        Instr::Fail => {}   //No extra options
    }
}
pub fn select_queue(instrs: &mut Instrs, cfg: &mut Config) -> Option<usize> {
    generic_select_simple(&instrs.display(), instrs.len(), |x| Some(x), cfg)
}
