use crate::{
    component::{Components, RecipeID},
    extra_bits,
    object::Object,
    resources::ResourceDict,
};

use super::{
    clipboard::Clipboard,
    config::Config,
    from_str::MenuRes,
    io::{get_from_input, get_from_input_valid, wait_for_input},
    select::generic_select,
};

pub fn select_recipes(cmp: &Components, obj: &Object, cfg: &mut Config) -> Option<(RecipeID, usize)> {
    let amts: Vec<usize> = cmp.recipe_list.iter().map(|x| obj.resources().amt_contained(x.cost_stat())).collect(); //Maximum amount of each recipe
    let id = select_recipe(cmp, obj, cfg)?;
    let amt: usize = get_from_input_valid(
        "Enter the amount of times you want to perform the recipe: ",
        "please enter a valid option (try q)",
        cfg,
        |x| x <= &amts[id.id()],
    ); //Gets amount
    Some((id, amt)) //Returns
} //Returns a recipe and amount from input unless aborted
pub fn select_recipe(cmp: &Components, obj: &Object, cfg: &mut Config) -> Option<RecipeID> {
    let amts: Vec<usize> = cmp.recipe_list.iter().map(|x| obj.resources().amt_contained(x.cost_stat())).collect(); //Maximum amount of each recipe
    let filter: Vec<bool> = amts.iter().map(|x| x != &0).collect(); //Whether it's actually an option
    let len = filter.iter().filter(|x| **x).count(); //Amount of options
    generic_select(
        &cmp.display_contained_r(&amts),
        len,
        |x| Some(RecipeID::new(extra_bits::filter(x, &filter))),
        cfg,
        |x| if let Clipboard::Recipe(val) = &x { Some(*val) } else { None },
    )
} //Returns a recipe from input unless aborted
pub fn select_recipe_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<RecipeID> {
    generic_select(
        &cmp.display_r(),
        cmp.len_r(),
        |x| Some(RecipeID::new(x)),
        cfg,
        |x| if let Clipboard::Recipe(val) = &x { Some(*val) } else { None },
    )
} //Returns a recipe from input unless aborted
pub fn select_recipes_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<(RecipeID, usize)> {
    let recipe = select_recipe_unfiltered(cmp, cfg)?;
    let amt: usize = get_from_input(
        "Enter the amount of times you want to perform the recipe: ",
        "please enter a valid id",
        cfg,
    ); //Gets amount
    Some((recipe, amt)) //Returns value
}
pub fn r_details(rss: &ResourceDict, cmp: &Components, cfg: &mut Config) {
    println!("{}", cmp.display_detailed_r(rss));
    wait_for_input("Press enter to continue:", cfg);
}
pub fn r_detail(rss: &ResourceDict, cmp: &Components, cfg: &mut Config) {
    let temp = select_recipe_unfiltered(cmp, cfg);
    if let Some(val) = temp {
        cmp.display_one_r(rss, val);
        if let MenuRes::Copy(v) =
            get_from_input::<MenuRes>("Press q to continue (you can also copy): ", "Please enter a valid input! Try q.", cfg)
        {
            *(cfg.clipboard(v)) = Clipboard::Recipe(val)
        }
    } else {
        println!("Operation aborted!");
    }
}
pub fn perform_recipe(cmp: &Components, obj: &mut Object, cfg: &mut Config) {
    let recipe = select_recipes(cmp, obj, cfg);
    if let Some(recipe) = recipe {
        let amt_success = obj.do_recipes(recipe.0, cmp, recipe.1);
        println!("{} recipes successfully performed!", amt_success);
    } else {
        println!("Recipe aborted!");
    }
} //Performs a recipe from input
