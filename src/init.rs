use std::{fs::File, str::FromStr};

use crate::{component::{Component, ComponentID, Components, accessible, init}, file::{read_folder, FileObject, FilePresets}, instr::Directions, location::Location, resources::ResourceDict, systems::object_id::ObjectID, systems::{system_id::SystemID, Systems}};
use crate::{resources, ui::config::Config};
pub fn sys(rss: &ResourceDict, cmp: &mut Components, dir: &mut Directions) -> Systems {
    let mut sys = Systems::new();
    sys.add_s("Sol".to_string(), Location::new(0.0, 0.0));
    let id1 = SystemID::new(0);
    sys.add_o(rss, cmp, dir, "Test Ship".to_string(), Location::new(0.0, 0.0), id1);
    {
        let test_ship = sys.get_o(ObjectID::new(0));
        test_ship.force_install_components(ComponentID::new_h(init::constants::INIT), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::HULL), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::HULL), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::HULL), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::HULL), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::ENGINE), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::BATTERY), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::LIVING_QUARTERS), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::STORAGE_SPACE), cmp, 1);
        test_ship.force_install_components(ComponentID::new(accessible::constants::ENGINE), cmp, 1);
        //test_ship.force_install_component(ComponentID::new(accessible::
        // constants::SOLAR_PANELS), cmp);
    }
    sys.add_o(rss, cmp, dir, "Earth".to_string(), Location::new(0.0, 0.0), id1);
    {
        let earth = sys.get_o(ObjectID::new(1));
        earth.force_install_components(ComponentID::new_h(init::constants::INIT), cmp, 1);
        earth.force_install_components(ComponentID::new_h(init::constants::SMALL_PLANET), cmp, 4);
        earth.force_install_components(ComponentID::new_h(init::constants::AIR_POCKET), cmp, 4);
        earth.force_install_components(ComponentID::new_h(init::constants::MINERAL_DEPOSIT), cmp, 4);
        earth.force_install_components(ComponentID::new_h(init::constants::URANIUM_DEPOSIT), cmp, 4);
        earth.force_install_components(ComponentID::new_h(init::constants::WATER_POCKET), cmp, 10);
        earth.force_install_components(ComponentID::new_h(init::constants::BIOSPHERE), cmp, 4);
        earth.force_install_components(ComponentID::new(accessible::constants::SOLAR_PANELS), cmp, 4);
        earth.force_install_components(ComponentID::new(accessible::constants::LIVING_QUARTERS), cmp, 10);
        earth.force_install_components(ComponentID::new(accessible::constants::FACTORY), cmp, 1);
        earth.force_install_components(ComponentID::new(accessible::constants::STORAGE_SPACE), cmp, 10);
        earth.resources_mut().add_res(resources::constants::POPULATION, 5);
        earth.resources_mut().add_res(resources::constants::FOOD, 5);
    }
    sys
}
pub const RESOURCES: &str = "Resources";
pub const TRANSFER_COST: &str = "Move Cost";
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
    ResourceDict::new(names, transfer_costs)
}
pub const COMPONENTS: &str = "Components";
pub const ACCESSIBLE: &str = "Accessible";
pub const HIDDEN: &str = "Hidden";
pub fn cmp(rss: &ResourceDict, file:&FileObject) -> Components {
    let mut cmp = Components::new();
    let mut names:Vec<String> = Vec::new();
    let mut h_names:Vec<String> = Vec::new();
    let mut components:Vec<Component> = Vec::new();
    let mut h_components:Vec<Component> = Vec::new();
    if let Some(val) = file.get(COMPONENTS){
        if let Some(val) = val.get(ACCESSIBLE){
            for (name, val) in val.grab_contents(){
                names.push(name.clone());
                components.push(generate_component(val, rss));
            }
        }
        if let Some(val) = val.get(HIDDEN){
            for (name, val) in val.grab_contents(){
                h_names.push(name.clone());
                h_components.push(generate_component(val, rss));
            }
        }
    }
    cmp.add_l(names, components);
    cmp.add_h_l(h_names, h_components);
    cmp.init(rss);
    cmp
}
pub const COST:&str = "Cost";
pub const SURPLUS:&str = "Surplus";
pub const STORAGE:&str = "Storage";
pub fn generate_component(file: &FileObject, rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    if let Some(val) = file.get(COST){
        for (name, new) in val.grab_contents(){
            if let Some(resource) = rss.find(name){
                res.change_cost(resource, parse(new, i64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary!", name);
            }
        }
    }
    if let Some(val) = file.get(SURPLUS){
        for (name, new) in val.grab_contents(){
            if let Some(resource) = rss.find(name){
                res.change_surplus(resource, parse(new, i64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary! Contents of resource dictionary: {:?}", name, rss);
            }
        }
    }
    if let Some(val) = file.get(STORAGE){
        for (name, new) in val.grab_contents(){
            if let Some(resource) = rss.find(name){
                res.change_storage(resource, parse(new, u64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary!", name);
            }
        }
    }
    res
}
fn parse<T>(obj:&FileObject, max:T) -> T where T:FromStr{
    if let Ok(val) = obj.name().parse::<T>() {
        return val;
    } else if obj.name() == "MAX" {
        return max;
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
            res.merge(FileObject::read_from(line, String::new())); //Merges the contents
        }
    }
    return res;
}
