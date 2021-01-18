use crate::location::Location;

use super::io::get_from_input;

pub fn get_location() -> Location{
    let x:f64 = get_from_input("Where do you want to put it (x)?", "please enter a valid number");//Gets x location from input
    let y:f64 = get_from_input("Where do you want to put it (y)?", "please enter a valid number");//Gets y location from input
    return Location::new(x, y);//Returns the result
}//Returns a location from input