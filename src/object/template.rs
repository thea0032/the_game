use crate::{component::Components, resources::ResourceDict, systems::{Systems, object_id::ObjectID}, ui::resources};


#[derive(Clone)]
pub struct Template {
    component_amounts: Vec<usize>,
    name: String,
    surplus: Vec<i64>,
    storage: Vec<u128>,
    cost: Vec<i64>,
    transfer_cost: Option<u64>,
}
impl Template{
    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn c_amt(&self) -> &Vec<usize>{
        &self.component_amounts
    }
    pub fn cost(&self) -> &Vec<i64>{
        &self.cost
    }
    pub fn storage(&self) -> &Vec<u128>{
        &self.storage
    }
    pub fn surplus(&self) -> &Vec<i64>{
        &self.surplus
    }
    pub fn t_cost(&self) -> Option<u64>{
        self.transfer_cost
    }
    pub fn install(&self, obj:ObjectID, sys:&mut Systems) -> bool{
        if sys.get_o(obj).resources_mut().spend(self.cost()){
            sys.get_o(obj).resources_mut().add_storage_vec(self.storage());
            sys.get_o(obj).resources_mut().add_surplus_vec(self.surplus());
            true
        } else {
            false
        }
    }//Tries to install the template. Returns whether it was successful. 
    pub fn grab(&self, orig:ObjectID, dest:ObjectID, sys:&mut Systems) -> bool{
        match self.transfer_cost{
            Some(val) => {
                let mut real_cost:Vec<i64> = self.cost().clone();
                real_cost[crate::resources::constants::TRANSFER.get()] += val as i64;
                if sys.get_o(orig).resources_mut().spend(&real_cost){
                    sys.get_o(dest).resources_mut().add_storage_vec(self.storage());
                    sys.get_o(dest).resources_mut().add_surplus_vec(self.surplus());
                    true
                } else {
                    false
                }
            },
            None =>false
        }
    }
    pub fn new(component_amounts: Vec<usize>, name: String, surplus: Vec<i64>, storage: Vec<u128>, cost: Vec<i64>, transfer_cost: Option<u64>) -> Template{
        Template{
            component_amounts, name, surplus, storage, cost, transfer_cost
        }
    }
}