use super::*;
impl ResourceDict {
    pub fn new(vals: Vec<String>, t_costs: Vec<u128>) -> ResourceDict {
        ResourceDict {
            names: vals,
            transfer_costs: t_costs,
        }
    } //Basic new function
    pub fn display_filtered_addon<T>(&self, filter: &Vec<bool>, extra_text: &Vec<T>) -> String
    where
        T: Display, {
        let mut res = "".to_string();
        let mut i = 0;
        for j in 0..self.names.len() {
            if filter[j] {
                res.push_str(&format!("{}: {} ({})\n", i, self.names[j], extra_text[j]));
                i += 1;
            }
        }
        res
    } //An add-on to the display function that helps with filtration
    pub fn len(&self) -> usize {
        self.names.len()
    } //Returns the amount of resources
    pub fn get(&self, id: ResourceID) -> String {
        self.names[id.get()].clone()
    } //Returns the resource name
    pub fn get_transfer_costs(& self) -> & Vec<u128> {
        &self.transfer_costs
    } //Returns all of the transfer costs
}
