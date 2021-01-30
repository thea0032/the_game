use crate::object::template::Template;
use crate::{
    component::{ComponentID, RecipeID},
    instr::{Instr, Instrs, Queue, Quickie},
    resources::{ResourceID, Resources},
    systems::{object_id::ObjectID, system_id::SystemID},
};

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
    Resource(ResourceID),
    Component(ComponentID),
    None,
}
