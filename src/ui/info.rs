use crate::{component::Components, resources::ResourceDict};

use super::{
    ansi,
    component::{detail, details},
    config::Config,
    context::INFO,
    from_str::{InBounds, MenuRes},
    io::{get_from_input_valid, wait_for_input},
    recipe::{r_detail, r_details},
};

pub fn information(rss: &ResourceDict, cmp: &mut Components, cfg: &mut Config) {
    loop {
        println!("{}", cfg.display(INFO));
        println!("What do you want to get information on?");
        println!("0. Recipes");
        println!("1. A specific recipe");
        println!("2. Components");
        println!("3. A specific component");
        let len: usize = 4;
        let input = get_from_input_valid("", "Please enter a valid input", cfg, |x: &MenuRes| x.in_bounds(&len));
        match input {
            MenuRes::Enter(0) => {
                r_details(rss, cmp, cfg);
            }
            MenuRes::Enter(1) => {
                r_detail(rss, cmp, cfg);
            }
            MenuRes::Enter(2) => {
                details(rss, cmp, cfg);
            }
            MenuRes::Enter(3) => detail(rss, cmp, cfg),
            MenuRes::Exit => break,
            MenuRes::Del => break,
            _ => {
                wait_for_input(&format!("{}Please enter a valid input!", ansi::RED), cfg);
            }
        }
    }
}

pub fn info_context(ctx: &mut Vec<String>, dis: &mut Vec<bool>, cfg: &Config) {
    cfg.update_context_all(dis);
    cfg.update_context(Config::QUIT, Some("exit".to_string()), ctx, dis);
}
