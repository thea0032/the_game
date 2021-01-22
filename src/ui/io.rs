use std::{fs::File, io::{stdin, stdout, Write}};
use super::defaults::{BACK, DEL, HELP, NEW, QUIT, TICK};

use super::{from_str::FromString};

use super::{ansi};

const PATH:&str = "stuff";
const PLACEHOLDER:&str = "placeholder";
pub struct Config{
    recording:bool,
    playing:bool,//We assume that at least one of these is false. 
    queue:Vec<String>,
    write_to:File,
    quit:String,
    tick:String,
    delete:String,
    help:String,
    new:String,
    back:String,
}
impl Config{
    pub fn setup() -> Config{
        println!("You will now be prompted to configure how inputting works.");
        let playing:bool = get_from_input_raw("Configuration: Do you want to play what was recorded?", "Please enter true or false.");
        let recording:bool = 
        if !playing{
            get_from_input_raw("Do you want to record?", "Please enter true or false.")
        } else {
            false
        };
        let queue =
        if playing{
            crate::file::ensure_file_exists(PATH);
            crate::file::read_file(PATH.to_string())
        } else {
            Vec::new()
        };
        let write_to = if recording{
            File::create(PATH)
        } else {
            File::create(PLACEHOLDER)
        }.expect("");
        Config{
            recording:recording, 
            playing:playing,
            queue,
            write_to,
            quit:QUIT.to_string(),
            tick:TICK.to_string(),
            delete:DEL.to_string(),
            help:HELP.to_string(),
            new:NEW.to_string(),
            back:BACK.to_string(),
        }
    }
    pub fn quit(&self) -> &str{
        return &self.quit;
    }
    pub fn tick(&self) -> &str{
        return &self.tick;
    }
    pub fn delete(&self) -> &str{
        return &self.delete;
    }
    pub fn help(&self) -> &str{
        return &self.help;
    }
    pub fn new(&self) -> &str{
        return &self.new;
    }
    pub fn back(&self) -> &str{
        return &self.back;
    }
}


fn get_str_raw(msg: &str) -> String {
    print!("{}", msg); //Prints out the message
    let _ = stdout().flush(); //Flushes output
    let mut s = String::new(); //Initializes string
    stdin()
        .read_line(&mut s)
        .expect("Something went horribly wrong!"); //Reads the line
    if let Some('\n') = s.chars().next_back() {
        s.pop(); //Removes newline character
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop(); //Removes carriage return character
    }
    s //Returns the string
}//Raw get string function. Don't mess with this. 
fn get_from_input_raw<T>(msg: &str, err: &str) -> T
where
    T: FromString, {
    loop {
        if let Some(val) = T::from_string_s(&get_str_raw(msg)) {
            //Gets a string. If we can parse it...
            return val; //Return the result!
        }
        println!("{}{}", super::ansi::RED, err); //Displays error message, lets
                                                 // you try again
    }
}//Raw get T function. Don't mess with this. 
pub fn get_str(msg: &str, cfg:&mut Config) -> String {
    print!("{}", msg); //Prints out the message
    let _ = stdout().flush(); //Flushes output
    if cfg.playing {
        //If we're playing the file outputs...
        if cfg.queue.len() != 0{//If the queue is empty...
            return cfg.queue.remove(0);//Gets, removes, and returns the latest input
        } else {
            cfg.playing = false;//We have nothing to play anymore.
            //Nothing is returned, so continues
        }
    }
    let mut s = String::new(); //Initializes a string
    stdin()
        .read_line(&mut s)
        .expect("Something went horribly wrong!"); //Reads the line from standard input
    if let Some('\n') = s.chars().next_back() {
        s.pop(); //Removes newline character
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop(); //Removes carriage return character
    }
    if cfg.recording {//If we're recording...
        cfg.queue.push(s.clone())//Records the input
    }
    refresh(); //Refreshes stuff
    s //Returns the string
}
pub fn wait_for_input(msg: &str, cfg:&mut Config) {
    get_str(msg, cfg);
} //Waits for the user to enter something, and throws it away.
fn refresh() {
    println!("{}", ansi::RESET); //We don't want any lingering colors!
    for _ in 0..100 {
        println!(); //Prints 100 empty lines to make things look really nice.
    }
}
pub fn get_from_input<T>(msg: &str, err: &str, cfg:&mut Config) -> T
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
pub fn get_from_input_valid<T, P>(msg: &str, err: &str, cfg:&mut Config, mut valid: P) -> T
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
