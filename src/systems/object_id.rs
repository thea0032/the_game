#[derive(Clone, Debug, Copy)]
pub struct ObjectID{
    id:usize
}//Wrapper class for object pointer
impl ObjectID{
    pub fn new(id:usize) -> ObjectID{
        return ObjectID{id:id};
    }//New function
    pub fn get(&self) -> usize{
        return self.id;
    }//Getter function
}