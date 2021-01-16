use crate::component::Components;
use crate::resources::ResourceDict;
use crate::object::Object;

impl Object{
    pub fn display(&self, rss:&ResourceDict, cmp:&Components) -> String{
        let mut res = "".to_string();
        res.push_str(&self.name);
        res.push('\n');
        res.push_str(&format!("Location: ({}, {})\n", self.location.x, self.location.y));
        res.push_str(&self.resources.display(rss, &self.past));
        res.push_str("Components: ");
        res.push_str(&cmp.display_contained(&self.component_amounts));
        return res;
    }
}//Displays the object. Pretty self-explanatory. 