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
pub fn get_rss(rss:&ResourceDict, input:&mut Vec<u128>){
    println!("You will now be directed to specify an amount of each resource to be used.");
    loop{
        println!("Select a resource to modify it. ");
        println!("0. Quit and exit. ");
        println!("{}", display_rss(rss, &input, 1));
        let i:usize = get_from_input_valid("", "Please enter a valid number!", |x| x <= &input.len());
        if i == 0{
            break;
        }
        input[i - 1] = get_from_input("What do you want to change this field to?", "Please enter a number!");
    }
}
pub fn display_rss(rss:&ResourceDict, v:&Vec<u128>, s:usize) -> String{
    let mut res = "".to_string();
    for i in 0..v.len(){
        res.push_str(&format!("{}: {} ({})\n", i + s, rss.get(ResourceID::new(i)), v[i]));
    }
    return res;
}