pub mod init;
pub mod accessible;
mod recipe;
use crate::{extra_bits};
use crate::resources::*;

use self::recipe::Recipe;
#[derive(Clone, Debug)]
pub struct Components{
    pub list:Vec<Component>,//list of all accessible components
    pub names:Vec<String>,//names of all accessible components
    pub hidden_list:Vec<Component>,//list of all hidden components (hidden = can't install yourself)
    pub hidden_names:Vec<String>,//names of all hidden components
    pub recipe_list:Vec<Recipe>,//list of all recipes
    pub recipe_names:Vec<String>,//names of all recipes
}
impl Components{
    pub fn get<'a>(&'a self, id:ComponentID) -> &'a Component{
        if !id.is_hidden{
            return &self.list[id.id];
        } else {
            return &self.hidden_list[id.id];
        }
    }//gets a component from the lists
    pub fn get_r<'a>(&'a self, id:RecipeID) -> &'a Recipe{
        return &self.recipe_list[id.id];
    }//gets a recipe from the list
    pub fn get_name<'a>(&'a self, id:ComponentID) -> &'a String{
        if !id.is_hidden{
            return &self.names[id.id];
        } else {
            return &self.hidden_names[id.id];
        }
    }//gets the component name from the lists
    pub fn get_r_name<'a>(&'a self, id:RecipeID) -> &'a String{
        return &self.recipe_names[id.id];
    }//gets the recipe name from the list
    pub fn init(&mut self, rss:&ResourceDict){
        self.add_l(accessible::get_names(), accessible::get_all(rss));
        self.add_h_l(init::get_names(), init::get_all(rss));
        self.add_r_l(recipe::get_names(), recipe::get_all(rss));
    }//initializes the components object from accessible.rs, init.rs, and recipe.rs
    pub fn new() -> Components{
        return Components{
            list:vec![],
            names:vec![],
            hidden_list:vec![],
            hidden_names:vec![],
            recipe_list:vec![],
            recipe_names:vec![],
        };
    }//new function
    pub fn add_l(&mut self, mut name:Vec<String>, mut component:Vec<Component>){
        self.list.append(&mut component);
        self.names.append(&mut name);
    }//adds a list of components and names to the component dictionary
    pub fn add_h_l(&mut self, mut name:Vec<String>, mut component:Vec<Component>){
        self.hidden_list.append(&mut component);
        self.hidden_names.append(&mut name);
    }//add_l but in the hidden category
    pub fn add_r_l(&mut self, mut name:Vec<String>, mut recipe:Vec<Recipe>){
        self.recipe_list.append(&mut recipe);
        self.recipe_names.append(&mut name);
    }//adds a list of recipes and names to the component dictionary
    pub fn display(&self) -> String{
        let mut x:String = "".to_string();
        for i in 0..self.list.len(){
            x.push_str(&format!("{}: {}", i, &self.names[i]));
            x.push('\n');//separates them by line
        }
        return x;
    }//displays the accessible components
    pub fn display_contained(&self, a:&Vec<usize>) -> String{
        let mut x:String = "".to_string();
        let mut counter:usize = 0;
        for i in 0..self.list.len(){
            if a[i] != 0{
                x.push_str(&format!("{}: {} ({})", counter, &self.names[i], a[i]));
                x.push_str(", \n");
                counter += 1;
            }
        }
        return x;
    }//displays the accessible components, but filters them based on how many of them there are
    pub fn display_detailed(&self, rss:&ResourceDict) -> String{
        let mut x:String = "".to_string();
        for i in 0..self.list.len(){
            x.push_str(&format!("{}: {}", i, &self.names[i]));
            x.push_str("\n");
            x.push_str(&self.list[i].display(rss));
        }
        return x;
    }
    pub fn display_r(&self) -> String{
        let mut x:String = "".to_string();
        for i in 0..self.recipe_list.len(){
            x.push_str(&format!("{}: {}", i, &self.recipe_names[i]));
            x.push('\n');
        }
        return x;
    }
    pub fn display_contained_r(&self, a:&Vec<usize>) -> String{
        let mut x:String = "".to_string();
        let mut counter:usize = 0;
        for i in 0..self.recipe_list.len(){
            if a[i] != 0{
                x.push_str(&format!("{}: {} ({})", counter, &self.recipe_names[i], a[i]));
                x.push_str(", \n");
                counter += 1;
            }
        }
        return x;
    }
    pub fn display_detailed_r(&self, rss:&ResourceDict) -> String{
        let mut x:String = "".to_string();
        for i in 0..self.recipe_list.len(){
            x.push_str(&format!("{}: {}", i, &self.recipe_names[i]));
            x.push_str("\n");
            x.push_str(&self.recipe_list[i].display(rss));
        }
        return x;
    }
    pub fn len(&self) -> usize{
        return self.list.len();
    }
    pub fn len_r(&self) -> usize{
        return self.recipe_list.len();
    }
}
#[derive(Clone, Debug)]
pub struct Component{
    surplus:Vec<i64>,
    storage:Vec<u128>,
    cost:Vec<i64>,
}
impl Component{
    pub fn cost<'a>(&'a self) -> &'a Vec<i64>{
        return & self.cost;
    }
    pub fn surplus<'a>(&'a self) -> &'a Vec<i64>{
        return & self.surplus;
    }
    pub fn storage<'a>(&'a self) -> &'a Vec<u128>{
        return & self.storage;
    }
    pub fn change_cost(&mut self, id:ResourceID, val:i64){
        self.cost[id.get()] = val;
    }
    pub fn change_surplus(&mut self, id:ResourceID, val:i64){
        self.surplus[id.get()] = val;
    }
    pub fn change_storage(&mut self, id:ResourceID, val:u128){
        self.storage[id.get()] = val;
    }
    pub fn new(size:usize) -> Component{
        Component{
            surplus:extra_bits::fill(size, 0),
            storage:extra_bits::fill(size, 0),
            cost:extra_bits::fill(size, 0),
        }
    }
    pub fn display(&self, rss:&ResourceDict) -> String{
        let mut x:String = "".to_string();
        x.push_str(&self.display_func(rss, "  cost: ", &self.cost, 0));
        x.push_str(&self.display_func(rss, "  surplus: ", &self.surplus, 0));
        x.push_str(&self.display_func(rss, "  storage: ", &self.storage, 0));
        return x;
    }
    pub fn display_func<T>(&self, rss:&ResourceDict, msg:&str, a:&Vec<T>, zero:T) -> String where T:PartialEq, T:Copy, T:ToString{
        let mut x:String = "".to_string();//Initializes rseult
        let mut flag:bool = false;
        for i in 0..a.len(){//For every resource...
            if a[i] != zero{//Doesn't display resources that you have zero of
                if !flag {//Does this once, if a resource is present
                    x.push_str(msg);//Adds the message parameter
                    flag = true;//Sets flag to true
                }
                x.push_str(&a[i].to_string());//Adds the amount of the resource
                x.push(' ');
                x.push_str(&rss.get(ResourceID::new(i)));//Adds the resource type
                x.push(',');
                x.push(' ');//Extra formatting stuff
            }
        }
        x.push('\n');//adds newline character so that everything appears on a separate line
        return x;//returns result
    }//Displays lots of stuff
}
#[derive(Clone, Copy, Debug)]
pub struct ComponentID{
    id:usize,
    is_hidden:bool,
}//Identifies component
impl ComponentID{
    pub fn id(&self) -> usize{
        return self.id
    }//getter
    pub fn is_hidden(&self) -> bool{
        return self.is_hidden;
    }//getter
    pub fn new(id:usize) -> ComponentID{
        return ComponentID{
            id:id,
            is_hidden:false,
        }
    }//new, hidden set to false
    pub fn new_h(id:usize) -> ComponentID{
        return ComponentID{
            id:id,
            is_hidden:true,
        }
    }//new, hidden set to true
}
#[derive(Debug, Clone, Copy)]
pub struct RecipeID{
    id:usize,
}//Recipe id wrapper
impl RecipeID{
    pub fn new(id:usize) -> RecipeID{
        return RecipeID{id:id};
    }//new 
}