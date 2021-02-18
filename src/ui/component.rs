use crate::{component::*, object::*, resources::*};
use crate::{extra_bits::filter, ui::io::*};

use super::{clipboard::Clipboard, config::Config, from_str::MenuRes, select::generic_select};

pub fn select_components_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<(ComponentID, usize)> {
    let component = select_component_unfiltered(cmp, cfg)?;
    let amt: usize = get_from_input(
        "Enter the amount of installations you want to perform: ",
        "please enter a valid number no higher than the max",
        cfg,
    ); //Gets amount of component from input
    Some((component, amt)) //Returns result
} //Returns a component, and an amount to install from input. None if aborted.

pub fn detail(rss: &ResourceDict, cmp: &mut Components, cfg: &mut Config) {
    let temp = select_component_unfiltered(cmp, cfg);
    if let Some(val) = temp {
        cmp.display_one(rss, val);
        if let MenuRes::Copy(v) = get_from_input::<MenuRes>("Press q to continue (you can also copy): ", "Please enter a valid input! Try q.", cfg) {
            *(cfg.clipboard(v)) = Clipboard::Component(val)
        }
    } else {
        println!("Operation aborted!");
    }
}
pub fn select_component_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<ComponentID> {
    generic_select(
        &cmp.display(),
        cmp.len(),
        |x| Some(ComponentID::new(x)),
        cfg,
        |x| if let Clipboard::Component(val) = &x { Some(*val) } else { None },
    )
} //Returns a component. None if aborted.
pub fn select_components_filtered(cmp: &mut Components, v: &Vec<usize>, cfg: &mut Config) -> Option<(ComponentID, usize)> {
    let is_included: Vec<bool> = v.iter().map(|x| x != &0).collect(); //Maps whether each option is displayed
    let len = is_included.iter().filter(|x| **x).count(); //The amount of options displayed
    let input: Option<ComponentID> = generic_select(
        &cmp.display_contained(v),
        len,
        |x| Some(ComponentID::new(filter(x, &is_included))),
        cfg,
        |x| {
            if let Clipboard::Component(val) = &x {
                if is_included[val.id()] {
                    Some(*val)
                } else {
                    None
                }
            } else {
                None
            }
        },
    );
    let id = input?;
    let amt = get_from_input_valid(
        "Enter the amount of components you want to remove",
        "Please enter a valid number",
        cfg,
        |x| *x <= v[id.id()],
    ); //Gets an amount
    Some((id, amt)) //Returns a value
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
