use crate::resources::*;
use crate::{extra_bits, ui::io::*};

use super::ansi;
pub fn get_resource_filtered(rss: &ResourceDict, amts: &Vec<u128>) -> Option<ResourceID> {
    let filter: Vec<bool> = amts.iter().map(|x| x != &0).collect(); //If the option exists
    let len = filter.iter().filter(|x| **x).count(); //Amount of options
    println!("{}", rss.display_filtered_addon(&filter, amts));
    println!("{}{}. Quit{}", ansi::RED, len, ansi::RESET);
    let input: usize = get_from_input_valid(
        "Enter the resource you want: ",
        "Please enter a valid id",
        |x| x <= &len,
    ); //gets input
    if input == len {
        return None; //quit option
    }
    Some(ResourceID::new(extra_bits::filter(input, &filter)))//Returns
                                                                      // the resource
}
pub fn get_transfer_max(rss: &ResourceDict, cap: u128) -> Vec<u128> {
    let costs = rss.get_transfer_costs();
    let mut res = vec![]; //Initializes result
    for i in costs{
        if *i == 0 {
            res.push(u128::MAX); //No max
        } else {
            res.push(cap / i); //Maximum
        }
    }
    res //Returns the result
} //Gets the maximum amount of resources you can transfer
pub fn get_rss(rss: &ResourceDict, input: &mut Vec<u128>) {
    println!("You will now be directed to specify an amount of each resource to be used.");
    loop {
        println!("Select a resource to modify it. ");
        println!("{}", display_rss(rss, &input, 1));
        println!("{}{}. Quit and exit. ", ansi::RED, rss.len()); //Quit option
        let i: usize =
            get_from_input_valid("", "Please enter a valid number!", |x| x <= &input.len()); //Gets resource (or quit and exit)
        if i == rss.len() {
            break;
        } //Breaks
        input[i] = get_from_input(
            "What do you want to change this field to?",
            "Please enter a number!",
        ); //Gets new amount
    }
} //Grabs a resources object from input
pub fn display_rss(rss: &ResourceDict, v: &Vec<u128>, s: usize) -> String {
    let mut res = "".to_string(); //Initializes result
    for (i, item) in v.iter().enumerate() {
        res.push_str(&format!(
            "{}: {} ({})\n",
            i + s,
            rss.get(ResourceID::new(i)),
            item
        )); //Adds resource
    }
    res //Returns result
}
