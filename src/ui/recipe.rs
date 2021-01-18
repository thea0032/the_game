use crate::{component::{Components, RecipeID}, object::Object, resources::ResourceDict};

use super::{ansi, io::{get_from_input, get_from_input_valid, wait_for_input}};



pub fn select_recipes(cmp:&Components, obj:&Object) -> Option<(RecipeID, usize)>{
    let amts:Vec<usize> = cmp.recipe_list.iter().map(|x| obj.resources().amt_contained(x.cost_stat())).collect();//
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
}//Returns a recipe and amount from input unless aborted
pub fn select_recipe_unfiltered(cmp:&Components) -> Option<RecipeID>{
    println!("{}", cmp.display_r());
    println!("{}{}. Quit{}", ansi::RED, cmp.len_r(), ansi::RESET);//Displays options
    let input:usize = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", |x| x <= &cmp.len_r());//Gets input
    if input == cmp.len_r(){
        return None;//Abort
    }
    return Some(RecipeID::new(input));//return
}//Returns a recipe from input unless aborted
pub fn select_recipes_unfiltered(cmp:&Components) -> Option<(RecipeID, usize)>{
    println!("{}", cmp.display_r());
    println!("{}{}. Quit{}", ansi::RED, cmp.len_r(), ansi::RESET);//Displays options
    let input:usize = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", |x| x <= &cmp.len_r());//Gets input
    if input == cmp.len_r(){
        return None;//abort
    }

    let amt:usize = get_from_input("Enter the amount of times you want to perform the recipe: ", "please enter a valid id");//Gets amount
    return Some((RecipeID::new(input), amt));//Returns value
}
pub fn r_details(rss:&ResourceDict, cmp:&mut Components){
    println!("{}", cmp.display_detailed_r(rss));
    wait_for_input("Press enter to continue:");
}
pub fn perform_recipe(cmp:&mut Components, obj:&mut Object){
    let recipe = select_recipes(cmp, obj);
    if let Some(recipe) = recipe{
        let amt_success = obj.do_recipes(recipe.0, cmp, recipe.1);
        println!("{} recipes successfully performed!", amt_success);
    } else {
        println!("Recipe aborted!");
    }
}//Performs a recipe from input