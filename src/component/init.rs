use crate::resources::constants::*;
use crate::resources::*;
use crate::component::*;

/*
    Planets
*/

pub fn get_all(rss:&ResourceDict) -> Vec<Component>{
    let mut res:Vec<Component> = Vec::new();
    res.push(get_small_body(rss));
    res.push(get_small_planet(rss));
    res.push(get_brown_dwarf(rss));
    res.push(get_red_dwarf(rss));
    res.push(get_yellow_dwarf(rss));
    res.push(get_red_giant(rss));
    res.push(get_black_hole(rss));
    res.push(mineral_deposit(rss));
    res.push(air_pocket(rss));
    res.push(uranium_deposit(rss));
    res.push(water_pocket(rss));
    res.push(biosphere(rss));
    res.push(get_init(rss));
    return res;
}
pub fn get_names() -> Vec<String>{
    let mut res:Vec<String> = Vec::new();
    res.push("small body".to_string());
    res.push("small planet".to_string());
    res.push("brown dwarf".to_string());
    res.push("red dwarf".to_string());
    res.push("yellow dwarf".to_string());
    res.push("red giant".to_string());
    res.push("black hole".to_string());
    res.push("mineral deposit".to_string());
    res.push("air pocket".to_string());
    res.push("uranium deposit".to_string());
    res.push("water pocket".to_string());
    res.push("biosphere".to_string());
    res.push("init".to_string());
    return res;
}
pub mod constants{
    pub const SMALL_BODY:usize = 0;
    pub const SMALL_PLANET:usize = 1;
    pub const BROWN_DWARF:usize = 2;
    pub const RED_DWARF:usize = 3;
    pub const YELLOW_DWARF:usize = 4;
    pub const RED_GIANT:usize = 5;
    pub const BLACK_HOLE:usize = 6;
    pub const MINERAL_DEPOSIT:usize = 7;
    pub const AIR_POCKET:usize = 8;
    pub const URANIUM_DEPOSIT:usize = 9;
    pub const WATER_POCKET:usize = 10;
    pub const BIOSPHERE:usize = 11;
    pub const INIT:usize = 12;
}

fn get_small_body(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(SPACE, -200);
    res.change_cost(MASS, -1000);
    return res;
}
fn get_small_planet(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(SPACE, -2000);
    res.change_cost(MASS, -10000);
    return res;
}
fn get_brown_dwarf(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(MASS, -10000);
    return res;
}
fn get_red_dwarf(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(MASS, -30000);
    return res;
}
fn get_yellow_dwarf(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(MASS, -100000);
    return res;
}
fn get_red_giant(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(MASS, -10000);
    return res;
}
fn get_black_hole(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_cost(MASS, -1000000);
    return res;
}

   /*  
    * Resource deposits 
    */
/*
fn generic(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_surplus(AIR, 0);
    return res;
}//Generic function; helps me when I'm copy pasting
*/
fn mineral_deposit(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_surplus(MINING_JOBS, 3);
    return res;
}
fn air_pocket(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_surplus(AIR, 30);
    return res;
}
fn uranium_deposit(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_surplus(URANIUM_MINING_JOBS, 3);
    return res;
}
fn water_pocket(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_surplus(WATER, 10);
    return res;
}
fn biosphere(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_storage(BIOMASS, 100);
    res.change_cost(BIOMASS, -100);
    return res;
}
fn get_init(rss:&ResourceDict) -> Component{
    let mut res = Component::new(rss.len());
    res.change_storage(SPACE, u128::MAX);
    res.change_storage(MASS, u128::MAX);
    return res;
}