use std::unimplemented;

use crate::{
    component::Components,
    instr::{Instrs, Queue},
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
};

use super::{
    ansi,
    clipboard::Clipboard,
    config::Config,
    context,
    from_str::{InBounds, MenuRes},
    instr::{instr_menu, make_instr, make_queue, select_queue},
    io::{get_from_input_valid, wait_for_input},
};

pub fn instrs_menu(sys: &mut Systems, obj: ObjectID, cmp: &Components, rss: &ResourceDict, instrs: &mut Instrs, cfg: &mut Config) {
    loop {
        println!("Viewing current instructions for {}:", sys.get_object_name(obj));
        print!("{}", ansi::GREEN);
        println!("{}", cfg.display(context::INSTRS_MENU));
        println!("{}", instrs.display()); //Prints stuff
        print!("{}", ansi::RESET);
        let input: MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x: &MenuRes| x.in_bounds(&instrs.len())); //Gets input
        match input {
            MenuRes::Exit => break,                                      //Exit out
            MenuRes::New => make_queue(sys, obj, cmp, rss, instrs, cfg), //Make new queue
            MenuRes::Del => {
                if let Some(pos) = select_queue(instrs, cfg) {
                    instrs.rmv(pos);
                    wait_for_input(&format!("{}Queue successfully removed!", ansi::GREEN), cfg);
                } else {
                    wait_for_input(&format!("{}Queue removal aborted!", ansi::RED), cfg);
                }
            } //Remove queue
            MenuRes::Enter(val) => {
                let name = instrs.get_name(val);
                queue_menu(sys, obj, cmp, rss, instrs.get_queue(val), name, cfg)
            } //Enter another queue
            MenuRes::Copy(val) => *(cfg.clipboard(val)) = Clipboard::Instrs(instrs.clone()),
            MenuRes::Paste(val) => match cfg.clipboard(val) {
                Clipboard::Instrs(val) => {
                    instrs.merge(val);
                }
                Clipboard::Queue(val) => {
                    instrs.add(val.clone(), "pasted queue".to_string());
                }
                Clipboard::Instr(val, temp) => {
                    instrs.add(Queue::new(*temp, val.clone()), "pasted instruction".to_string());
                }
                _ => {
                    wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg);
                }
            },
            _ => {
                wait_for_input(&format!("{}Please enter a valid input", ansi::RED), cfg);
            }
        }
    }
} //A menu
pub fn instrs_menu_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context_all(dis);
    cfg.update_context(Config::QUIT, Some("exit".to_string()), ctx, dis);
    cfg.update_context(Config::NEW, Some("make a new queue".to_string()), ctx, dis);
    cfg.update_context(Config::DELETE, Some("delete a queue".to_string()), ctx, dis);
}

pub fn queue_menu(sys: &mut Systems, obj: ObjectID, cmp: &Components, rss: &ResourceDict, queue: &mut Queue, name: String, cfg: &mut Config) {
    loop {
        println!("Viewing queue {}:", name);
        println!("{}", cfg.display(context::QUEUE_MENU));
        println!("{}", queue.display(obj, sys, rss, cmp)); //Displays stuff
        let input: MenuRes = get_from_input_valid("", "Please enter a valid input.", cfg, |x: &MenuRes| x.in_bounds(&queue.len())); //Gets input
        match input {
            MenuRes::Exit => break, //Exits out
            MenuRes::New => {
                println!("{}", queue.display(obj, sys, rss, cmp)); //Displays stuff
                println!("{}. Add on end", queue.len());
                let pos: usize = get_from_input_valid("Where do you want to put the instruction?", "Please enter a valid location!", cfg, |x| {
                    *x <= queue.len()
                }); //Gets pos
                if let Some(val) = make_instr(rss, cmp, sys, obj, queue.len() + 1, cfg) {
                    //Gets instr
                    queue.ins(val, pos); //Inserts if it wasn't aborted
                    println!("Instruction insertion successful!");
                } else {
                    println!("{}Instruction insertion aborted!", ansi::RED);
                }
                wait_for_input("Press enter to continue: ", cfg);
            } //Makes a new instruction from input
            MenuRes::Del => {
                println!("{}", queue.display(obj, sys, rss, cmp)); //Displays stuff
                println!("{}{}. Abort", ansi::RED, queue.len());
                let pos: usize = get_from_input_valid(
                    "Which instruction do you want to delete?",
                    "Please enter a valid number (or the queue's length)",
                    cfg,
                    |x| *x <= queue.len(),
                );
                if pos < queue.len() {
                    //If a valid number was inputted...
                    queue.rmv(pos); //Removes the queue
                    println!("Instruction removal successful!");
                } else {
                    println!("{}Instruction removal aborted!", ansi::RED);
                    //Otherwise, does nothing.
                }
                wait_for_input("Press enter to continue: ", cfg);
            } //Removes an instruction based on input
            MenuRes::Enter(val) => {
                let temp = queue.len();
                instr_menu(rss, cmp, sys, obj, queue.get(val), temp, cfg)
            }
            MenuRes::Copy(val) => *cfg.clipboard(val) = Clipboard::Queue(queue.clone()),
            MenuRes::Paste(_) => unimplemented!(),
            _ => wait_for_input(&format!("{}Please enter a valid input.", ansi::RED), cfg),
        }
    }
}

pub fn queue_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context(Config::QUIT, Some("exit to object menu".to_string()), ctx, dis);
    cfg.update_context(Config::NEW, Some("create a queue".to_string()), ctx, dis);
    cfg.update_context(Config::DELETE, Some("delete a queue".to_string()), ctx, dis);
    cfg.update_context(Config::TICK, None, ctx, dis);
    cfg.update_context(Config::INFO, None, ctx, dis);
}
