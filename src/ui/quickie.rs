use crate::{component::Components, instr::Quickie, resources::ResourceDict, systems::{Systems, object_id::ObjectID}};

use super::{ansi, instr, io::{get_from_input_valid, refresh, wait_for_input}};

pub fn quickie(rss:&ResourceDict, cmp:&Components, sys:&mut Systems, dir:&mut Quickie, obj:ObjectID){
    loop{
        refresh();
        print!("{}", ansi::GREEN);
        println!("0. Go back");
        println!("1. Add a permanent instruction");
        println!("2. Add a temporary instruction");
        println!("3. Remove an instruction.");
        let len = 4;
        println!("{}", dir.display(len, obj, sys, rss, cmp));
        let input:usize = get_from_input_valid("", "Please enter a valid input", |x| *x < len + dir.len());
        refresh();
        match input{
            0=>break,
            1=>{
                println!("{}", dir.display(0, obj, sys, rss, cmp));
                println!("{}. Add on end", dir.len());
                let pos = get_from_input_valid("Enter the place where you want to add an instruction", "Please enter a valid number!", |x| *x <= dir.len());
                let instr = instr::make_instr(rss, cmp, sys, obj, dir.len() + 1);
                if let Some(val) = instr{
                    dir.ins(pos, val, false);
                    println!("{}Instruction insertion successful!", ansi::GREEN);
                } else {
                    println!("{}Instruction insertion aborted!", ansi::RED);
                }
                wait_for_input("");
            },
            2=>{
                println!("{}", dir.display(0, obj, sys, rss, cmp));
                println!("{}. Add on end", dir.len());
                let pos = get_from_input_valid("Enter the place where you want to add an instruction", "Please enter a valid number!", |x| *x <= dir.len());
                let instr = instr::make_instr(rss, cmp, sys, obj, dir.len() + 1);
                if let Some(val) = instr{
                    dir.ins(pos, val, true);
                    println!("{}Instruction insertion successful!", ansi::GREEN);
                } else {
                    println!("{}Instruction insertion aborted!", ansi::RED);
                }
                wait_for_input("");
            },
            3=>{
                println!("{}", dir.display(0, obj, sys, rss, cmp));
                println!("{}{}. abort", ansi::RED, dir.len());
                let pos = get_from_input_valid("Enter the place where you want to remove an instruction: ", "Please enter a valid number!", |x| *x <= dir.len());
                if pos < dir.len(){
                    dir.rmv(pos);
                    println!("{}Instruction removal successful!", ansi::GREEN);
                } else {
                    println!("{}Instruction removal aborted!", ansi::RED);
                }
                wait_for_input("");
            }
            _=>{
                let len = dir.len();
                instr::instr_menu(rss, cmp, sys, obj, &mut dir.get(input - 4), len);
            }
        }
    }
}