use crate::{component::Components, instr::{Instr, InstrID, Instrs, Queue}, resources::{ResourceDict}, systems::{Systems, object_id::ObjectID}};

use super::{ansi, component::{select_component_unfiltered, select_components_unfiltered}, condition::make_condition, io::{get_from_input, get_from_input_valid, refresh, wait_for_input}, location::get_location, object::get_object, recipe::{select_recipe_unfiltered, select_recipes_unfiltered}, resources::{get_rss}, system::{get_system}};

pub fn make_queue(sys:&mut Systems, obj:ObjectID, cmp:&Components, rss:&ResourceDict, instrs:&mut Instrs){
    refresh();
    let is_kept:bool = get_from_input("Is this queue for tasks that are going to be re-used? ", "Please enter true or false. ");//Gets property from input
    let name:String = get_from_input("What do you want to call this queue?", "Please enter a valid string! You probably shouldn't see this.");//Gets property from input
    wait_for_input("You will now be prompted to enter the first instruction of the queue. Press enter to continue: ");//Explains what the next part is. 
    if let Some(val) = make_instr(rss, cmp, sys, obj, 1){//Gets an instruction
        let new_queue = Queue::new(!is_kept, val);//Makes the queue based on the inputs
        instrs.add(new_queue, name);//Adds it
    } else {
        wait_for_input(&format!("{}Queue aborted!", ansi::RED));//If you didn't make a new instruction, abort
    }
}
pub fn make_instr(rss:&ResourceDict, cmp:&Components, sys:&mut Systems, obj:ObjectID, queue_len:usize) -> Option<Instr>{
    refresh();
    println!("Enter the type of instruction: ");
    println!("{}", ansi::GREEN);
    println!("0. Move to another location within system. ");
    println!("1. Jump to another system. ");
    println!("2. Move to another object (Includes jumping to another system if needed)");
    println!("{}", ansi::CYAN);
    println!("3. Transfer resources to another object (includes movement to the object)");
    println!("4. Perform a recipe.");
    println!("5. Install a component.");
    println!("{}", ansi::MAGENTA);
    println!("6. If a certain condition is fulfilled, do something. Otherwise, do something else. ");
    println!("7. Execute as many of the following instructions at once as possible.");
    println!("8. Go to a different position in the queue (default = advance 1).");
    println!("{}", ansi::BLUE);
    println!("9. Do nothing and stay on this until manual intervention (useful to suspend a queue temporarily, or in if statements");
    print!("{}", ansi::GREEN);
    println!("10. Continue immediately (useful as a placeholder)");
    print!("{}", ansi::RED);
    println!("11. fail and cause an instruction error (useful as a warming)");
    println!("");
    println!("12. Abort instruction creation");
    print!("{}", ansi::RESET);
    const LEN:usize = 13;
    let response:usize = get_from_input_valid("", "Please enter a valid input!", |x| x <= &LEN);
    match response{
        0=>Some(make_move()),
        1=>Some(make_jump(sys)),
        2=>Some(make_moveto(sys)),
        3=>Some(make_transfer(rss, sys)),
        4=>Some(make_auto_recipe(cmp)),
        5=>Some(make_auto_cmp(cmp)),
        6=>Some(make_if(sys, obj, cmp, rss, queue_len)),
        7=>Some(make_all()),
        8=>Some(make_goto(queue_len)),
        9=>Some(make_sticky()),
        10=>Some(make_end()),
        11=>Some(make_fail()),
        12=>None,
        _=>panic!("Input validation has deeply failed!")
    }
}
pub fn make_move() -> Instr{
    Instr::Move(get_location())
}
pub fn make_jump(sys:&Systems) -> Instr{
    Instr::Jump(get_system(sys))
}
pub fn make_transfer(rss:&ResourceDict, sys:&Systems) -> Instr{
    return Instr::Transfer(get_rss(rss), get_object(sys, get_system(sys)));
}
pub fn make_moveto(sys:&Systems) -> Instr{
    Instr::MoveTo(get_object(sys, get_system(sys)))
}
pub fn make_if(sys:&mut Systems, obj:ObjectID, cmp:&Components, rss:&ResourceDict, len:usize) -> Instr{
    Instr::If(make_condition(), Box::new(make_instr(rss, cmp, sys, obj,len).unwrap_or(Instr::Fail)), Box::new(make_instr(rss, cmp, sys, obj,len).unwrap_or(Instr::Fail)))
}
pub fn make_all() -> Instr{
    Instr::All(vec![])
}
pub fn make_goto(queue_len:usize) -> Instr{
    Instr::GoTo(InstrID::new(get_from_input_valid(&format!("Enter the position you want to go to (from 0 to {}): ", queue_len), "Please enter a valid number!", |x| x < &queue_len)))
}
pub fn make_auto_recipe(cmp:&Components) -> Instr{
    let res = select_recipes_unfiltered(cmp);
    if let Some(res) = res{
        return Instr::PerformRecipe(res.0, res.1);
    } else {
        return Instr::Fail;
    }
}
pub fn make_auto_cmp(cmp:&Components) -> Instr{
    let res = select_components_unfiltered(cmp);
    if let Some(res) = res{
        return Instr::InstallComponent(res.0, res.1);
    } else {
        return Instr::Fail;
    }
}
pub fn make_sticky() -> Instr{
    return Instr::Sticky;
}
pub fn make_end() -> Instr{
    return Instr::End;
}
pub fn make_fail() -> Instr{
    return Instr::Fail;
}
pub fn instrs_menu(sys:&mut Systems, obj:ObjectID, cmp:&Components, rss:&ResourceDict, instrs:&mut Instrs){
    loop{
        refresh();
        println!("Viewing current instructions for {}:", sys.get_o_name(obj));
        print!("{}", ansi::GREEN);
        println!("0. Exit");
        println!("1. Make a new queue");
        println!("2. Delete a queue");
        let len = 3;
        println!("{}", instrs.display(2));
        print!("{}", ansi::RESET);
        let input:usize = get_from_input_valid("", "Please enter a valid input.", |x| *x < (len + instrs.len()));
        match input{
            0=>break,
            1=>make_queue(sys, obj, cmp, rss, instrs),
            2=>{
                println!("{}", instrs.display(0));
                println!("{}. abort", instrs.len());
                let pos:usize = get_from_input_valid("Which queue do you want to remove?", "Please enter a valid input", |x| *x <= instrs.len());
                if pos < instrs.len(){
                    instrs.rmv(pos);
                    println!("Queue successfully removed!");
                } else {
                    println!("Queue removal aborted!");
                }
            }
            _=>{let name = instrs.get_name(input - len); queue_menu(sys, obj, cmp, rss, instrs.get_queue(input - len), name)},
        }
    }
}
pub fn queue_menu(sys:&mut Systems, obj:ObjectID, cmp:&Components, rss:&ResourceDict, queue:&mut Queue, name:String){
    loop{
        refresh();
        println!("Viewing queue {}:", name);
        println!("0. Exit");
        println!("1. Make a new instruction. ");
        println!("2. Remove an instruction. ");
        let len = 3;
        println!("{}", queue.display(len, obj, sys, rss, cmp));
        let input:usize = get_from_input_valid("", "Please enter a valid input.", |x| *x < (len + queue.len()));
        match input{
            0=>break,
            1=>{
                let pos:usize = get_from_input_valid("Where do you want to put the instruction?", "Please enter a valid location!", |x| *x <= queue.len());
                if let Some(val) = make_instr(rss, cmp, sys, obj,queue.len() + 1){
                    queue.ins(val, pos);
                }
            }, 2 => {
                let pos:usize = get_from_input_valid("Which instruction do you want to delete?", "Please enter a valid number (or n+1 to abort)", |x| *x <= queue.len());
                queue.rmv(pos);
            }
            _=>{
                let temp = queue.len();
                instr_menu(rss, cmp, sys, obj, queue.get(input - len), temp)
            }
        }
    }
}
pub fn instr_menu(rss:&ResourceDict, cmp:&Components, sys:&mut Systems, obj:ObjectID, instr:&mut Instr, queue_len:usize){
    loop{
        refresh();
        println!("Viewing {}", instr.display(obj, sys, rss, cmp));
        println!("Options:");
        println!("0. Exit");
        let len = 1;
        println!("{}", display_options(instr, obj, sys, rss, cmp, len));
        let input:usize = get_from_input_valid("", "Please enter a valid input!", |x| *x < get_len(instr, len));
        match input{
            0=>break,
            _=>parse_options(instr, input - len, obj, sys, rss, cmp, queue_len)
        }
    }
}
pub fn display_options(instr:&Instr, obj:ObjectID, sys:&mut Systems, rss:&ResourceDict, cmp:&Components, len:usize) -> String{
    match instr{
        Instr::All(val) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}: Add an instruction to this: \n", len));
            res.push_str(&format!("{}: Remove an instruction from this: \n", len + 1));
            let extra = 2;
            for i in 0..val.len(){
                res.push_str(&format!("{}. {}\n", len + i + extra, val[i].display(obj, sys, rss, cmp)));
            }
            return res;
        }
        Instr::Move(_val) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Change destination\n", len));
            return res;}
        Instr::Jump(_val) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Change system to jump to\n", len));
            return res;}
        Instr::Transfer(_val1,_val22) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Change resources transferred\n", len));
            res.push_str(&format!("{}. Change object destination\n", len + 1));
            return res;}
        Instr::MoveTo(_val) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Change object destination\n", len));
            return res;}
        Instr::If(_val1, _val2,_val33) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Change condition\n", len));
            res.push_str(&format!("{}. Change instruction if true\n", len + 1));
            res.push_str(&format!("{}. Change instruction if false\n", len + 2));
            return res;}
        Instr::GoTo(_val) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Change id that is went to\n", len));
            return res;}
        Instr::PerformRecipe(_val1, _val2) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Replace recipe performed\n", len));
            res.push_str(&format!("{}. Replace amount of times recipe is performed\n", len + 1));
            return res;
        }
        Instr::InstallComponent(_val1, _val2) => {
            let mut res:String = "".to_string();
            res.push_str(&format!("{}. Replace component installed\n", len));
            res.push_str(&format!("{}. Replace amount of times component is installed\n", len + 1));
            return res;
        }
        Instr::Sticky => {"".to_string()}
        Instr::End => {"".to_string()}
        Instr::Fail => {"".to_string()}
    }
}
pub fn get_len(instr:&Instr, len:usize) -> usize{
    match instr{
        Instr::All(val) => 
            val.len() + 2 + len, //based on options
        Instr::Transfer(_, _)| Instr::PerformRecipe(_, _) | Instr::InstallComponent(_, _) => 
            len + 2,
        Instr::If(_, _, _) => 
            len + 3,
        Instr::MoveTo(_) | Instr::GoTo(_) | Instr::Move(_) | Instr::Jump(_) 
        => 
            len + 1,
        Instr::Sticky | Instr::End | Instr::Fail => len,
    }
}
pub fn parse_options(instr:&mut Instr, input:usize,  obj:ObjectID, sys:&mut Systems, rss:&ResourceDict, cmp:&Components, queue_len:usize){
    match instr{
        Instr::All(val)=>{
            match input{
                0=>{
                    let add_num = get_from_input_valid("Enter the position where you want to insert your position:", "Please enter a valid number!", |x| *x <= val.len());
                    let add_val = make_instr(rss, cmp, sys, obj, queue_len);
                    if let Some(v) = add_val{
                        val.insert(add_num, v);
                    }
                },
                1=>{
                    let rmv_num:usize = get_from_input_valid("Enter the position you want to remove:", "Please enter a valid number", |x| *x < val.len());
                    val.remove(rmv_num);
                },
                _=>{
                    instr_menu(rss, cmp, sys, obj, &mut val[input - 1], queue_len);
                },
            }
        }
        Instr::Move(val) => {
            *val = get_location();
        }
        Instr::Jump(val) => {
            *val = get_system(sys);
        }
        Instr::Transfer(_, _) => {

        }
        Instr::MoveTo(val) => {
            *val = get_object(sys, get_system(sys));
        }
        Instr::If(_, _, _) => {

        }
        Instr::GoTo(val) => {
            *val = InstrID::new(get_from_input_valid("Enter the new position", "Please enter a valid input!", |x| *x < queue_len));
        }
        Instr::PerformRecipe(recipe, amt) => {
            match input{
                0=>{
                    if let Some(val) = select_recipe_unfiltered(cmp){
                        *recipe = val;
                    };
                },
                1=>{
                    *amt = get_from_input_valid("Enter the new amount", "Please enter a valid input!", |_| true);
                },
                _=>{}
            }
        }
        Instr::InstallComponent(component, amt) => {
            match input{
                0=>{
                    if let Some(val) = select_component_unfiltered(cmp){
                        *component = val;
                    };
                },
                1=>{
                    *amt = get_from_input_valid("Enter the new amount", "Please enter a valid input!", |_| true);
                },
                _=>{}
            }}
        Instr::Sticky => {}
        Instr::End => {}
        Instr::Fail => {}
    }
}