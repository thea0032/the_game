use crate::file::FileObject;

pub trait Saveable {
    fn save(&self, name: &str) -> FileObject;
}
impl Saveable for dyn ToString {
    fn save(&self, name: &str) -> FileObject {
        FileObject::assemble(self.to_string(), Vec::new(), Vec::new())
    }
}
pub fn save_vec<T>(v: &Vec<T>, names: &Vec<&str>) -> Vec<FileObject>
where
    T: Saveable, {
    v.iter().zip(names).map(|(x, y)| x.save(y)).collect()
}
pub fn save_vec_one<T>(v: &Vec<T>, name: &str) -> FileObject
where
    T: ToString, {
    FileObject::assemble(
        name.to_string(),
        v.iter().map(|x| x.to_string()).collect(),
        v.iter().map(|x| FileObject::blank(x.to_string())).collect(),
    )
}
