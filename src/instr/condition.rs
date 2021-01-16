use crate::{resources::Resources, systems::{Systems, object_id::ObjectID}};
//Conditions - used for advanced logic in-game. Not implemented yet!
#[derive(Debug, Clone)]
pub enum Condition{
    Not(Box<Condition>),//Basic logic
    And(Vec<Condition>),//True if all are true, false otherwise
    Or(Vec<Condition>),//False if all are false, true otherwise
    Has(ObjectID, Resources),//True if the object has certain resources, false otherwise
}
impl Condition{
    pub fn eval(&self, sys:&Systems) -> bool{
        panic!("Not implemented yet!");
    }//Will evaluate a condition
    pub fn display(&self) -> String{
        panic!("Not implemented yet!");
    }//Will display a condition
}