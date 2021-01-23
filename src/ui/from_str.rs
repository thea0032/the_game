use crate::init::config;

use super::config::Config;
use std::str::FromStr;
pub trait FromString {
    fn from_string(s: &str, cfg: &mut Config) -> Option<Self>
    where
        Self: Sized;
}
pub struct BooleanDefYes {
    pub b: bool,
}
impl FromString for BooleanDefYes {
    fn from_string(s: &str, _: &mut Config) -> Option<Self> {
        match s {
            "n" | "N" | "no" | "No" | "NO" | "false" => Some(BooleanDefYes { b: false }),
            _ => Some(BooleanDefYes { b: true }),
        }
    }
}
pub struct BooleanDefNo {
    pub b: bool,
}
impl FromString for BooleanDefNo {
    fn from_string(s: &str, _: &mut Config) -> Option<Self> {
        match s {
            "y" | "Y" | "yes" | "Yes" | "YES" | "true" => Some(BooleanDefNo { b: true }),
            _ => Some(BooleanDefNo { b: false }),
        }
    }
}
pub trait InBounds {
    fn in_bounds(&self, bounds: &usize) -> bool;
}
impl FromString for usize {
    fn from_string(s: &str, _: &mut Config) -> Option<Self> {
        if let Ok(val) = usize::from_str(s) {
            return Some(val);
        }
        None
    }
}
impl FromString for f64 {
    fn from_string(s: &str, _: &mut Config) -> Option<Self> {
        if let Ok(val) = f64::from_str(s) {
            return Some(val);
        }
        None
    }
}
impl FromString for u128 {
    fn from_string(s: &str, _: &mut Config) -> Option<Self> {
        if let Ok(val) = u128::from_str(s) {
            return Some(val);
        }
        None
    }
}
impl FromString for bool {
    fn from_string(s: &str, _: &mut Config) -> Option<Self> {
        match s {
            "y" | "Y" | "yes" | "Yes" | "YES" | "true" => Some(true),
            "n" | "N" | "no" | "No" | "NO" | "false" => Some(false),
            _ => None,
        }
    }
}

pub enum MenuRes {
    Enter(usize),
    Tick,
    Exit,
    Del,
    New,
    Copy,
    Paste,
} //
impl FromString for MenuRes {
    fn from_string(s: &str, cfg: &mut Config) -> Option<Self> {
        if s == cfg.tick().id() {
            Some(MenuRes::Tick)
        } else if s == cfg.quit().id() {
            Some(MenuRes::Exit)
        } else if s == cfg.delete().id() {
            Some(MenuRes::Del)
        } else if s == cfg.new_key().id() {
            Some(MenuRes::New)
        } else if s == cfg.copy().id() {
            Some(MenuRes::Copy)
        } else if s == cfg.paste().id() {
            Some(MenuRes::Paste)
        } else if let Ok(val) = usize::from_str(s) {
            Some(MenuRes::Enter(val))
        } else {
            None
        }
    }
}
impl InBounds for MenuRes {
    fn in_bounds(&self, bounds: &usize) -> bool {
        if let MenuRes::Enter(val) = self {
            val < bounds
        } else {
            true
        }
    }
}
