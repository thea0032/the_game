use crate::{component::{ComponentID, Components, accessible, init}, instr::Directions, location::Location, resources::ResourceDict, systems::object_id::ObjectID, systems::{system_id::SystemID, Systems}};
use crate::resources;
pub fn sys(rss:&mut ResourceDict, cmp:&mut Components, dir:&mut Directions) -> Systems{
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
        //test_ship.force_install_component(ComponentID::new(accessible::constants::SOLAR_PANELS), cmp);
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
    return sys;
}
pub fn rss() -> ResourceDict{
    return resources::ResourceDict::new(vec![
        "Energy".to_string(),
        "Water".to_string(),
        "Air".to_string(),
        "Ore".to_string(),
        "uranium".to_string(),
        "Biomass".to_string(),
        "Food".to_string(),
        "Metal".to_string(),
        "Population".to_string(),
        "Luxuries".to_string(),
        "Production".to_string(),
        "Transfer".to_string(),
        "Fuel".to_string(),
        "Movement".to_string(),
        "Space".to_string(), 
        "Living space".to_string(), 
        "Mass".to_string(), 
        "Mining jobs".to_string(), 
        "Uranium mining jobs".to_string(), 
        "Factory jobs".to_string(),
    ], vec![
        0, //Energy
        1, //Water
        1, //Air
        1, //Ore
        4, //Uranium
        5, //Biomass
        1, //Food
        1, //Metal
        10, //Population
        1, //Luxuries
        1, //Production
        0, //Transfer
        1, //Fuel
        u128::MAX, //Movement
        u128::MAX, //Space
        u128::MAX, //Living space
        u128::MAX, //Mass
        u128::MAX, //Mining jobs
        u128::MAX, //Uranium mining jobs
        u128::MAX, //Factory jobs
    ]);
}
pub fn cmp(rss:&mut ResourceDict) -> Components{
    let mut cmp = Components::new();
    cmp.init(rss);
    return cmp;
}
pub fn dir() -> Directions{
    return Directions::new();
}
pub fn init_file(){
    crate::ui::io::init();
}