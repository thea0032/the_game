use crate::{component::RecipeID, instr::{Instr, Instrs, Queue, Quickie}, resources::Resources, systems::{object_id::ObjectID, system_id::SystemID}};
use crate::object::template::Template;

#[derive(Clone)]
pub enum Clipboard {
    SystemID(SystemID),
    Template(Template),
    Object(ObjectID),
    Instrs(Instrs),
    Queue(Queue),
    Quickie(Quickie),
    Instr(Instr, bool),
    Resources(Resources),
    Recipe(RecipeID),
    None,
}