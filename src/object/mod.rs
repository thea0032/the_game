use crate::component::*;
use crate::resources::*;
use crate::{location::*, systems::system_id::*};
mod component;
mod display;
mod tick;
#[derive(Clone, Debug)]
pub struct Object {
    location: Location,              //The object's current location.
    resources: Resources,            //The resources the object has.
    past: Resources,                 //The resources the object had last tick.
    component_amounts: Vec<usize>,   //Tracker for each component.
    h_component_amounts: Vec<usize>, //Tracker for each hidden component.
    name: String,                    //The object's name.
    system: SystemID,                //What system the object's in.
    wait: usize,                     //How long the object's going to wait.
} //The structure for an object. Objects are ships, planets, even projectiles.
impl Object {
    pub fn new(
        rss: &ResourceDict,
        cmp: &Components,
        name: String,
        loc: Location,
        sys: SystemID,
    ) -> Object {
        Object {
            location: loc,
            resources: Resources::new(rss.len()),
            past: Resources::new(rss.len()),
            component_amounts: crate::extra_bits::fill(cmp.list.len(), 0),
            h_component_amounts: crate::extra_bits::fill(cmp.hidden_list.len(), 0),
            name,
            system: sys,
            wait: 0,
        }
    } //Basic constructor
    pub fn get_location(&mut self) -> &mut Location {
        &mut self.location
    } //Basic getter
    pub fn get_location_stat(&self) -> &Location {
        &self.location
    } //Basic getter, immutable
    pub fn get_cmp_amts(&self) -> &Vec<usize> {
        &self.component_amounts
    } //Getter
    pub fn resources(&self) -> &Resources {
        &self.resources
    } //Getter
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    } //Mutable getter
}
