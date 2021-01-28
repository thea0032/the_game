use crate::ui::io::*;
use crate::{component::*, object::*, resources::*};

use super::{ansi, clipboard, config::Config, from_str::{InBounds, MenuRes}};

pub fn select_components_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<(ComponentID, usize)> {
    println!("{}", cmp.display()); //Displays all possible components
    println!("{}{}. Quit{}", ansi::RED, &cmp.len(), ansi::RESET); //Quit option
    let input: usize = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", cfg, |x| x <= &cmp.len()); //Gets component
    if input == cmp.len() {
        return None;
    } //Quit option
    let amt: usize = get_from_input(
        "Enter the amount of installations you want to perform: ",
        "please enter a valid number no higher than the max",
        cfg,
    ); //Gets amount of component from input
    Some((ComponentID::new(input), amt)) //Returns result
} //Returns a component, and an amount to install from input. None if aborted.
pub fn select_component_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<ComponentID> {
    println!("{}", cmp.display());
    println!("{}{}. Quit{}", ansi::RED, &cmp.len(), ansi::RESET);
    let input: usize = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", cfg, |x| x <= &cmp.len());
    if input == cmp.len() {
        return None;
    } //See above function for documentation. I'm not typing all of this again.
    Some(ComponentID::new(input))
} //Returns a component. None if aborted.
pub fn select_components_filtered(cmp: &mut Components, v: &Vec<usize>, cfg: &mut Config) -> Option<(ComponentID, usize)> {
    let is_included: Vec<bool> = v.iter().map(|x| x != &0).collect(); //Maps whether each option is displayed
    let len = is_included.iter().filter(|x| **x).count(); //The amount of options displayed
    let mut ctx = cfg.generate_context();
    let mut dis = cfg.generate_display();
    cfg.update_context_all(&mut dis);
    cfg.update_context(Config::PASTE, Some("paste".to_string()), &mut ctx, &mut dis);
    cfg.update_context(Config::QUIT, Some("abort".to_string()), &mut ctx, &mut dis);
    let id;
    loop{
        println!("{}", cfg.display(&ctx, &dis));
        println!("{}", cmp.display_contained(&v)); //Displays the options
        let input: MenuRes = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", cfg, |x:&MenuRes| x.in_bounds(&len)); //Gets input
        match input {
            MenuRes::Enter(val) => {
                id = val;
                break;
            }
            MenuRes::Exit | MenuRes::Del => {
                return None;
            }
            MenuRes::Paste => {
                match cfg.cpb{
                    clipboard::Clipboard::Component(val) => {
                        if is_included[val.id()]{
                            id = val.id();
                            break;
                        }
                    }
                    _ => {},
                }
                wait_for_input(&format!("{}You cannot paste that there!", ansi::RED), cfg);
            }
            _ => {
                wait_for_input(&format!("{}Please enter a valid id", ansi::RED), cfg);
            }
        }
    };
    let amt = get_from_input_valid(
        "Enter the amount of components you want to remove",
        "Please enter a valid number",
        cfg,
        |x| *x <= v[id],
    ); //Gets an amount
    Some((ComponentID::new(id), amt)) //Returns a value
}
pub fn details(rss: &ResourceDict, cmp: &mut Components, cfg: &mut Config) {
    println!("{}", cmp.display_detailed(rss)); //Displays helpful stuff
    wait_for_input("Press enter to continue:", cfg); //So you can see everything
}
pub fn add_component(cmp: &mut Components, obj: &mut Object, cfg: &mut Config) {
    let amts: Vec<usize> = cmp.list.iter().map(|x| obj.resources().amt_contained(x.cost())).collect(); //Gets amount of components you can afford
    let component = select_components_filtered(cmp, &amts, cfg); //Gets a component that you can afford
    if let Some(component) = component {
        let amt_success = obj.install_components(component.0, cmp, component.1); //Attempts to install components.
        println!("{} components successfully installed!", amt_success); //Tells you how many successes there are
    } else {
        println!("Component installation aborted!");
    }
    wait_for_input("Press enter to continue:", cfg); //Makes sure that you can see
                                                     // the message
}
pub fn remove_component(cmp: &mut Components, obj: &mut Object, cfg: &mut Config) {
    let component = select_components_filtered(cmp, obj.get_cmp_amts(), cfg); //Gets a component that you currently have
    if let Some(component) = component {
        //If you didn't abort...
        let amt_success = obj.remove_components(component.0, cmp, component.1); //Attempts to remove components.
        println!("{} components successfully removed!", amt_success); //Tells you how many
                                                                      // successes there were
    } else {
        println!("Component removal aborted!");
    }
    wait_for_input("Press enter to continue:", cfg); //Makes sure that you can see
                                                     // the message
}
