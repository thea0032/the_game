pub mod condition;
use crate::{component::{ComponentID, Components, RecipeID}, location::*, systems::{object_id::ObjectID, system_id::SystemID}, ui::{ansi}};
use crate::systems::*;
use crate::resources::*;

use crate::instr::condition::Condition;
#[derive(Debug, Clone)]
pub enum Instr{
    Move(Location),//Move to a location. 
    Jump(SystemID),//Jump to another system. 
    Transfer(Vec<u128>, ObjectID),//Transfer resources to another object (moves to it first)
    MoveTo(ObjectID),//Moves to another object
    If(Condition, Box<Instr>, Box<Instr>),//If the condition is true, evaluates the first condition. Otherwise, evaluates the second condition. 
    All(Vec<Instr>),//Does all of these, in order, until a failure or delay. 
    GoTo(InstrID),//Moves to another position on the queue. 
    PerformRecipe(RecipeID, usize),//Performs a recipe a certain number of times. 
    InstallComponent(ComponentID, usize),//Installs a component a certain number of times. 
    Sticky,//Sticks here, doing nothign forever. 
    End,//Immediately goes to the next instruction. 
    Fail,//Fails. 
}//An instruction. Automates the boring parts of this game. 

#[derive(Debug, Clone)]
pub struct InstrID{//Instruction identification wrapper, to make it obvious what the usize will refer to. 
    id:usize
}
impl InstrID{
    pub fn new(id:usize) -> InstrID{
        InstrID{
            id:id
        }
    }//Creates wrapper
    pub fn get(&self) -> usize{
        return self.id;
    }//Simple getter
}
#[derive(Debug, Clone)]
pub enum InstrRes{//Instructions, when executed, return an instruction result. Here's what these results mean:
    Success(usize),//The instruction has finished, and you should go to the next thing. 
    Fail(String),//The instruction has failed, and has an error message. 
    Continue,//The instruction is still in progress, and will continue next tick. 
}
impl Instr{
    //Context required: The object that is performing the instructions, the position in the queue we're in, the system dictionary, the resource dictionary, the component dictionary. 
    pub fn exe(&self, obj:ObjectID, pos:usize, sys:&mut Systems, rss:&ResourceDict, cmp:&Components) -> InstrRes{
        match self{
            Instr::Move(val) =>{//Movement
                if val.eq(sys.get_o(obj).get_location()){//If we're already at the destination...
                    return InstrRes::Success(pos + 1);//We've succeeded! onto the next thing!
                }
                let movement:f64 = sys.get_o(obj).resources().get_curr(crate::resources::constants::MOVEMENT) as f64;//Amount of movement generated
                let mass:f64 = sys.get_o(obj).resources().get_curr(crate::resources::constants::MASS) as f64;//Mass of the object
                let distance = movement / mass;//Distance travelled (this is an Aristotelian universe, where force = mass * velocity)
                sys.get_o(obj).get_location().move_towards(*val, distance);//Moves towards the location
                sys.get_o(obj).resources_mut().change_amt(crate::resources::constants::MOVEMENT, 0);//Resets the movement generated to zero
                if (*sys.get_o(obj).get_location()).eq(val){//If we got there...
                    return InstrRes::Success(pos + 1);//We've succeeded! Onto the next thing!
                }
                return InstrRes::Continue;//We haven't gotten there yet!
            },
            Instr::GoTo(val) => {
                return InstrRes::Success(val.id);//We've succeeded, and we're going to the location specified by the instruction!
            },
            Instr::If(val1, val2, val3)=>{
                if val1.eval(sys){//Evaluate the condition! If it's true...
                    return val2.exe(obj, pos, sys, rss, cmp);//Execute the first instruction and return the result.
                } else { //Otherwise... 
                    return val3.exe(obj, pos, sys, rss, cmp);//Execute the second instruction and return the result.
                }
            },
            Instr::All(val)=>{
                let mut saved_pos:usize = pos + 1;
                for instr in val{//For every instruction...
                    match instr.exe(obj, pos, sys, rss, cmp){//Execute the instruction. 
                        InstrRes::Fail(val)=>return InstrRes::Fail(val),//If it fails, return the result. 
                        InstrRes::Continue=>return InstrRes::Continue,//If it's incomplete, return the result generated. 
                        InstrRes::Success(val)=>{saved_pos = val;},//If we've succeeded, store the value. 
                    }
                }
                return InstrRes::Success(saved_pos);
                //Returns the last stored position (which makes gotos work); eg:
                //0. Install a component
                //1. move to (0, 0)
                //2. move to (3, 3)
                //3. go to position 1
                //This would give us a turn doing nothing. 
                //Returning the last value allows us to do this:
                //0. Install a component
                //1. move to (0, 0)
                //2. do all these: [move to (3, 3), AND go to position 1]
                //No turn is wasted here. 
                //NOTE: You could also install the component manually and then do this:
                //0. move to (0, 0)
                //1. move to (3, 3)
                //After position 0, we automatically go to position 1. After executing 1, we automatically go to 2, which rounds back to 0. 
            }
            Instr::Jump(val)=>{//Jumps to a different system: 
                if val.get() == sys.get_o_sys(obj).get(){//If we're already in the right system...
                    return InstrRes::Success(pos + 1); //Success!
                }
                return InstrRes::Fail("Jumping to another system hasn't been implemented yet!".to_string());//Jumping isn't implemented yet. 
            },
            Instr::MoveTo(val)=>{//Moves to another object. 
                match Instr::Jump(sys.get_o_sys(*val)).exe(obj, pos, sys, rss, cmp){//Starts by jumping to the system the object is in. 
                    InstrRes::Continue=> return InstrRes::Continue,//If it's in progress, return the result and wait. 
                    InstrRes::Fail(val)=>return InstrRes::Fail(val),//If it failed, we return the same failure. 
                    InstrRes::Success(_)=>{},//If we succeeded, continue on. 
                }
                Instr::Move(*sys.get_o(*val).get_location()).exe(obj, pos, sys, rss, cmp)//We move to the object's location. 
            },
            Instr::Transfer(val1, val2)=>{//Transfers resources to another object. 
                let res = Instr::MoveTo(*val2).exe(obj, pos, sys, rss, cmp);//Moves to the object. 
                match res{
                    InstrRes::Fail(val)=>return InstrRes::Fail(val),//If we fail, fail. 
                    InstrRes::Success(_)=>{},//If we succeed, continue on in the function. 
                    InstrRes::Continue=>return InstrRes::Continue,//If we aren't done, continue moving instead. 
                }
                let mut temp = rss.get_transfer_costs().iter();//Generates transfer cost. 
                let transfer_cap_cost:u128 = val1.iter().map(|x| x * temp.next().unwrap()).sum();//Sums transfer costs up. 
                let mut total_cost = val1.clone();//Generates a clone, that we can manipulate. 
                total_cost[crate::resources::constants::TRANSFER.get()] += transfer_cap_cost;//Adds the cost of transferring resources on. 
                if !sys.get_o(obj).resources_mut().spend_unsigned(&total_cost){//Attempts to spend the resources. If it fails...
                    return InstrRes::Fail("Not enough resources!".to_string());//fail!
                }
                sys.get_o(*val2).resources_mut().gain_unsigned(val1);//Otherwise, gain extra resources. 
                return InstrRes::Success(pos + 1);//We've succeeded!
            }
            Instr::Sticky => return InstrRes::Continue,//Sticks to the instruction
            Instr::End => return InstrRes::Success(pos + 1),//immediately advances
            Instr::Fail => return InstrRes::Fail("This instruction was supposed to fail.".to_string()),//Fails
            Instr::PerformRecipe(recipe, amt) => {//Performs a recipe. 
                let amt_success = sys.get_o(obj).do_recipes(*recipe, cmp, *amt);//Performs recipes, gets amount of successes. 
                if &amt_success == amt{//If we did all of them...
                    return InstrRes::Success(pos + 1);//We've succeeded!
                } else {
                    return InstrRes::Fail(format!("We only had enough resources to do {} out of {} recipes", amt_success, amt));//We've failed. 
                }
            },
            Instr::InstallComponent(component, amt)=>{//Installs a component. 
                let amt_success = sys.get_o(obj).install_components(*component, cmp, *amt);//Installs components, gets amount of successes. 
                if &amt_success == amt{//If we did all of them...
                    return InstrRes::Success(pos + 1);//We've succeeded!
                } else {
                    return InstrRes::Fail(format!("We only had enough resources to install {} out of {} components", amt_success, amt));//We've failed. 
                }
            },
        }
    }//Executes instructions. 
    pub fn display(&self, obj:ObjectID, sys:&Systems, rss:&ResourceDict, cmp:&Components) -> String{
        match self{
            Instr::All(val)=>{
                let mut res:String = "Do all: [".to_string();
                for line in val{
                    res.push_str(&line.display(obj, sys, rss, cmp));
                    res.push_str(", ");
                }
                res.pop();
                res.push(']');
                return res;
            }
            Instr::Move(val) => {
                format!("Move from ({}, {}) to ({}, {})", sys.get_o_stat(obj).get_location_stat().x, sys.get_o_stat(obj).get_location_stat().y, val.x, val.y)
            }
            Instr::Jump(val) => {format!("Jumping from {} to {}", sys.get_o_sys(obj).get(), val.get())}
            Instr::Transfer(val1, val2) => {
                format!("Transfer {} to {}", crate::resources::display_vec_one(rss, val1, ", "), sys.get_o_name(*val2))
            }
            Instr::MoveTo(val) => {
                format!("Move to {}", sys.get_o_name(*val))
            }
            Instr::If(val1, val2, val3) => {
                format!("If [{}], then [{}] else [{}]", val1.display(), val2.display(obj, sys, rss, cmp), val3.display(obj, sys, rss, cmp))
            }
            Instr::GoTo(val) => {
                format!("Jump to instruction {}", val.get())
            }
            Instr::PerformRecipe(val1, val2) => {
                format!("Perform recipe {} {} times", cmp.get_r_name(*val1), val2)
            }
            Instr::InstallComponent(val1, val2) => {
                format!("Installing component {} {} times", cmp.get_name(*val1), val2)}
            Instr::Sticky => "Remain here".to_string(),
            Instr::End => "Advance".to_string(),
            Instr::Fail => "Fail".to_string(),
        }
    }//Displays instructions. Shouls be simple enough. 
}
#[derive(Debug, Clone)]
pub struct Queue{
    queue:Vec<Instr>,//The instructions themselves. 
    delete_after_exe:bool,//Whether we delete instructions after they execute. 
    curr:usize,//The current instruction. 
    last:usize,//The previous instruction. 
    last_res:InstrRes,//The previous result. 
    flag:Option<usize>,//A flag. 
}//A queue of instructions. 
#[derive(Debug, Clone)]
pub enum QueueRes{
    Completion,//This queue has finished, and is ready to be deleted. 
    Fail(String),//This queue has failed. 
    Continue,//This queue is in progress. 
}//The result of a queue's execution. 
impl Queue{
    pub fn exe(&mut self, obj:ObjectID, sys:&mut Systems, rss:&ResourceDict, cmp:&Components) -> QueueRes{
        if let Some(mut new) = self.flag{//If this flag has triggered...
            self.flag = None;//Reset the flag. 
            if self.delete_after_exe{//If we're deleting after executing...
                self.queue.remove(self.curr);//Removes the current instruction. 
                if new > self.curr{
                    new -= 1;//Decreases the value in the flag by 1 to compensate for the removed instruction. 
                }
            }
            self.curr = new;//Resets the current position. 
        }
        if self.queue.len() == 0{//If the queue's length is zero...
            return QueueRes::Completion;//The queue is done. 
        }
        self.curr = self.curr % self.queue.len();//Rounds off any invalid positions

        let res = self.queue[self.curr].exe(obj, self.curr, sys, rss, cmp);//Performs the instruction at the current location. 
        self.last_res = res.clone();//Sets the last result variable. 
        let placeholder = self.curr;//Sets a placeholder. 
        let ret_val = match res{
            InstrRes::Success(new) => {
                self.flag = Some(new);//If we've succeeded in an instruction, we place a flag down. 
                QueueRes::Continue//The queue isn't done yet. 
            }
            InstrRes::Continue=>QueueRes::Continue,//If the instruction isn't done, the queue isn't done. 
            InstrRes::Fail(val)=>QueueRes::Fail(val),//If the instruction fails, the queue fails. 
        };
        if placeholder != self.curr{//If the current location changed... 
            self.last = placeholder;//Sets the last location to the placeholder. 
        }
        return ret_val;//returns the value we should return. 
    }
    pub fn new(delete_after_exe:bool, first_instr:Instr) -> Queue{
        return Queue{
            delete_after_exe:delete_after_exe,
            curr:0,
            last:0,
            queue:vec![first_instr],
            last_res:InstrRes::Continue,
            flag:None,
        }
    }//Creates a new queue. 
    pub fn ins(&mut self, instr:Instr, pos:usize){
        self.queue.insert(pos, instr);
    }//Adds a new instruction to the queue. 
    pub fn rmv(&mut self, pos:usize){
        self.queue.remove(pos);
    }//Removes an instruction from the queue. 
    pub fn color(&self) -> &str{
        match self.last_res{
            InstrRes::Continue=>ansi::BLUE,
            InstrRes::Fail(_)=>ansi::RED,
            InstrRes::Success(_)=>ansi::GREEN,
        }
    }//Returns the color of the queue (used to help the user tell which queues have failed and which haven't)
    pub fn display(&self, amt_before:usize, obj:ObjectID, sys:&mut Systems, rss:&ResourceDict, cmp:&Components) -> String{
        let mut res = "".to_string();//Initializes result
        for i in 0..self.queue.len(){
            res.push_str(&format!("{}{}: {}", self.color_instr(i), i + amt_before, self.queue[i].display(obj, sys, rss, cmp)));
            if let InstrRes::Fail(val) = &self.last_res{
                res.push_str(&format!("(FAILED: {})\n", val));
            } else {
                res.push('\n');
            }
        }
        return res;
    }//Displays the queue. amt_before allows it to fit neatly 
    pub fn color_instr(&self, pos:usize) -> &str{
        if pos == self.curr{//The current instruction is colored based on this:
            match self.last_res{
                InstrRes::Continue=>{
                    return ansi::CYAN;//In-progress stuff is colored cyan. 
                },
                InstrRes::Fail(_)=>{
                    return ansi::RED;//Failed stuff is colored red. 
                },
                InstrRes::Success(_)=>{
                    return ansi::GREEN;//Succeeded stuff is colored green. 
                },
            }
        }
        if pos == self.last{//The last instruction is colored yellow. 
            return ansi::YELLOW;
        }
        return ansi::RESET;//All other instructions are colored white. 
    }//Returns the color of the instruction. 
    pub fn len(&self) -> usize{
        return self.queue.len();
    }//Returns the queue's length. 
    pub fn get<'a>(&'a mut self, pos:usize) -> &'a mut Instr{
        return &mut self.queue[pos];
    }//Returns the instruction at the position given. 
}
#[derive(Debug, Clone)]
pub struct Instrs{
    instrs:Vec<Queue>,//The queues. 
    names:Vec<String>,//The names of the queues. 
}//A vector of queues, basically.  
impl Instrs{
    pub fn exe(&mut self, obj:ObjectID, sys:&mut Systems, rss:&ResourceDict, cmp:&Components){
        let mut will_remove:Vec<bool> = vec![];//Whether we should remove the queues. 
        for instr in &mut self.instrs{//For every queue...
            if let QueueRes::Completion = instr.exe(obj, sys, rss, cmp){//Executes the queues. If they're complete... 
                will_remove.push(true);//Markes them to be deleted. 
            } else {
                will_remove.push(false);//Otherwise, marks them not to be deleted. 
            }
        }
        let mut i = will_remove.len();
        while i > 0{//Deconstructs the vector in reverse order. 
            i -= 1;
            if will_remove[i]{//If marked for removal...
                self.rmv(i);//Removes the queue. 
            }
        }
    }
    pub fn new() -> Instrs{
        Instrs{
            instrs:vec![],
            names:vec![],
        }
    }
    pub fn add(&mut self, queue:Queue, name:String){
        self.instrs.push(queue);
        self.names.push(name);
    }//Adds a queue and name
    pub fn rmv(&mut self, index:usize){
        self.instrs.remove(index);
        self.names.remove(index);
    }//Removes a queue and name
    pub fn get_queue<'a>(&'a mut self, id:usize) -> &'a mut Queue{
        return &mut self.instrs[id];
    }//Gets the queue based on the position
    pub fn len(&self) -> usize{
        return self.instrs.len();
    }//Gets the length. 
    pub fn display(&self, amt_before:usize) -> String{
        let mut res = "".to_string();
        for i in 0..self.instrs.len(){
            res.push_str(&format!("{}{}: {}\n", self.instrs[i].color(), i + amt_before, self.names[i]));
        }
        return res;
    }//Displays the object. 
    pub fn get_name(&self, id:usize) -> String{
        return self.names[id].clone();
    }//Gets the name. 
}
pub struct Quickie{
    dirs:Vec<Instr>,//The directions
    res:Vec<InstrRes>,//The last results of the instruction
    del:Vec<bool>,//Whether these will be deleted after they're finished or failed. 
}//A quick instruction storage. Made out of instructions, not queues, for easy access. 
impl Quickie{
    pub fn new() -> Quickie{
        Quickie{
            dirs:vec![],
            res:vec![],
            del:vec![],
        }
    }//Initializes the structure
    pub fn exe(&mut self, obj:ObjectID, sys:&mut Systems, rss:&ResourceDict, cmp:&Components){
        let mut will_remove:Vec<bool> = vec![];//Whether to remove the instructions
        for i in 0..self.dirs.len(){
            if self.del[i]{//If these are marked to be deleted...
                if let InstrRes::Continue = self.res[i]{//If the instruction is still going...
                } else {//Otherwise...
                    will_remove.push(true);//Marks index for removal
                    continue;
                }
            }
            let new_res = self.dirs[i].exe(obj, 0, sys, rss, cmp);
            self.res[i] = new_res;//Updates the result
            will_remove.push(false);
        }
        let mut i = will_remove.len();//See a similar process in instrs. 
        while i > 0{
            i -= 1;
            if will_remove[i]{
                self.rmv(i);
            }
        }
    }//Execution
    pub fn rmv(&mut self, index:usize){
        self.dirs.remove(index);
        self.res.remove(index);
        self.del.remove(index);
    }//Removes a certain index
    pub fn ins(&mut self, index:usize, instr:Instr, del:bool){
        self.dirs.insert(index, instr);
        self.res.insert(index, InstrRes::Continue);
        self.del.insert(index, del);
    }//Adds a new function
    pub fn display(&self, amt_before:usize, obj:ObjectID, sys:&Systems, rss:&ResourceDict, cmp:&Components) -> String{
        let mut res:String = "".to_string();//Initializes result
        let mut i = 0;//Counter variable
        for line in &self.dirs{
            res.push_str(&format!("{}{}. {}", self.color(i), i + amt_before, line.display(obj, sys, rss, cmp)));//Adds a few things
            if self.del[i]{//If it's temporary...
                res.push_str(" (temp)");//helpful text
            } else {
                res.push_str(" (perm)");//helpful text
            }
            if let InstrRes::Fail(val) = &self.res[i]{
                res.push_str(&format!("(FAILED: {})", val));//If it's failed, more helpful text
            }
            res.push('\n');//newline character for formatting
            i += 1;
        }
        return res;
    }//Displays the quick queue
    pub fn color(&self, i:usize) -> &str{
        match self.res[i]{
            InstrRes::Continue=>ansi::BLUE,
            InstrRes::Fail(_)=>ansi::RED,
            InstrRes::Success(_)=>ansi::GREEN,
        }
    }//Gives it a color based on the index and result
    pub fn len(&self) -> usize{
        return self.dirs.len();
    }//Returns the length
    pub fn get<'a>(&'a mut self, i:usize) -> &'a mut Instr{
        return &mut self.dirs[i];
    }//Gets a certain index
}
pub struct Directions{
    directions:Vec<Instrs>,
    quick:Vec<Quickie>
}//Each position corresponds to an object. 
impl Directions{
    pub fn new() -> Directions{
        Directions{
            directions:vec![],
            quick:vec![],
        }
    }//Basic new function
    pub fn directions<'a>(&'a mut self) -> &'a mut Vec<Instrs>{
        return &mut self.directions;
    }///Returns directions
    pub fn instrs<'a>(&'a mut self, id:ObjectID) -> &'a mut Instrs{
        return &mut self.directions[id.get()];
    }//Returns instruction vector at a certain object id
    pub fn add_new(&mut self){
        self.directions.push(Instrs::new());
        self.quick.push(Quickie::new());
    }//Adds a new instruction queue; corresponds with object creation
    pub fn quickie<'a>(&'a mut self, id:ObjectID) -> &'a mut Quickie{
        return &mut self.quick[id.get()];
    }//Returns the corresponding quick queue
    pub fn quickies<'a>(&'a mut self) -> &'a mut Vec<Quickie>{
        return &mut self.quick;
    }//Returns all of the quick queues. 
}