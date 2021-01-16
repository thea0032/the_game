#[derive(Clone, Debug, Copy)]
pub struct SystemID{
    id:usize
}//Wrapper class for system pointer
impl SystemID{
    pub fn new(id:usize) -> SystemID{
        return SystemID{id:id};
    }
    pub fn get(&self) -> usize{
        return self.id;
    }
}