
use super::{ansi, config::Config, context, from_str::{InBounds, MenuRes}, io::{get_from_input_valid, wait_for_input}};

pub fn generic_select<T, P, Cvt>(display: &String, bounds: usize, mut convert: Cvt, cfg: &mut Config, mut paste: P) -> Option<T>
where
    P: FnMut(&mut Config) -> Option<T>,
    Cvt: FnMut(usize) -> Option<T>, {
    loop {
        println!("{}", cfg.display(context::SELECT));
        println!("{}", display);
        let input: MenuRes = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", cfg, |x: &MenuRes| {
            x.in_bounds(&bounds)
        }); //Gets input
        match input {
            MenuRes::Enter(val) => {
                if let Some(val) = convert(val) {
                    return Some(val);
                }
            }
            MenuRes::Exit | MenuRes::Del => {
                return None;
            }
            MenuRes::Paste => {
                if let Some(val) = paste(cfg) {
                    return Some(val);
                }
                wait_for_input(&format!("{}You cannot paste that there!", ansi::RED), cfg);
            }
            _ => {
                wait_for_input(&format!("{}Please enter a valid id", ansi::RED), cfg);
            }
        }
    }
}
pub fn generic_select_simple<T, Cvt>(display: &String, bounds: usize, mut convert: Cvt, cfg: &mut Config) -> Option<T>
where
    Cvt: FnMut(usize) -> Option<T>, {
    loop {
        println!("{}", cfg.display(context::SELECT));
        println!("{}", display);
        let input: MenuRes = get_from_input_valid("Enter the component you want: ", "Please enter a valid id", cfg, |x: &MenuRes| {
            x.in_bounds(&bounds)
        }); //Gets input
        match input {
            MenuRes::Enter(val) => {
                if let Some(val) = convert(val) {
                    return Some(val);
                }
            }
            MenuRes::Exit | MenuRes::Del => {
                return None;
            }
            _ => {
                wait_for_input(&format!("{}Please enter a valid id", ansi::RED), cfg);
            }
        }
    }
}
pub fn menu_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context_all(dis);
    cfg.update_context(Config::PASTE, Some("paste".to_string()), ctx, dis);
    cfg.update_context(Config::QUIT, Some("abort".to_string()), ctx, dis);
}
