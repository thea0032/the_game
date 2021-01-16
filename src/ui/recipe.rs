use crate::{component::{Components, RecipeID}, object::Object, resources::ResourceDict};

use super::{ansi, io::{get_from_input, get_from_input_valid, refresh, wait_for_input}};



pub fn select_recipes(cmp:&Components, obj:&Object) -> Option<(RecipeID, usize)>{
    let amts:Vec<usize> = cmp.recipe_list.iter().map(|x| obj.resources().amt_contained(x.cost_stat())).collect();
    refresh();
    println!("{}", cmp.display_contained_r(&amts));
    let filter:Vec<bool> = amts.iter().map(|x| x != &0).collect();
    let len = filter.iter().filter(|x| **x).count();
    println!("{}{}. Quit{}", ansi::RED, len, ansi::RESET);
    let input:usize = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", |x| x <= &len);
    if input == len{
        return None;
    }
    let id:usize = crate::extra_bits::filter(input, &filter);
    let amt:usize = get_from_input_valid("Enter the amount of times you want to perform the recipe: ", "please enter a valid id", |x| x <= &amts[id]);
    return Some((RecipeID::new(id), amt));
}
pub fn select_recipe_unfiltered(cmp:&Components) -> Option<RecipeID>{
    refresh();
    println!("{}", cmp.display_r());
    println!("{}{}. Quit{}", ansi::RED, cmp.len_r(), ansi::RESET);
    let input:usize = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", |x| x <= &cmp.len_r());
    if input == cmp.len_r(){
        return None;
    }
    return Some(RecipeID::new(input));
}
pub fn select_recipes_unfiltered(cmp:&Components) -> Option<(RecipeID, usize)>{
    refresh();
    println!("{}", cmp.display_r());
    println!("{}{}. Quit{}", ansi::RED, cmp.len_r(), ansi::RESET);
    let input:usize = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", |x| x <= &cmp.len_r());
    if input == cmp.len_r(){
        return None;
    }

    let amt:usize = get_from_input("Enter the amount of times you want to perform the recipe: ", "please enter a valid id");
    return Some((RecipeID::new(input), amt));
}
pub fn r_details(rss:&ResourceDict, cmp:&mut Components){
    refresh();
    println!("{}", cmp.display_detailed_r(rss));
    wait_for_input("Press enter to continue:");
}
pub fn recipe(cmp:&mut Components, obj:&mut Object){
    refresh();
    let recipe = select_recipes(cmp, obj);
    if let Some(recipe) = recipe{
        let amt_success = obj.do_recipes(recipe.0, cmp, recipe.1);
        println!("{} recipes successfully performed!", amt_success);
    } else {
        println!("Recipe aborted!");
    }
}