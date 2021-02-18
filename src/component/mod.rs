pub mod recipe;
use crate::extra_bits;
use crate::resources::*;

use self::recipe::Recipe;
#[derive(Clone, Debug)]
///A gigantic list of components. Contains all visible components, hidden components, and
/// (only used in initialization), and recipes.
pub struct Components {
    pub list: Vec<Component>, //list of all accessible components
    pub names: Vec<String>,   //names of all accessible components
    pub hidden_list: Vec<Component>, /* list of all hidden components (hidden = can't install
                               * yourself) */
    pub hidden_names: Vec<String>, //names of all hidden components
    pub recipe_list: Vec<Recipe>,  //list of all recipes
    pub recipe_names: Vec<String>, //names of all recipes
}
impl Components {
    ///Gets a component from an ID. Returns the corresponding component.
    pub fn get(&self, id: ComponentID) -> &Component {
        if !id.is_hidden {
            &self.list[id.id]
        } else {
            &self.hidden_list[id.id]
        }
    }
    ///Gets a component's id from the name entered. Searches the visible components.
    ///Panics if the name wasn't found.
    pub fn get_from_name(&self, name: &str) -> ComponentID {
        for (i, line) in self.names.iter().enumerate() {
            if line == name {
                return ComponentID::new(i);
            }
        }
        panic!("{} was not found!", name);
    }
    ///Gets a component's id from the name entered. Searches the hidden components.
    ///Panics if the name wasn't found.
    pub fn get_from_name_h(&self, name: &str) -> ComponentID {
        for (i, line) in self.hidden_names.iter().enumerate() {
            if line == name {
                return ComponentID::new_h(i);
            }
        }
        panic!("{} was not found!", name);
    }
    ///Gets a recipe in this object from an ID. Returns the corresponding recipe.
    pub fn get_r(&self, id: RecipeID) -> &Recipe {
        &self.recipe_list[id.id]
    }
    ///Gets a component's name in this object from the ID entered.
    pub fn get_name(&self, id: ComponentID) -> &String {
        if !id.is_hidden {
            &self.names[id.id]
        } else {
            &self.hidden_names[id.id]
        }
    }
    ///Gets a recipe's name in this object from the ID entered.
    pub fn get_r_name(&self, id: RecipeID) -> &String {
        &self.recipe_names[id.id]
    }
    ///Initializes an empty components object.
    pub fn new() -> Components {
        Components {
            list: Vec::new(),
            names: Vec::new(),
            hidden_list: Vec::new(),
            hidden_names: Vec::new(),
            recipe_list: Vec::new(),
            recipe_names: Vec::new(),
        }
    }
    ///Adds a vector of components and a vector of corresponding names to this object.
    ///All objects are appended to the visible list.
    pub fn add_l(&mut self, mut name: Vec<String>, mut component: Vec<Component>) {
        self.list.append(&mut component);
        self.names.append(&mut name);
    }
    ///Adds a vector of components and a vector of corresponding names to this object.
    ///All objects are appended to the hidden list.
    pub fn add_h_l(&mut self, mut name: Vec<String>, mut component: Vec<Component>) {
        self.hidden_list.append(&mut component);
        self.hidden_names.append(&mut name);
    }

    ///Adds a vector of recipes and a vector of corresponding names to this object.
    pub fn add_r_l(&mut self, mut name: Vec<String>, mut recipe: Vec<Recipe>) {
        self.recipe_list.append(&mut recipe);
        self.recipe_names.append(&mut name);
    }
    ///Returns a numbered list of the visible components inside this object.
    pub fn display(&self) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.list.len() {
            x.push_str(&format!("{}: {}", i, &self.names[i]));
            x.push('\n'); //separates them by line
        }
        x
    }
    /// Returns a numbered list of visible components inside this object.
    /// The list is filtered based on the "contents" of a.
    /// A should be the same size as the components vector. If a is too sort,
    /// the function will end early. If a is too long, there will be a panic.
    /// # Example
    /// ```
    /// Components object = Components::new();
    /// object.add_l(vec!["a".to_string(), "b".to_string(), "c".to_string()], vec![]);//Do not do this.
    /// object.display_contained(vec![0, 1, 3])
    /// /*
    /// prints:
    /// 0: b (1)
    /// 1: c(3)
    /// */
    /// ```
    pub fn display_contained(&self, a: &Vec<usize>) -> String {
        let mut x: String = "".to_string();
        let mut counter: usize = 0;
        for (i, item) in a.iter().enumerate() {
            if *item != 0 {
                x.push_str(&format!("{}: {} ({})", counter, &self.names[i], item));
                x.push_str(", \n");
                counter += 1;
            }
        }
        x
    }
    /// Returns a numbered list of visible components inside this object.
    /// The list includes information on the components - their costs, and what
    /// resources they give.
    /// # Example
    /// ```
    /// let mut rss:ResourceDict = ResourceDict::new(vec!["A".to_string(), "B".to_string(), "C".to_string()], vec![0; 3], HashMap::new(), HashMap::new(), None);
    /// // Note: This creates 3 basic resources called A, B, and C, that cost nothing to transfer. Ignore the latter 3.
    /// let mut object:Components = Components::new();
    /// let mut a:Component = Component::new();
    /// a.change_cost(rss.find("A").unwrap(), 50); //a costs 50 A's.
    /// a.change_cost(rss.find("B").unwrap(), -50); //Installing a gives 50 B's.
    /// a.change_surplus(rss.find("C").unwrap(), 10); // a gives 10 C's every turn.
    /// let mut b:Component = Component::new();
    /// b.change_surplus(rss.find("A").unwrap(), 1); // b gives an A every turn.
    /// object.add_l(vec!["a".to_string(), "b".to_string()], vec![a, b]);
    /// object.display_detailed(&rss);
    /// /*
    /// Prints:
    /// 0: a
    ///   cost: 50 a, 50 b
    ///   surplus: 10 c
    ///   <There is no storage line.>
    /// 1: b
    ///   surplus: 1 a
    /// */
    /// ```
    pub fn display_detailed(&self, rss: &ResourceDict) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.list.len() {
            x.push_str(&format!("{}: {}", i, &self.names[i]));
            x.push('\n');
            x.push_str(&self.list[i].display(rss));
        }
        x
    }
    pub fn display_one(&self, rss: &ResourceDict, id: ComponentID) -> String {
        let mut x: String = "".to_string();
        x.push_str(&format!("{}: {}", id.id(), &self.names[id.id()]));
        x.push('\n');
        x.push_str(&self.list[id.id()].display(rss));
        x
    }
    pub fn display_r(&self) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.recipe_list.len() {
            x.push_str(&format!("{}: {}", i, &self.recipe_names[i]));
            x.push('\n');
        }
        x
    }
    pub fn display_contained_r(&self, a: &Vec<usize>) -> String {
        let mut x: String = "".to_string();
        let mut counter: usize = 0;
        for (i, item) in a.iter().enumerate() {
            if *item != 0 {
                x.push_str(&format!("{}: {} ({})", counter, &self.recipe_names[i], item));
                x.push_str(", \n");
                counter += 1;
            }
        }
        x
    }
    pub fn display_detailed_r(&self, rss: &ResourceDict) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.recipe_list.len() {
            x.push_str(&format!("{}: {}", i, &self.recipe_names[i]));
            x.push('\n');
            x.push_str(&self.recipe_list[i].display(rss));
            x.push('\n');
        }
        x
    }
    pub fn display_one_r(&self, rss: &ResourceDict, i: RecipeID) -> String {
        let mut x: String = "".to_string();
        x.push_str(&format!("{}: {}", i.id, &self.recipe_names[i.id]));
        x.push('\n');
        x.push_str(&self.recipe_list[i.id].display(rss));
        x
    }
    /// Gets the amount of visible components inside this object.
    pub fn len(&self) -> usize {
        self.list.len()
    }
    ///Gets the amount of recipes inside the object.
    pub fn len_r(&self) -> usize {
        self.recipe_list.len()
    }
}
///The components structure is mostly made out of this structure.
#[derive(Clone, Debug)]
pub struct Component {
    surplus: Vec<i64>,
    storage: Vec<u64>,
    cost: Vec<i64>,
}
impl Component {
    pub fn cost(&self) -> &Vec<i64> {
        &self.cost
    }
    pub fn surplus(&self) -> &Vec<i64> {
        &self.surplus
    }
    pub fn storage(&self) -> &Vec<u64> {
        &self.storage
    }
    pub fn change_cost(&mut self, id: ResourceID, val: i64) {
        self.cost[id.get()] = val;
    }
    pub fn change_surplus(&mut self, id: ResourceID, val: i64) {
        self.surplus[id.get()] = val;
    }
    pub fn change_storage(&mut self, id: ResourceID, val: u64) {
        self.storage[id.get()] = val;
    }
    pub fn new(size: usize) -> Component {
        Component {
            surplus: extra_bits::fill(size, 0),
            storage: extra_bits::fill(size, 0),
            cost: extra_bits::fill(size, 0),
        }
    } //Basic accessing functions
    pub fn display(&self, rss: &ResourceDict) -> String {
        let mut x: String = "".to_string();
        x.push_str(&self.display_func(rss, "  cost: ", &self.cost, 0));
        x.push_str(&self.display_func(rss, "  surplus: ", &self.surplus, 0));
        x.push_str(&self.display_func(rss, "  storage: ", &self.storage, 0));
        x
    }
    pub fn display_func<T>(&self, rss: &ResourceDict, msg: &str, a: &Vec<T>, zero: T) -> String
    where
        T: PartialEq,
        T: Copy,
        T: ToString, {
        let mut x: String = "".to_string(); //Initializes rseult
        let mut flag: bool = false;
        for (i, item) in a.iter().enumerate() {
            //For every resource...
            if *item != zero {
                //Doesn't display resources that you have zero of
                if !flag {
                    //Does this once, if a resource is present
                    x.push_str(msg); //Adds the message parameter
                    flag = true; //Sets flag to true
                }
                x.push_str(&(*item).to_string()); //Adds the amount of the resource
                x.push(' ');
                x.push_str(&rss.get(ResourceID::new(i))); //Adds the resource type
                x.push(',');
                x.push(' '); //Extra formatting stuff
            }
        }
        x.push('\n'); //adds newline character so that everything appears on a separate line
        x //returns result
    } //Displays lots of stuff
}
#[derive(Clone, Copy, Debug)]
pub struct ComponentID {
    id: usize,
    is_hidden: bool,
} //Identifies component
impl ComponentID {
    pub fn id(&self) -> usize {
        self.id
    } //getter
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    } //getter
    pub fn new(id: usize) -> ComponentID {
        ComponentID { id, is_hidden: false }
    } //new, hidden set to false
    pub fn new_h(id: usize) -> ComponentID {
        ComponentID { id, is_hidden: true }
    } //new, hidden set to true
}
#[derive(Debug, Clone, Copy)]
pub struct RecipeID {
    id: usize,
} //Recipe id wrapper
impl RecipeID {
    pub fn new(id: usize) -> RecipeID {
        RecipeID { id }
    } //new
    pub fn id(&self) -> usize {
        self.id
    }
}
