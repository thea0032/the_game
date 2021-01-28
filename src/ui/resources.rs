use crate::resources::*;
use crate::{extra_bits, ui::io::*};

use super::{ansi, config::Config, from_str::{InBounds, MenuRes}};
pub fn get_resource_filtered(rss: &ResourceDict, amts: &Vec<u128>, cfg: &mut Config) -> Option<ResourceID> {
    let filter: Vec<bool> = amts.iter().map(|x| x != &0).collect(); //If the option exists
    let len = filter.iter().filter(|x| **x).count(); //Amount of options
    loop{
        let mut ctx = cfg.generate_context();
        let mut dis = cfg.generate_display();
        cfg.update_context_all(&mut dis);
        cfg.update_context(Config::PASTE, Some("paste".to_string()), &mut ctx, &mut dis);
        cfg.update_context(Config::QUIT, Some("abort".to_string()), &mut ctx, &mut dis);
        println!("{}", cfg.display(&ctx, &dis));
        println!("{}", rss.display_filtered_addon(&filter, amts));
        let input: MenuRes = get_from_input_valid("Enter the resource you want: ", "Please enter a valid id", cfg, |x:&MenuRes| x.in_bounds(&len)); //gets input
        match input{
            MenuRes::Enter(val) => {
                return Some(ResourceID::new(extra_bits::filter(val, &filter)));
            },
            MenuRes::Exit | MenuRes::Del => {
                return None;
            },
            MenuRes::Paste => {
                match cfg.cpb{
                    super::clipboard::Clipboard::Resource(val) => {
                        if filter[val.get()]{
                            return Some(val);
                        } else {
                            wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg);
                        }
                    }
                    _ => wait_for_input(&format!("{}You can't paste that there!", ansi::RED), cfg)
                }
            }
            _ => {
                wait_for_input(&format!("{}Please enter a valid id", ansi::RED), cfg)
            }
        }
         //Returns the resource
    }
}
pub fn get_transfer_max(rss: &ResourceDict, cap: u128) -> Vec<u128> {
    let costs = rss.get_transfer_costs();
    let mut res = Vec::new(); //Initializes result
    for i in costs {
        if *i == 0 {
            res.push(u128::MAX); //No max
        } else {
            res.push(cap / i); //Maximum
        }
    }
    res //Returns the result
} //Gets the maximum amount of resources you can transfer
pub fn get_rss(rss: &ResourceDict, input: &mut Vec<u128>, cfg: &mut Config) {
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
pub fn display_rss(rss: &ResourceDict, v: &Vec<u128>, s: usize) -> String {
    let mut res = "".to_string(); //Initializes result
    for (i, item) in v.iter().enumerate() {
        res.push_str(&format!("{}: {} ({})\n", i + s, rss.get(ResourceID::new(i)), item)); //Adds resource
    }
    res //Returns result
}
