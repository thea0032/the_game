
use std::{collections::HashMap, str::FromStr};

use crate::{component::recipe::Recipe, ui::config::Config};
use crate::{
    component::{Component, Components},
    file::{read_folder, FileObject, FilePresets},
    instr::Directions,
    location::Location,
    object::Object,
    resources::{ResourceDict, ResourceID},
    systems::{system_id::SystemID, Systems},
};
pub const SYSTEMS: &str = "Systems";
pub const OBJECTS: &str = "Objects";
pub const LOCX: &str = "x";
pub const LOCY: &str = "y";
pub const LOCATION: &str = "Location";
pub fn sys_new(rss: &ResourceDict, cmp: &mut Components, dir: &mut Directions, file: &FileObject) -> Systems {
    let mut s = Systems::new();
    if let Some(val) = file.get(SYSTEMS) {
        //If there's a systems object in the file object...
        for (name, line) in val.grab_contents() {
            //For every system in the systems object...
            s.add_s(name.to_string(), grab_location(line)); //Adds the system based on the location grabbed
            if let Some(val) = line.get(OBJECTS) {
                //If there's an objects object in the systems object...
                for (name, line) in val.grab_contents() {
                    //For every object in the objects object...
                    let temp = s.add_o(rss, cmp, dir, name.to_string(), grab_location(line), SystemID::new(s.len() - 1));
                    init_object(line, s.get_o(temp), cmp, rss);
                }
            } else {
                panic!("Can't find objects!");
            }
        }
    } else {
        println!("Couldn't find systems object!");
    }
    s
}
pub fn init_object(file: &FileObject, obj: &mut Object, cmp: &Components, rss: &ResourceDict) {
    if let Some(val) = file.get(COMPONENTS) {
        if let Some(val) = val.get(ACCESSIBLE) {
            for (name, line) in val.grab_contents() {
                let component = cmp.get_from_name(name);
                let amt = parse(line, usize::MAX);
                obj.force_install_components(component, cmp, amt as u64);
            }
        }
        if let Some(val) = val.get(HIDDEN) {
            for (name, line) in val.grab_contents() {
                let component = cmp.get_from_name_h(name);
                let amt = parse(line, u64::MAX);
                obj.force_install_components(component, cmp, amt);
            }
        }
    }
    if let Some(val) = file.get(RESOURCES) {
        for (name, line) in val.grab_contents() {
            let resource = if let Some(val) = rss.find(name) {
                val
            } else {
                panic!("Couldn't find resource {:?}", name);
            };
            let amt = parse(line, u64::MAX);
            obj.resources_mut().add_res(resource, amt);
        }
    }
}
pub fn grab_location(file: &FileObject) -> Location {
    if let Some(val) = file.get(LOCATION) {
        Location::new(parse_field(val, f64::MAX, LOCX), parse_field(val, f64::MAX, LOCY))
    } else {
        panic!("Couldn't find location of {:?}!", file);
    }
}
pub const RESOURCES: &str = "Resources";
pub const TRANSFER_COST: &str = "Move Cost";
const RSSMOD:&str = "ResourceMod";
pub fn rss(file: &FileObject) -> ResourceDict {
    let res = file.get(RESOURCES);
    let mut names: Vec<String> = Vec::new();
    let mut transfer_costs: Vec<u64> = Vec::new();
    if let Some(val) = res {
        for (name, data) in val.grab_contents() {
            names.push(name.clone());
            if let Some(val) = data.get(TRANSFER_COST) {
                if let Ok(val) = val.name().parse::<u64>() {
                    transfer_costs.push(val);
                } else if val.name() == "MAX" {
                    transfer_costs.push(u64::MAX);
                } else {
                    panic!("{:?} cannot be parsed!", val.name());
                }
            } else {
                panic!("Couldn't find transfer cost for {:?}!", name);
            }
        }
    } else {
        let _ = 1;
        panic!("No resource object was found in {:?}!", file);
    }
    if let Some(val) = file.get(RSSMOD){
        let temp = rss_mod(val, &names);
        ResourceDict::new(names, transfer_costs, temp.0, temp.1, temp.2)
    } else {
        ResourceDict::new(names, transfer_costs, HashMap::new(), HashMap::new(), None)
    }
}
const REQUIRES:&str = "Requires";
const GROWTH:&str = "Growth";
const TRANSFER:&str = "IsTransfer";
pub fn rss_mod(
    file: &FileObject, names: &Vec<String>
) -> (
    HashMap<ResourceID, f64>,
    HashMap<ResourceID, HashMap<ResourceID, f64>>,
    Option<ResourceID>,
) {
    let mut res1:HashMap<ResourceID, f64> = HashMap::new();
    let mut res2:HashMap<ResourceID, HashMap<ResourceID, f64>> = HashMap::new();
    let mut res3:Option<ResourceID> = None;
    for (name, line) in file.grab_contents(){
        let idpos = ResourceID::new(names.iter().position(|x| x == name).expect(&format!("{:?} is not inside the resource dictionary!", name)));
        if let Some(val) = line.get(REQUIRES){
            let mut intermediate:HashMap<ResourceID, f64> = HashMap::new();
            for (name, new) in val.grab_contents() {
                if let Some(resource) = names.iter().position(|x| x == name) {
                    intermediate.insert(ResourceID::new(resource), parse(new, f64::MAX));
                } else {
                    panic!("{:?} is not inside the resource dictionary!", name);
                }
            }
            res2.insert(idpos, intermediate);
        }
        if let Some(val) = line.get(GROWTH){
            res1.insert(idpos, parse(val, f64::MAX));
        }
        if let Some(_) = line.get(TRANSFER){
            if res3.is_none(){
                res3 = Some(idpos);
            } else {
                panic!("Only one resource can be used as transfer currency!");
            }
        }
    }
    (res1, res2, res3)
}
const COMPONENTS: &str = "Components";
const ACCESSIBLE: &str = "Accessible";
const HIDDEN: &str = "Hidden";
const RECIPE: &str = "Recipes";
pub fn cmp(rss: &ResourceDict, file: &FileObject) -> Components {
    let mut cmp = Components::new();
    let mut names: Vec<String> = Vec::new();
    let mut h_names: Vec<String> = Vec::new();
    let mut components: Vec<Component> = Vec::new();
    let mut h_components: Vec<Component> = Vec::new();
    let mut r_names: Vec<String> = Vec::new();
    let mut recipes: Vec<Recipe> = Vec::new();
    if let Some(val) = file.get(COMPONENTS) {
        if let Some(val) = val.get(ACCESSIBLE) {
            for (name, val) in val.grab_contents() {
                names.push(name.clone());
                components.push(generate_component(val, rss));
            }
        }
        if let Some(val) = val.get(HIDDEN) {
            for (name, val) in val.grab_contents() {
                h_names.push(name.clone());
                h_components.push(generate_component(val, rss));
            }
        }
    }
    if let Some(val) = file.get(RECIPE) {
        for (name, val) in val.grab_contents(){
            r_names.push(name.clone());
            let mut recipe = Recipe::new(rss.len());
            for (name, val) in val.grab_contents(){
                let resource = rss.find(name).expect(&format!("Couldn't find {} in resources!", name));
                let amt = parse(val, i64::MAX);
                recipe.cost()[resource.get()] = amt;
            }
            recipes.push(recipe);
        }
    } else {
        panic!("No recipes object found!");
    }
    cmp.add_l(names, components);
    cmp.add_h_l(h_names, h_components);
    cmp.add_r_l(r_names, recipes);
    cmp
}
pub const COST: &str = "Cost";
pub const SURPLUS: &str = "Surplus";
pub const STORAGE: &str = "Storage";
pub fn generate_component(file: &FileObject, rss: &ResourceDict) -> Component {
    let mut res = Component::new(rss.len());
    if let Some(val) = file.get(COST) {
        for (name, new) in val.grab_contents() {
            if let Some(resource) = rss.find(name) {
                res.change_cost(resource, parse(new, i64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary!", name);
            }
        }
    }
    if let Some(val) = file.get(SURPLUS) {
        for (name, new) in val.grab_contents() {
            if let Some(resource) = rss.find(name) {
                res.change_surplus(resource, parse(new, i64::MAX));
            } else {
                panic!(
                    "{:?} is not inside the resource dictionary! Contents of resource dictionary: {:?}",
                    name, rss
                );
            }
        }
    }
    if let Some(val) = file.get(STORAGE) {
        for (name, new) in val.grab_contents() {
            if let Some(resource) = rss.find(name) {
                res.change_storage(resource, parse(new, u64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary!", name);
            }
        }
    }
    res
}
fn parse<T>(obj: &FileObject, max: T) -> T
where
    T: FromStr, {
    let val = obj.name().trim();
    if let Ok(val) = val.parse::<T>() {
        val
    } else if obj.name() == "MAX" {
        max
    } else {
        panic!("{:?} cannot be parsed!", obj.name());
    }
}
fn parse_field<T>(obj: &FileObject, max: T, field: &str) -> T
where
    T: FromStr, {
    if let Some(val) = obj.get(field) {
        parse(val, max)
    } else {
        panic!("{:?} cannot be parsed!", obj.name());
    }
}
pub fn dir() -> Directions {
    Directions::new()
}
pub fn config(presets: FilePresets) -> Config {
    Config::setup(presets)
}
pub fn load(presets: FilePresets, paths: Vec<&str>) -> FileObject {
    let mut res: FileObject = FileObject::blank(String::new()); //initializes the result
    for line in paths {
        let v = read_folder(&presets, line); //Reads the folder
        for line in v {
            //For every file in the folder...
            res.merge(FileObject::read_from(line, String::new(), 0)); //Merges the contents
        }
    }
    res
}
