use crate::{
    component::{accessible, init, ComponentID, Components},
    file::{read_folder, FileObject, FilePresets},
    instr::Directions,
    location::Location,
    resources::ResourceDict,
    systems::object_id::ObjectID,
    systems::{system_id::SystemID, Systems},
};
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
pub const TRANSFER_COST: &str = "move cost";
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
pub fn cmp(rss: &ResourceDict) -> Components {
    let mut cmp = Components::new();
    cmp.init(rss);
    cmp
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
