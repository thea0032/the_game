pub mod object_id;
pub mod system_id;
pub mod tick;
use crate::object::*;
use crate::systems::object_id::*;
use crate::systems::system_id::*;
use crate::{component::Components, instr::Directions, location::Location, resources::ResourceDict, system::*};
pub struct Systems {
    systems: Vec<System>,
    sys_names: Vec<String>,
    objects: Vec<Object>,
    obj_names: Vec<String>,
    obj_systems: Vec<SystemID>,
} //Contains all of the systems, and all of the objects.
impl Systems {
    pub fn new() -> Systems {
        Systems {
            systems: Vec::new(),
            sys_names: Vec::new(),
            objects: Vec::new(),
            obj_names: Vec::new(),
            obj_systems: Vec::new(),
        }
    } //Basic new function
    pub fn add_s(&mut self, name: String, loc: Location) {
        self.systems.push(System::new(name.clone(), loc));
        self.sys_names.push(name);
    } //Adds a system to the list
    pub fn add_o(&mut self, rss: &ResourceDict, cmp: &Components, dir: &mut Directions, name: String, loc: Location, sys: SystemID) -> ObjectID {
        let obj = Object::new(rss, cmp, name.clone(), loc, sys); //Makes a new object
        self.objects.push(obj); //Adds the object to the list
        self.obj_names.push(name); //Adds its name to the list
        self.obj_systems.push(sys); //Adds its system to the list
        dir.add_new(); //Adds a new set of directions for the object
        let temp = self.objects.len() - 1; //Part 1 of 2
        self.get_s(sys).add_obj(ObjectID::new(temp)); //Marks that the object
                                                      // is inside the correct
                                                      // system
        ObjectID::new(temp) //Returns the object
    } //Adds an object to the list
    pub fn get_s(&mut self, sys: SystemID) -> &mut System {
        &mut self.systems[sys.get()]
    } //Gets a system
    pub fn get_o(&mut self, obj: ObjectID) -> &mut Object {
        &mut self.objects[obj.get()]
    } //Gets an object
    pub fn get_s_stat(&self, sys: SystemID) -> &System {
        &self.systems[sys.get()]
    } //Gets a system immutably
    pub fn get_o_stat(&self, obj: ObjectID) -> &Object {
        &self.objects[obj.get()]
    } //Gets an object immuably
    pub fn get_s_name(&mut self, sys: SystemID) -> String {
        self.sys_names[sys.get()].clone()
    } //Gets a system's name
    pub fn get_o_name(&self, obj: ObjectID) -> String {
        self.obj_names[obj.get()].clone()
    } //Gets an object's name
    pub fn get_o_sys(&self, obj: ObjectID) -> SystemID {
        self.obj_systems[obj.get()]
    } //Gets the system the object is contained
    pub fn get_os(&self, ids: &Vec<ObjectID>) -> Vec<&Object> {
        let mut res: Vec<&Object> = Vec::new();
        for id in ids {
            res.push(&self.objects[id.get()]);
        }
        res
    } //Gets all of the objects
    pub fn get_o_names(&self) -> &Vec<String> {
        &self.obj_names
    } //Gets all of the object names
    pub fn display(&self) -> String {
        let mut result: String = "".to_string();
        for i in 0..self.sys_names.len() {
            result.push_str(&format!(
                "{}{}. {}\n",
                self.get_s_stat(SystemID::new(i)).color(self),
                i,
                self.sys_names[i]
            ));
        }
        result
    } //Displays the systems
    pub fn len(&self) -> usize {
        self.sys_names.len()
    } //Gets the amount of systems
}
