use std::io::{stdin, stdout, Write};
use std::str::FromStr;

use super::ansi;
use crate::constants::*;

/*

*/
pub fn get_str(msg: &str) -> String {
    print!("{}", msg); //Prints out the message
    let _ = stdout().flush(); //Flushes output
    if !INTERACTIVE {
        //If we aren't in interactive mode...
        let output = read_in(); //Reads input from the file
        if !output.is_empty(){
            //If the line isn't empty...
            return output; //Returns the output read
        }
    }
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
    if INTERACTIVE {
        //If we're in interactive mode...
        write_out(&s); //Record input
    }
    refresh(); //Refreshes stuff
    s //Returns the string
}
pub fn read_in() -> String {
    let mut read = crate::file::read_file(PATH.to_string()); //Reads the entire file
    let mut out = read.remove(0); //Removes the line, which we're using
    let file = crate::file::flush(PATH); //Wipes the file clean
    for line in read {
        crate::file::write(&file, line); //Adds the rest of the file back
    }
    if let Some('\n') = out.chars().next_back() {
        out.pop(); //Removes any newline characters
    }
    if let Some('\r') = out.chars().next_back() {
        out.pop(); //Removes any carriage return characters
    }
    out //Returns the output
} //Generates a string from constant file input, based on the PATH variable.
pub fn write_out(msg: &str) {
    let read = crate::file::read_file(PATH2.to_string()); //Reads the entire file
    let file = crate::file::flush(PATH2); //Wipes the file clean
    for line in read {
        crate::file::write(&file, line); //Puts back all non-empty lines
    }
    if msg == "quit" {
        panic!("Successful exit!"); //If we quit, this exits the program
    }
    if msg.is_empty(){
        crate::file::write(&file, "EMPTY"); //So that we don't have any empty
                                            // lines
    }
    crate::file::write(&file, msg); //Enters the message
    crate::file::write(&file, "\n"); //Adds a newline character
}
pub fn init() {
    //Runs at the start
    crate::file::cp(PATH2, PATH); //Copies the second path to the first path (which is going to be destroyed)
    if INTERACTIVE {
        crate::file::flush(PATH2); //If we're recording, wipes the first file
                                   // clean
    }
}
pub fn wait_for_input(msg: &str) {
    get_str(msg);
} //Waits for the user to enter something, and throws it away.
fn refresh() {
    println!("{}", ansi::RESET); //We don't want any lingering colors!
    for _ in 0..100 {
        println!(); //Prints 100 empty lines to make things look really nice.
    }
}
pub fn get_from_input<T>(msg: &str, err: &str) -> T
where
    T: FromStr, {
    loop {
        if let Ok(val) = str::parse::<T>(&get_str(msg)) {
            //Gets a string. If we can parse it...
            return val; //Return the result!
        }
        println!("{}{}", super::ansi::RED, err); //Displays error message, lets
                                                 // you try again
    }
}
pub fn get_from_input_valid<T, P>(msg: &str, err: &str, mut valid: P) -> T
where
    T: FromStr,
    P: FnMut(&T) -> bool, {
    loop {
        if let Ok(val) = str::parse::<T>(&get_str(msg)) {
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
