/*
    Lists all accessible components. Sooner or later, I'll change it to a text-based version. 
*/

use crate::resources::constants::*;
use crate::resources::*;
use crate::component::*;
pub fn get_names() -> Vec<String>{
    let mut res:Vec<String> = Vec::new();
    res.push("hull".to_string());//0
    res.push("engine".to_string());//1
    res.push("battery".to_string());//2
    res.push("living quarters".to_string());//3
    res.push("storage space".to_string());//4
    res.push("reactor".to_string());//5
    res.push("solar panels".to_string());//6
    res.push("hydroponics farm".to_string());//7
    res.push("soil farm".to_string());//8
    res.push("factory".to_string());//9
    res.push("fuel plant".to_string());//10
    res.push("fuel burner".to_string());//11
    res.push("fuel tank".to_string());//12
    return res;
}//All component names listed here
pub fn get_all(rss:&ResourceDict) -> Vec<Component>{
    let mut res:Vec<Component> = Vec::new();
    res.push(get_hull(rss));//0
    res.push(get_engine(rss));//1
    res.push(get_battery(rss));//2
    res.push(get_living_quarters(rss));//3
    res.push(get_storage(rss));//4
    res.push(get_reactor(rss));//5
    res.push(get_solar_panels(rss));//6
    res.push(get_hydroponics_farm(rss));//7
    res.push(get_soil_farm(rss));//8
    res.push(get_factory(rss));//9
    res.push(get_fuel_plant(rss));//10
    res.push(get_fuel_burner(rss));//11
    res.push(get_fuel_tank(rss));//12
    return res;
}//All components listed here
pub mod constants{
    pub const HULL:usize = 0;
    pub const ENGINE:usize = 1;
    pub const BATTERY:usize = 2;
    pub const LIVING_QUARTERS:usize = 3;
    pub const STORAGE_SPACE:usize = 4;
    pub const REACTOR:usize = 5;
    pub const SOLAR_PANELS:usize = 6;
    pub const HYDROPONICS_FARM:usize = 7;
    pub const SOIL_FARM:usize = 8;
    pub const FACTORY:usize = 9;
    pub const FUEL_PLANT:usize= 10;
    pub const FUEL_BURNER:usize= 11;
    pub const FUEL_TANK:usize= 12;
}//Constants for use in this program. 
pub fn get_hull(rss:&ResourceDict) -> Component{//Sample component:
    let mut res = Component::new(rss.len());//Initializes component
    res.change_cost(METAL, 500);//Changes the metal cost to 500
    res.change_cost(SPACE, -5);//Changes the space cost to -5 (gain 5 space)
    res.change_cost(MASS, -3);//Changes the mass cost to -3 (gain 3 mass)
    res.change_cost(PRODUCTION, 3);//Changes the production cost to 3
    return res;//returns the component. 
}
pub fn get_battery(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_storage(ENERGY, 500);
    return res;
}
pub fn get_reactor(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_surplus(URANIUM, -1);
    res.change_surplus(ENERGY, 20);
    return res;
}
pub fn get_solar_panels(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 1000);
    res.change_surplus(ENERGY, 10);
    res.change_cost(SPACE, 0);
    res.change_cost(MASS, -6);
    res.change_cost(PRODUCTION, 5);
    return res;
}
pub fn get_living_quarters(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(MASS, - 3);
    res.change_cost(PRODUCTION, 1);
    res.change_storage(POPULATION, 100);
    return res;
}
pub fn get_hydroponics_farm(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_storage(BIOMASS, 200);
    res.change_surplus(ENERGY, -5);
    return res;
}
pub fn get_soil_farm(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 10);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_storage(BIOMASS, 200);
    return res;
}
pub fn get_storage(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 2);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 0);
    res.change_storage(FOOD, 500);
    res.change_storage(WATER, 500);
    res.change_storage(AIR, 500);
    res.change_storage(METAL, 500);
    res.change_storage(ORE, 500);
    res.change_storage(LUXURIES, 500);
    return res;
}
pub fn get_factory(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_surplus(FACTORY_JOBS, 3);
    return res;
}
pub fn get_engine(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_surplus(ENERGY, -5);
    res.change_surplus(MOVEMENT, 5);
    return res;
}
pub fn get_fuel_plant(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_surplus(ENERGY, -10);
    res.change_surplus(WATER, -10);
    res.change_surplus(FUEL, 2);
    return res;
}
pub fn get_fuel_burner(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_surplus(FUEL, -2);
    res.change_surplus(ENERGY, 10);
    res.change_surplus(WATER, 10);
    return res;
}
pub fn get_fuel_tank(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    res.change_storage(FUEL, 500);
    return res;
}
pub fn get_generic(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(METAL, 500);
    res.change_cost(SPACE, 1);
    res.change_cost(MASS, -3);
    res.change_cost(PRODUCTION, 3);
    return res;
}
pub fn get_empty(rss:&ResourceDict) -> Component{
    let res = Component::new(rss.len());
    return res;
}