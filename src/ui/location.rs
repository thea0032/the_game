use crate::location::Location;

use super::io::{Config, get_from_input};

pub fn get_location(cfg:&mut Config) -> Location {
    let x: f64 = get_from_input(
        "Where do you want to put it (x)?",
        "please enter a valid number",
        cfg
    ); //Gets x location from input
    let y: f64 = get_from_input(
        "Where do you want to put it (y)?",
        "please enter a valid number",
        cfg
    ); //Gets y location from input
    Location::new(x, y) //Returns the result
} //Returns a location from input
