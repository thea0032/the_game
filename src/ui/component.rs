use crate::ui::io::*;
use crate::{component::*, resources::*, object::*};

use super::ansi;

pub fn select_components_unfiltered(cmp:&Components) -> Option<(ComponentID, usize)>{
    refresh();
    println!("{}", cmp.display());//Displays all possible components
    println!("{}{}. Quit{}", ansi::RED, &cmp.len(), ansi::RESET);//Quit option
    let input:usize = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", |x| x <= &cmp.len());//Gets component
    if input == cmp.len(){
        return None;
    }//Quit option
    let amt:usize = get_from_input("Enter the amount of installations you want to perform: ", "please enter a valid number no higher than the max");//Gets amount of component from input
    return Some((ComponentID::new(input), amt));//Returns result
}//Returns a component, and an amount to install from input. None if aborted. 
pub fn select_component_unfiltered(cmp:&Components) -> Option<ComponentID>{
    refresh();
    println!("{}", cmp.display());
    println!("{}{}. Quit{}", ansi::RED, &cmp.len(), ansi::RESET);
    let input:usize = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", |x| x <= &cmp.len());
    if input == cmp.len(){
        return None;
    }//See above function for documentation. I'm not typing all of this again. 
    return Some(ComponentID::new(input));
}//Returns a component. None if aborted. 
pub fn select_components_filtered(cmp:&mut Components, v:&Vec<usize>) -> Option<(ComponentID, usize)>{
    refresh();
    let is_included:Vec<bool> = v.iter().map(|x| x != &0).collect();//Maps whether each option is displayed
    let len = is_included.iter().filter(|x| **x).count();//The amount of options displayed
    println!("{}", cmp.display_contained(&v));//Displays the options
    println!("{}{}. Quit{}", ansi::RED, len, ansi::RESET);//Quit option
    let input:usize = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", |x| x <= &len);//Gets input
    if input == len{
        return None;
    }//Quit option
    let id = crate::extra_bits::filter(input, &is_included);//Maps option selected to component selected
    let amt = get_from_input_valid("Enter the amount of components you want to remove", "Please enter a valid number", |x| *x <= v[id]);//Gets an amount
    return Some((ComponentID::new(input), amt));//Returns a value
}
pub fn details(rss:&ResourceDict, cmp:&mut Components){
    refresh();
    println!("{}", cmp.display_detailed(rss));//Displays helpful stuff
    wait_for_input("Press enter to continue:");//So you can see everything
}
pub fn add_component(cmp:&mut Components, obj:&mut Object){
    refresh();
    let amts:Vec<usize> = cmp.list.iter().map(|x| obj.resources().amt_contained(x.cost())).collect();//Gets amount of components you can afford
    let component = select_components_filtered(cmp, &amts);//Gets a component that you can afford
    if let Some(component) = component{
        let amt_success = obj.install_components(component.0, cmp, component.1);//Attempts to install components. 
        refresh();
        println!("{} components successfully installed!", amt_success);//Tells you how many successes there are
    } else {
        println!("Component installation aborted!");
    }
    wait_for_input("Press enter to continue:");//Makes sure that you can see the message
}
pub fn remove_component(cmp:&mut Components, obj:&mut Object){
    refresh();
    let component = select_components_filtered(cmp, obj.get_cmp_amts());//Gets a component that you currently have
    if let Some(component) = component{//If you didn't abort...
        let amt_success = obj.remove_components(component.0, cmp, component.1);//Attempts to remove components. 
        refresh();
        println!("{} components successfully removed!", amt_success);//Tells you how many successes there were
    } else {
        println!("Component removal aborted!");
    }
    wait_for_input("Press enter to continue:");//Makes sure that you can see the message
}