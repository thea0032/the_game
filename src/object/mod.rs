use crate::{location::*, systems::system_id::*};
use crate::resources::*;
use crate::component::*;
mod component;
mod tick;
mod display;
#[derive(Clone, Debug)]
pub struct Object{
    location:Location,//The object's current location. 
    resources:Resources,//The resources the object has. 
    past:Resources,//The resources the object had last tick. 
    component_amounts:Vec<usize>,//Tracker for each component. 
    h_component_amounts:Vec<usize>,//Tracker for each hidden component. 
    name:String,//The object's name. 
    system:SystemID,//What system the object's in. 
    wait:usize,//How long the object's going to wait. 
}//The structure for an object. Objects are ships, planets, even projectiles. 
impl Object{
    pub fn new(rss:&ResourceDict, cmp:&Components, name:String, loc:Location, sys:SystemID) -> Object{
        Object{
            location:loc,
            resources:Resources::new(rss.len()),
            past:Resources::new(rss.len()),
            component_amounts:crate::extra_bits::fill(cmp.list.len(), 0),
            h_component_amounts:crate::extra_bits::fill(cmp.hidden_list.len(), 0),
            name:name,
            system:sys,
            wait:0,
        }
    }//Basic constructor
    pub fn get_location<'a>(&'a mut self) -> &'a mut Location{
        return &mut self.location;
    }//Basic getter
    pub fn get_location_stat<'a>(&'a self) -> &'a Location{
        return &self.location;
    }//Basic getter, immutable
    pub fn get_cmp_amts<'a>(&'a self) -> &'a Vec<usize>{
        return &self.component_amounts;
    }//Getter
    pub fn resources<'a>(&'a self) -> &Resources{
        return &self.resources;
    }//Getter
    pub fn resources_mut<'a>(&'a mut self) -> &mut Resources{
        return &mut self.resources;
    }//Mutable getter
}