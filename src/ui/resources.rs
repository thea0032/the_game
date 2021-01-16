use crate::{extra_bits, ui::io::*};
use crate::{resources::*, object::*};

use super::ansi;
pub fn get_resource_filtered(rss:&ResourceDict, amts:&Vec<u128>) -> Option<ResourceID>{
    let filter:Vec<bool> = amts.iter().map(|x| x != &0).collect();
    let len = filter.iter().filter(|x| **x).count();
    println!("{}", rss.display_filtered_addon(&filter, amts));
    println!("{}{}. Quit{}", ansi::RED, len, ansi::RESET);
    let input:usize = get_from_input_valid("Enter the resource you want: ", "Please enter a valid id", |x| x <= &len);
    if input == len{
        return None;
    }
    return Some(ResourceID::new(crate::extra_bits::filter(input, &filter)));
}
pub fn get_amt(rss:&ResourceDict, obj:&Object, id:ResourceID) -> u128{
    let max:u128 = obj.resources().get_curr(id);
    let entered = get_from_input_valid(
    &format!("Enter the amount of {} you want (0 to {}):", &rss.get(id), max), 
    "Please enter a valid number!", |x| x <= &max);
        return entered;
}
pub fn get_transfer_max(rss:&ResourceDict, cap:u128) -> Vec<u128>{
    let costs = rss.get_transfer_costs();
    let mut res = vec![];
    for i in 0..costs.len(){
        if costs[i] == 0{
            res.push(u128::MAX);
        } else {
            res.push(cap / costs[i]);
        }
    }
    return res;
}
pub fn get_rss(rss:&ResourceDict) -> Vec<u128>{
    let mut res = extra_bits::fill(rss.len(), 0);
    refresh();
    println!("You will now be directed to specify an amount of each resource to be used.");
    loop{
        println!("Select a resource to modify it. ");
        println!("0. Quit and exit. ");
        println!("{}", display_rss(rss, &res, 1));
        let input:usize = get_from_input_valid("", "Please enter a valid number!", |x| x <= &res.len());
        if input == 0{
            break;
        }
        res[input - 1] = get_from_input("What do you want to change this field to?", "Please enter a number!");
    }
    return res;
}
pub fn display_rss(rss:&ResourceDict, v:&Vec<u128>, s:usize) -> String{
    let mut res = "".to_string();
    for i in 0..v.len(){
        res.push_str(&format!("{}: {} ({})\n", i + s, rss.get(ResourceID::new(i)), v[i]));
    }
    return res;
}