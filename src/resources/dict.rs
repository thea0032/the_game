use super::*;
impl ResourceDict {
    pub fn new(
        vals: Vec<String>, t_costs: Vec<u64>, growth: HashMap<ResourceID, f64>, requirements: HashMap<ResourceID, HashMap<ResourceID, f64>>,
        transfer: Option<ResourceID>,
    ) -> ResourceDict {
        ResourceDict {
            names: vals,
            transfer_costs: t_costs,
            growth,
            requirements,
            transfer_resource: transfer,
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
    pub fn get_transfer_costs(&self) -> &Vec<u64> {
        &self.transfer_costs
    } //Returns all of the transfer costs
    pub fn find(&self, name: &str) -> Option<ResourceID> {
        Some(ResourceID::new(self.names.iter().position(|x| x == name)?))
    }
    pub fn get_growth(&self) -> &HashMap<ResourceID, f64> {
        &self.growth
    }
    pub fn get_requirements(&self) -> &HashMap<ResourceID, HashMap<ResourceID, f64>> {
        &self.requirements
    }
    pub fn get_transfer(&self) -> Option<ResourceID> {
        self.transfer_resource
    }
}
const RSSDICT: &str = "Resources";
const MOVE_COST: &str = "Move Cost";
impl Saveable for ResourceDict {
    fn save(&self, name: &str) -> FileObject {
        let mut names: Vec<String> = Vec::new();
        let mut contents: Vec<FileObject> = Vec::new();
        for (i, line) in self.names.iter().enumerate() {
            names.push(line.to_string());
            contents.push(FileObject::assemble(
                line.to_string(),
                vec![MOVE_COST.to_string()],
                vec![FileObject::blank(if self.transfer_costs[i] == u64::MAX {
                    crate::init::MAX.to_string()
                } else {
                    self.transfer_costs[i].to_string()
                })],
            ));
        }

        FileObject::assemble(RSSDICT.to_string(), names, contents)
    }
}
impl ResourceDict {
    fn save_2(&self, name: &str) -> FileObject {
        unimplemented!()
    }
}
//Well, crap. I can't use the trait here because this is a RARE EXCEPTION where one object is made from
// 2 fileObjects.
