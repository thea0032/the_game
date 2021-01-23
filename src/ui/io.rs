
use std::{
    io::{stdin, stdout, Write},
};

use crate::file::FilePresets;

use super::defaults;

use super::from_str::FromString;

use super::config::Config;

use super::ansi;

pub fn get_str_unparsed(msg: &str, cfg: &mut Config) -> String {
    print!("{}", msg); //Prints out the message
    let _ = stdout().flush(); //Flushes output
    if !cfg.play_queue().is_empty(){
        //If the queue isn't empty...
        return cfg.play_queue().remove(0); //Gets, removes, and returns the latest input
    }
    let mut s = String::new(); //Initializes a string
    stdin().read_line(&mut s).expect("Something went horribly wrong!"); //Reads the line from standard input
    if let Some('\n') = s.chars().next_back() {
        s.pop(); //Removes newline character
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop(); //Removes carriage return character
    }
    if let Some(val) = cfg.write_to() {
        //If we're recording...
        crate::file::write(val, s.clone());
    }
    refresh(); //Refreshes stuff
    s //Returns the string
} //The basic get_str function, w/o parsing.
pub fn get_str(msg: &str, cfg: &mut Config) -> String {
    if !cfg.line_queue().is_empty() {
        return cfg.line_queue().remove(0);
    }
    let s = get_str_unparsed(msg, cfg); //Gets the string unparsed.
    if s == cfg.help().id() {
        //Begins parsing: If it's the help key, enters the configuration menu.
        cfg.configure();
        return get_str(msg, cfg);
    }
    let v: Vec<&str> = s.split(cfg.sep().id()).collect(); //Separates lines based on semicolons.
    for line in v {
        cfg.line_queue().push(line.to_string());
    }
    get_str(msg, cfg)
} //The advanced input.
pub fn wait_for_input(msg: &str, cfg: &mut Config) {
    get_str(msg, cfg);
} //Waits for the user to enter something, and throws it away.
fn refresh() {
    println!("{}", ansi::RESET); //We don't want any lingering colors!
    for _ in 0..100 {
        println!(); //Prints 100 empty lines to make things look really nice.
    }
}
pub fn get_from_input<T>(msg: &str, err: &str, cfg: &mut Config) -> T
where
    T: FromString, {
    loop {
        if let Some(val) = T::from_string(&get_str(msg, cfg), cfg) {
            //Gets a string. If we can parse it...
            return val; //Return the result!
        }
        println!("{}{}", super::ansi::RED, err); //Displays error message, lets
                                                 // you try again
    }
}
pub fn get_from_input_valid<T, P>(msg: &str, err: &str, cfg: &mut Config, mut valid: P) -> T
where
    T: FromString,
    P: FnMut(&T) -> bool, {
    loop {
        if let Some(val) = T::from_string(&get_str(msg, cfg), cfg) {
            //Gets a string. If we can parse it...
            if valid(&val) {
                //If it's valid...
                return val; //Return the result!
            }
        }
        println!("{}{}", super::ansi::RED, err); //If the input cannot be
                                                 // parsed, or is invalid,
                                                 // displays error, lets you try
                                                 // again
    }
}
