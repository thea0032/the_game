pub mod constants;
pub mod dict;

use std::{fmt::Display};

use crate::{extra_bits::fill, ui::{ansi}};
#[derive(Clone, Debug)]
pub struct Resources{
    curr:Vec<u128>,//The amount of resources here
    surplus:Vec<i64>,//The current amount increases (or decreases) by this much each tick. 
    cap:Vec<u128>//Every resource is reduced to its cap at the end of each tick. 
}//Resources
impl Resources{
    pub fn new(len:usize) -> Resources{
        Resources{
            curr:fill(len, 0),
            surplus:fill(len, 0),
            cap:fill(len, 0),
        }
    }//Basic new function
    pub fn tick(&mut self) -> Vec<bool>{
        let mut res:Vec<bool> = vec![];//Initializes result
        for i in 0..self.curr.len(){//For every resource...
            if self.curr[i] > self.cap[i]{//If we have more resources than we have capacity for...
                self.curr[i] = self.cap[i];//Delete the extra resources
            }
            if self.surplus[i] < 0 && self.curr[i] >= (self.surplus[i] * -1) as u128{//If we have a negative surplus, but we can still lose a few...
                self.curr[i] -= (-1 * self.surplus[i]) as u128;//Do it
                res.push(false);//We didn't run out of resources yet...
            } else if self.surplus[i] >= 0{//If we have positive (or zero) surplus...
                self.curr[i] += self.surplus[i] as u128;
                res.push(false);//We didn't run out of resource, obviously!
            } else {
                res.push(true);//We ran out!
            }
        }
        return res;
    }//The tick function. For each resource, returns true if 
    pub fn get_curr(&self, id:ResourceID) -> u128{
        return self.curr[id.get()];
    }//Gets the current amount of this resource
    pub fn get_currs<'a>(&'a self) -> &'a Vec<u128>{
        return &self.curr;
    }//Gets the current amounts of all resources
    pub fn get_cap(&self, id:ResourceID) -> u128{
        return self.cap[id.get()];
    }//Gets the current storage of this resource
    pub fn spend(&mut self, other:&Vec<i64>) -> bool{
        for i in 0..self.curr.len(){//For every resource...
            if (self.curr[i] as i64) < other[i] {//If we can't spend this resource...
                return false;//We can't do this operation. 
            }
        }
        for i in 0..self.curr.len(){//Performs the operation
            if other[i] >= 0 {
                self.curr[i] -= other[i] as u128;
            } else {
                self.curr[i] += (other[i] * -1 ) as u128;
            }
        }
        return true;//We did this operation. 
    }//Attempts to spend these resources. Returns true if the operation was successful. 
    pub fn spend_unsigned(&mut self, other:&Vec<u128>) -> bool{
        for i in 0..self.curr.len(){//Same logic as above
            if self.curr[i] < other[i] {
                return false;
            }
        }
        for i in 0..self.curr.len(){
            self.curr[i] -= other[i];
        }
        return true;
    }//Attempts to spend these resources. Returns true if the operation was successful. 
    pub fn amt_contained(&self, other:&Vec<i64>) -> usize{
        let mut min = usize::MAX;//Defaults to the maximum value possible
        for i in 0..other.len(){//For every resource...
            if other[i] <= 0{
                continue;//If it doesn't cost anything, or gives you something, we skip it. 
            }
            let min_amt:usize = (self.curr[i] as usize / other[i] as usize) as usize;//Division by zero is impossible, by the way. Calculates the number of times you can buy a component. 
            if min_amt < min{
                min = min_amt;//Updates the mimimum value
            }
        }
        return min;//Returns the value
    }//Returns the amount of times you can spend other. 
    pub fn force_spend(&mut self, other:&Vec<i64>){
        for i in 0..self.curr.len(){
            if (self.curr[i] as i64) < other[i] {
                self.curr[i] = 0;//sets value to zero
            } else {
                if other[i] >= 0 {
                    self.curr[i] -= other[i] as u128;
                } else {
                    self.curr[i] += (other[i] * -1 ) as u128;
                }
            }
        }
    }//Forceful spending. Exactly the same as spending, but no check
    pub fn gain(&mut self, other:&Vec<i64>) -> bool{
        for i in 0..self.curr.len(){
            if (self.curr[i] as i64) < other[i] * -1 {
                return false;
            }
        }
        for i in 0..self.curr.len(){
            if other[i] >= 0 {
                self.curr[i] += other[i] as u128;
            } else {
                self.curr[i] -= (other[i] * -1) as u128;
            }
        }
        return true;
    }//Like spend, but it's a negative version. 
    pub fn gain_unsigned(&mut self, other:&Vec<u128>){
        for i in 0..self.curr.len(){
            self.curr[i] += other[i] as u128;
        }
    }//Gain the values inputted. They're positive, so checks aren't required. 
    pub fn add_surplus_vec(&mut self, other:&Vec<i64>){
        for i in 0..self.curr.len(){
            self.surplus[i] += other[i];
        }
    }//Gain the values inputted to surplus. 
    pub fn add_storage_vec(&mut self, other:&Vec<u128>){
        for i in 0..self.curr.len(){
            self.cap[i] += other[i];
        }
    }//Add the values inputted to storage. 
    pub fn rmv_surplus_vec(&mut self, other:&Vec<i64>){
        for i in 0..self.curr.len(){
            self.surplus[i] -= other[i];
        }
    }//Same as add_surplus_vec, but negative. 
    pub fn rmv_storage_vec(&mut self, other:&Vec<u128>) -> bool{
        for i in 0..self.curr.len(){
            if self.cap[i] < other[i]{
                return false;
            }
        }
        for i in 0..self.curr.len(){
            self.cap[i] -= other[i];
        }
        return true;
    }//Spend_unsigned, but removes values from storage instead. 
    pub fn can_rmv_storage_vec(&mut self, other:&Vec<u128>) -> bool{
        for i in 0..self.curr.len(){
            if self.cap[i] < other[i]{
                return false;
            }
        }
        return true;
    }//rmv_storage_vec, but just checks if it's possible. 
    pub fn add_res(&mut self, id:ResourceID, qty:u128){
        self.curr[id.get()] += qty;
    }//Adds a certain amount of a certain resource to the resources. 
    pub fn rmv_res(&mut self, id:ResourceID, qty:u128) -> bool{
        if self.curr[id.get()] < qty{
            return false;
        } else {
            self.curr[id.get()] -= qty;
            return true;
        }
    }//Tries to remove a certain amount of a certain resource from the resources. Returns true if it worked. 
    pub fn rmv_res_force(&mut self, id:ResourceID, qty:u128){
        if self.curr[id.get()] < qty{
            self.curr[id.get()] = 0;
        } else {
            self.curr[id.get()] -= qty;
        }
    }//Forcefully removes a resoure. 
    pub fn display(&self, rss:&ResourceDict, prev:&Resources) -> String{
        let mut res:String = "".to_string();//Initializes result
        res.push_str(&self.display_func_new(rss, "Current resources: ", &self.curr.iter().map(|x| *x as i128).collect(), 0, -1, &prev.curr.iter().map(|x| *x as i128).collect()));
        res.push_str(&self.display_func_new(rss, "Projected surplus: ", &self.surplus.iter().map(|x| *x as i128).collect(), 0, -1, &prev.surplus.iter().map(|x| *x as i128).collect()));
        res.push_str(&self.display_func_new(rss, "Storage: ", &self.cap.iter().map(|x| *x as i128).collect(), 0, -1, &prev.cap.iter().map(|x| *x as i128).collect()));
        return res;//Adds 3 lines, returns it. 
    }//A basic display function
    pub fn display_func_new(&self, rss:&ResourceDict, msg:&str, a:&Vec<i128>, zero:i128, max:i128, prev:&Vec<i128>) -> String {
        let mut x:String = "".to_string();//Initializes result
        let mut flag:bool = false;//A flag for if resources exist in this place
        x.push_str(msg);//Adds the inputted message onto the result. 
        for i in 0..a.len(){
            if a[i] != zero && a[i] != max{//If this resource should be displayed...
                flag = true;//We've displayed at least one resource
                let diff = a[i] - prev[i];//Calculates difference
                if diff > zero{//If we have a positive difference
                    x.push_str(ansi::GREEN);//The color is green
                } else if diff == zero {//If we have no difference
                    x.push_str(ansi::YELLOW);//The color is yellow
                } else {//If we have a negative difference
                    x.push_str(ansi::RED);//The color is red
                }
                x.push_str(&a[i].to_string());//Adds the number
                x.push(' ');//space
                x.push_str(&rss.names[i]);//name
                x.push(' ');
                if diff >= zero{
                    x.push('(');
                    x.push('+');
                    x.push_str(&diff.to_string());//(+val)
                    x.push(')');
                } else {
                    x.push('(');
                    x.push_str(&diff.to_string());//(-val)
                    x.push(')');
                }
                x.push(',');
                x.push(' ');
            }
        }
        if !flag{//If no resources were displayed
            x.push_str("N/A");//N/A
        } else {
            x.pop();//Removes a comma
        }
        x.push('\n');//newline character
        x.push_str(ansi::RESET);//Resets our ansi
        return x;//Returns the string
    }
    pub fn change_amt(&mut self, id:ResourceID, new_amt:u128){
        self.curr[id.get()] = new_amt;
    }//Basic functions; self-explanatory
}

#[derive(Clone, Debug, Copy)]
pub struct ResourceID{
    id:usize
}//Resource identification wrapper; to make code cleaner
impl ResourceID{
    pub const fn new(id:usize) -> ResourceID{
        ResourceID{id:id}
    }//basic new function
    pub fn get(&self) -> usize{
        return self.id;
    }//basic get function
}
#[derive(Clone, Debug)]
pub struct ResourceDict{
    names:Vec<String>,
    transfer_costs:Vec<u128>,
}//Resource dictionary; contains helpful information
pub fn display_vec_one(rss:&ResourceDict, amts:&Vec<u128>, sep:&str) -> String{
    let mut res = "".to_string();//Initializes result
    for i in 0..amts.len(){
        if amts[i] == 0{
            continue;
        }
        res.push_str(&amts[i].to_string());//45
        res.push(' ');// 
        res.push_str(&rss.get(ResourceID::new(i)));//energy
        res.push_str(sep);//, 
    }
    if res.len() != 0{
        for _ in 0..sep.len(){
            res.pop();//Removes the last separator 
        }
    }
    return res;
}