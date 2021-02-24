use std::{fs::File};
use serde_json::Value;
use serde_json::map::Map;

use crate::{component::Components, file, instr::Directions, object, resources::ResourceDict, systems::Systems, ui::io::get_raw};
pub fn save_game(path: &str, rss:&ResourceDict, cmp:&Components, sys: &Systems, dir: &Directions) -> bool {
    if File::open(path).is_ok() && ! get_raw::<bool>("Are you sure you want to overwrite this file?"){
        println!("Save failed: aborted");
        return false;
    }
    let f = if let Ok(val) = File::create(path){val} else {println!("File creation error!");return false;};
    let mut full:Map<String, Value> = Map::new();
    let resources = if let Ok(val) = serde_json::to_value(rss){val} else {println!("Resources conversion error!");return false;};
    let components = if let Ok(val) = serde_json::to_value(cmp){val} else {println!("Components conversion error!");return false;};
    let systems = if let Ok(val) = serde_json::to_value(sys){val} else {println!("Systems conversion error!");return false;};
    let directions = if let Ok(val) = serde_json::to_value(dir){val} else {println!("Directions conversion error!");return false;};
    full.insert("Resources".to_string(), resources);
    full.insert("Components".to_string(), components);
    full.insert("Systems".to_string(), systems);
    full.insert("Directions".to_string(), directions);
    let as_str = if let Ok(val) = serde_json::to_string(&full) {val} else {println!("Conversion error in final phase!");return false;};
    file::write(&f, as_str);
    println!("Success!");
    return true;
}
