use crate::location::Location;

pub enum Shape{
    Circle(f64, Location),
    Intersect(Vec<Shape>),
    Not(Box<Shape>),
    Any(Vec<Shape>),
}
impl Shape{
    pub fn is_inside(&self, pt:Location) -> bool{
        match self{
            Shape::Circle(val1, val2) => {
                return val2.close_enough(&pt, *val1);
            },
            Shape::Intersect(val) => {
                for line in val{
                    if !line.is_inside(pt){
                        return false;
                    }
                }
                return true;
            },
            Shape::Any(val) => {
                for line in val{
                    if line.is_inside(pt){
                        return true;
                    }
                }
                return false;
            },
            Shape::Not(val) => {
                return !(*val).is_inside(pt);
            }
        }
    }
    pub fn habitable_zone(star_location:Location, intensity:f64) -> Shape{
        return Shape::Intersect(vec![Shape::Circle(2.0 * intensity, star_location), Shape::Not(Box::new(Shape::Circle(intensity, star_location)))]);
    }
}
