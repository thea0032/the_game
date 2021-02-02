use crate::resources::*;
use crate::ui::io::*;

use super::{ansi, clipboard::Clipboard, config::Config, select::generic_select};
pub fn get_resource_filtered(rss: &ResourceDict, amts: &Vec<u64>, cfg: &mut Config) -> Option<ResourceID> {
    let filter: Vec<bool> = amts.iter().map(|x| x != &0).collect(); //If the option exists
    let len = filter.iter().filter(|x| **x).count(); //Amount of options
    generic_select(
        &rss.display_filtered_addon(&filter, amts),
        len,
        |x| Some(ResourceID::new(x)),
        cfg,
        |x| if let Clipboard::Resource(val) = x { Some(*val) } else { None },
    )
}
pub fn get_transfer_max(rss: &ResourceDict, cap: u64) -> Vec<u64> {
    let costs = rss.get_transfer_costs();
    let mut res = Vec::new(); //Initializes result
    for i in costs {
        if *i == 0 {
            res.push(u64::MAX); //No max
        } else {
            res.push(cap / i); //Maximum
        }
    }
    res //Returns the result
} //Gets the maximum amount of resources you can transfer
pub fn get_rss(rss: &ResourceDict, input: &mut Vec<u64>, cfg: &mut Config) {
    println!("You will now be directed to specify an amount of each resource to be used.");
    loop {
        println!("Select a resource to modify it. ");
        println!("{}", display_rss(rss, &input, 1));
        println!("{}{}. Quit and exit. ", ansi::RED, rss.len()); //Quit option
        let i: usize = get_from_input_valid("", "Please enter a valid number!", cfg, |x| x <= &input.len()); //Gets resource (or quit and exit)
        if i == rss.len() {
            break;
        } //Breaks
        input[i] = get_from_input("What do you want to change this field to?", "Please enter a number!", cfg); //Gets new amount
    }
} //Grabs a resources object from input
pub fn display_rss(rss: &ResourceDict, v: &Vec<u64>, s: usize) -> String {
    let mut res = "".to_string(); //Initializes result
    for (i, item) in v.iter().enumerate() {
        res.push_str(&format!("{}: {} ({})\n", i + s, rss.get(ResourceID::new(i)), item)); //Adds resource
    }
    res //Returns result
}
