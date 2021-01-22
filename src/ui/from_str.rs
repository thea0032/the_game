use super::io::Config;
use std::str::FromStr;
pub trait FromString{
    fn from_string(s:&str, cfg:&mut Config) -> Option<Self> where Self:Sized;
    fn from_string_s(s:&str) -> Option<Self> where Self:Sized;
}
pub struct BooleanDefYes{
    pub b:bool
}
impl FromString for BooleanDefYes{
    fn from_string(s:&str, _:&mut Config) -> Option<Self>{
        match s{
            "n" | "N" | "no" | "No" | "NO" | "false" => {
                Some(BooleanDefYes{b:false})
            }
            _=>Some(BooleanDefYes{b:true})
        }
    }
    fn from_string_s(s:&str) -> Option<Self>{
        match s{
            "n" | "N" | "no" | "No" | "NO" | "false" => {
                Some(BooleanDefYes{b:false})
            }
            _=>Some(BooleanDefYes{b:true})
        }
    }
}
pub struct BooleanDefNo{
    pub b:bool
}
impl FromString for BooleanDefNo{
    fn from_string(s:&str, _:&mut Config) -> Option<Self>{
        match s{
             "y" | "Y" | "yes" | "Yes" | "YES" | "true" => {
                Some(BooleanDefNo{b:true})
            },
            _=>Some(BooleanDefNo{b:false})
        }
    }
    fn from_string_s(s:&str) -> Option<Self>{
        match s{
             "y" | "Y" | "yes" | "Yes" | "YES" | "true" => {
                Some(BooleanDefNo{b:true})
            },
            _=>Some(BooleanDefNo{b:false})
        }
    }
}
pub trait InBounds{
    fn in_bounds(&self, bounds:&usize) -> bool;
}
impl FromString for usize{
    fn from_string(s:&str, _:&mut Config) -> Option<Self>{
        if let Ok(val) = usize::from_str(s){
            return Some(val);
        }
        return None;
    }
    fn from_string_s(s:&str) -> Option<Self>{
        if let Ok(val) = usize::from_str(s){
            return Some(val);
        }
        return None;
    }
}
impl FromString for f64{
    fn from_string(s:&str, _:&mut Config) -> Option<Self>{
        if let Ok(val) = f64::from_str(s){
            return Some(val);
        }
        return None;
    }
    fn from_string_s(s:&str) -> Option<Self>{
        if let Ok(val) = f64::from_str(s){
            return Some(val);
        }
        return None;
    }
}
impl FromString for u128{
    fn from_string(s:&str, _:&mut Config) -> Option<Self>{
        if let Ok(val) = u128::from_str(s){
            return Some(val);
        }
        return None;
    }
    fn from_string_s(s:&str) -> Option<Self>{
        if let Ok(val) = u128::from_str(s){
            return Some(val);
        }
        return None;
    }
}
impl FromString for bool{
    fn from_string(s:&str, _:&mut Config) -> Option<Self>{
        match s{
             "y" | "Y" | "yes" | "Yes" | "YES" | "true" => {
                Some(true)
            },
            "n" | "N" | "no" | "No" | "NO" | "false" => {
                Some(false)
            }
            _=>None,
        }
    }
    fn from_string_s(s:&str) -> Option<Self>{
        match s{
             "y" | "Y" | "yes" | "Yes" | "YES" | "true" => {
                Some(true)
            },
            "n" | "N" | "no" | "No" | "NO" | "false" => {
                Some(false)
            }
            _=>None,
        }
    }
}