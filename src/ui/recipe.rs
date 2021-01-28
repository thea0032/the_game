use crate::{
    component::{Components, RecipeID},
    object::Object,
    resources::ResourceDict,
};

use super::{ansi, clipboard::Clipboard, config::Config, from_str::{InBounds, MenuRes}, io::{get_from_input, get_from_input_valid, wait_for_input}};

pub fn select_recipes(cmp: &Components, obj: &Object, cfg: &mut Config) -> Option<(RecipeID, usize)> {
    let amts: Vec<usize> = cmp.recipe_list.iter().map(|x| obj.resources().amt_contained(x.cost_stat())).collect(); //Maximum amount of each recipe
    let mut ctx = cfg.generate_context();
    let mut dis = cfg.generate_display();
    cfg.update_context_all(&mut dis);
    cfg.update_context(Config::PASTE, Some("paste".to_string()), &mut ctx, &mut dis);
    cfg.update_context(Config::QUIT, Some("abort".to_string()), &mut ctx, &mut dis);
    println!("{}", cfg.display(&ctx, &dis));
    println!("{}", cmp.display_contained_r(&amts)); //Displays stuff
    let filter: Vec<bool> = amts.iter().map(|x| x != &0).collect(); //Whether it's actually an option
    let len = filter.iter().filter(|x| **x).count(); //Amount of options
    let input: MenuRes = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", cfg, |x:&MenuRes| x.in_bounds(&len)); //Gets input
    let id:Option<RecipeID> = match input{
        MenuRes::Enter(val) => {Some(RecipeID::new(crate::extra_bits::filter(val, &filter)))}
        MenuRes::Exit => {None}
        MenuRes::Del => {None}
        MenuRes::Paste => {
            match cfg.cpb{
                Clipboard::Recipe(val) => {
                    Some(val)
                }
                _ => {
                    let v:bool = get_from_input("Invalid response! Do you want to try again?", "Please enter a valid input", cfg);
                    if v{
                        select_recipe_unfiltered(cmp, cfg)
                    } else {
                        None
                    }
                }
            }
        }
        _ => {
            let v:bool = get_from_input("Invalid response! Do you want to try again?", "Please enter a valid input", cfg);
            if v{
                select_recipe_unfiltered(cmp, cfg)
            } else {
                None
            }
        }
    };
    if id.is_none(){
        return None;
    }
    let amt: usize = get_from_input_valid(
        "Enter the amount of times you want to perform the recipe: ",
        "please enter a valid id",
        cfg,
        |x| x <= &amts[id.unwrap().id()],
    ); //Gets amount
    Some((id.unwrap(), amt)) //Returns
} //Returns a recipe and amount from input unless aborted
pub fn select_recipe_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<RecipeID> {
    println!("{}", cmp.display_r());
    println!("{}{}. Quit{}", ansi::RED, cfg.quit().id(), ansi::RESET); //Displays options
    let input: MenuRes = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", cfg, |x:&MenuRes| x.in_bounds(&cmp.len_r())); //Gets input
    match input{
        MenuRes::Enter(val) => {Some(RecipeID::new(val))}
        MenuRes::Exit => {None}
        MenuRes::Del => {None}
        MenuRes::Paste => {
            match cfg.cpb{
                Clipboard::Recipe(val) => {
                    Some(val)
                }
                _ => {
                    let v:bool = get_from_input("Invalid response! Do you want to try again?", "Please enter a valid input", cfg);
                    if v{
                        select_recipe_unfiltered(cmp, cfg)
                    } else {
                        None
                    }
                }
            }
        }
        _ => {
            let v:bool = get_from_input("Invalid response! Do you want to try again?", "Please enter a valid input", cfg);
            if v{
                select_recipe_unfiltered(cmp, cfg)
            } else {
                None
            }
        }
    }
} //Returns a recipe from input unless aborted
pub fn select_recipes_unfiltered(cmp: &Components, cfg: &mut Config) -> Option<(RecipeID, usize)> {
    println!("{}", cmp.display_r());
    println!("{}{}. Quit{}", ansi::RED, cmp.len_r(), ansi::RESET); //Displays options
    let input: usize = get_from_input_valid("Enter the recipe you want: ", "Please enter a valid id", cfg, |x| x <= &cmp.len_r()); //Gets input
    if input == cmp.len_r() {
        return None; //abort
    }

    let amt: usize = get_from_input(
        "Enter the amount of times you want to perform the recipe: ",
        "please enter a valid id",
        cfg,
    ); //Gets amount
    Some((RecipeID::new(input), amt)) //Returns value
}
pub fn r_details(rss: &ResourceDict, cmp: &mut Components, cfg: &mut Config) {
    println!("{}", cmp.display_detailed_r(rss));
    wait_for_input("Press enter to continue:", cfg);
}
pub fn r_detail(rss: &ResourceDict, cmp: &mut Components, cfg: &mut Config) {
    let temp = select_recipe_unfiltered(cmp, cfg);
    if let Some(val) = temp{
        cmp.display_one_r(rss, val);
        match get_from_input::<MenuRes>("Press enter to continue (you can also copy): ", "Please enter a valid input! Try q.", cfg){
            MenuRes::Copy => {
                cfg.cpb = Clipboard::Recipe(val);
            }
            _=>{}
        }
    } else {
        println!("Operation aborted!");
    }
}
pub fn perform_recipe(cmp: &mut Components, obj: &mut Object, cfg: &mut Config) {
    let recipe = select_recipes(cmp, obj, cfg);
    if let Some(recipe) = recipe {
        let amt_success = obj.do_recipes(recipe.0, cmp, recipe.1);
        println!("{} recipes successfully performed!", amt_success);
    } else {
        println!("Recipe aborted!");
    }
} //Performs a recipe from input
