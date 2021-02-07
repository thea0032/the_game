use std::{fmt, fs::File, io::{stdin, stdout, Write}, str::FromStr};

use fmt::Debug;

use crate::{extra_bits, file::FilePresets};

use super::{clipboard::Clipboard, io};
use super::defaults;

const PATH: &str = "stuff";
pub struct Config {
    play_queue: Vec<String>,    //Stuff being played will appear here.
    line_queue: Vec<String>,    //Stuff being parsed will appear here.
    write_to: Option<File>,     //The file we write to.
    keys: Vec<Key>,             //Each key, and what it does.
    cpb2: Vec<Clipboard>,       //Clipboard.
    cpb: Clipboard,             //A clipboard
    contexts: Vec<Vec<String>>, //Contexts
    display: Vec<Vec<bool>>,    //Whether these functions are displayed, based on context
    file_presets: FilePresets,  //Presets for files
    buffer: String,             //A buffer
} //The configuration structure. Contains a variety of things.

impl Config {
    pub const QUIT: usize = 0;
    pub const TICK: usize = 1;
    pub const DELETE: usize = 2;
    pub const NEW: usize = 3;
    pub const HELP: usize = 4;
    pub const SEP: usize = 5;
    pub const COPY: usize = 6;
    pub const PASTE: usize = 7;
    pub const INFO: usize = 8;
    pub const FUNCTIONS: &'static [&'static str] = &["quit", "tick", "delete", "new", "help", "separate", "copy", "paste", "information"]; //Constants that correspond to keys and give data
    pub fn setup(presets: FilePresets) -> Config {
        println!("You will now be prompted to configure how inputting works."); //We configure how inputting works:
        let playing: bool = get_from_input_raw("Configuration: Do you want to play what was recorded?", "Please enter true or false."); //If we're playing some input
        let recording: bool = if !playing {
            get_from_input_raw("Do you want to record?", "Please enter true or false.") //If we're recording
        } else {
            false //If we're playing, we're not recording.
        };
        let queue = if playing {
            //If we're
            crate::file::ensure_file_exists(PATH, &presets);
            crate::file::read_file(PATH, &presets)
        } else {
            Vec::new()
        };
        let write_to = if recording { Some(File::create(PATH).expect("")) } else { None };
        let mut cfg = Config {
            play_queue: queue,
            line_queue: Vec::new(),
            write_to,
            keys: Vec::new(),
            cpb: Clipboard::None,
            cpb2: Vec::new(),
            contexts: Vec::new(),
            display: Vec::new(),
            file_presets: presets,
            buffer: String::new(),
        }; //Initializes config
        super::context::init(&mut cfg);
        cfg.keys.push(Key {
            id: defaults::QUIT.to_string(),
            show: true,
        });
        cfg.keys.push(Key {
            id: defaults::TICK.to_string(),
            show: true,
        });
        cfg.keys.push(Key {
            id: defaults::DEL.to_string(),
            show: true,
        });
        cfg.keys.push(Key {
            id: defaults::NEW.to_string(),
            show: true,
        });
        cfg.keys.push(Key {
            id: defaults::HELP.to_string(),
            show: true,
        });
        cfg.keys.push(Key {
            id: defaults::SEP.to_string(),
            show: false,
        });
        cfg.keys.push(Key {
            id: defaults::COPY.to_string(),
            show: true,
        });
        cfg.keys.push(Key {
            id: defaults::PASTE.to_string(),
            show: true,
        }); //Adds default keys
        cfg.keys.push(Key {
            id: defaults::INFO.to_string(),
            show: true,
        }); //Adds default keys
        cfg.configure(); //Allows you to configure it
        cfg //Returns
    }
    pub fn quit(&self) -> &Key {
        &self.keys[Self::QUIT]
    } //Returns the key
    pub fn tick(&self) -> &Key {
        &self.keys[Self::TICK]
    } //Returns the key
    pub fn delete(&self) -> &Key {
        &self.keys[Self::DELETE]
    } //Returns the key
    pub fn help(&self) -> &Key {
        &self.keys[Self::HELP]
    } //Returns the key
    pub fn new_key(&self) -> &Key {
        &self.keys[Self::NEW]
    } //Returns the key
    pub fn copy(&self) -> &Key {
        &self.keys[Self::COPY]
    } //Returns the key
    pub fn paste(&self) -> &Key {
        &self.keys[Self::PASTE]
    } //Returns the key
    pub fn sep(&self) -> &Key {
        &self.keys[Self::SEP]
    } //Returns the key
    pub fn info(&self) -> &Key {
        &self.keys[Self::INFO]
    } //Returns the key
    pub fn play_queue(&mut self) -> &mut Vec<String> {
        &mut self.play_queue
    }
    pub fn line_queue(&mut self) -> &mut Vec<String> {
        &mut self.line_queue
    }
    pub fn write_to(&mut self) -> &mut Option<File> {
        &mut self.write_to
    }
    pub fn write_to_stat(&self) -> &Option<File> {
        &self.write_to
    }
    pub fn presets(&self) -> &FilePresets {
        &self.file_presets
    }
    pub const EMPTY: &'static str = "EMPTY\0\t"; //Empty string; can't be replicated
    pub fn configure(&mut self) {
        loop {
            println!("You will now be directed to configure the interface. Choose the key you want to modify: ");
            for (i, line) in self.keys.iter().enumerate() {
                println!(
                    "{}. {}: {} ({})",
                    i,
                    Self::FUNCTIONS[i],
                    line.id(),
                    if line.show() { "shown" } else { "hidden" }
                );
            }
            println!("{}. Quit", self.keys.len());
            let res: usize = get_from_input_raw("Please enter something: ", "You've done something wrong! Try again.");
            if res >= self.keys.len() {
                break;
            } else {
                let mut response: String = get_from_input_raw("Enter what you want the new key to be. ", "Try again. It should have worked.");
                if response.is_empty() {
                    response = Self::EMPTY.to_string();
                } //Makes sure that the response isn't empty. If it is, makes sure that it can never
                  // be entered.
                for line in &mut self.keys {
                    if line.id() == response {
                        *(line.id_mut()) = Self::EMPTY.to_string(); //Resets any overwritten keys.
                    }
                }
                let obtained = &mut self.keys[res];
                *(obtained.id_mut()) = response;
                let response2: bool = get_from_input_raw("Do you want this to be shown?", "Please enter true or false.");
                *(obtained.show_mut()) = response2;
            }
        }
    }
    pub fn generate_context(&self) -> Vec<String> {
        Self::FUNCTIONS.iter().map(|x| x.to_string()).collect()
    }
    pub fn generate_display(&self) -> Vec<bool> {
        extra_bits::fill(Self::FUNCTIONS.len(), true)
    }
    pub fn update_context(&self, id: usize, new: Option<String>, curr: &mut Vec<String>, will_display: &mut Vec<bool>) {
        match new {
            Some(val) => {
                curr[id] = val;
                will_display[id] = true;
            }
            None => will_display[id] = false,
        }
    }
    pub fn update_context_all(&self, will_display: &mut Vec<bool>) {
        for line in will_display {
            *line = false;
        }
    }
    pub fn display(&mut self, context: usize) -> String {
        let mut res: String = "".to_string();
        for (i, key) in self.keys.iter().enumerate() {
            if key.show() && self.display[context][i] {
                res.push_str(&format!("{}. {}\n", key.id(), self.contexts[context][i]));
            }
        }
        res
    }
    pub fn add_context(&mut self, update: fn(&mut Vec<String>, &mut Vec<bool>, &Config)) {
        let mut ctx = self.generate_context();
        let mut dis = self.generate_display();
        update(&mut ctx, &mut dis, &self);
        self.contexts.push(ctx);
        self.display.push(dis);
    }
    pub fn clipboard(&mut self, v: Option<usize>) -> &mut Clipboard {
        if let Some(v) = v {
            if v >= self.cpb2.len() {
                for _ in self.cpb2.len()..=v {
                    self.cpb2.push(Clipboard::None);
                }
            }
            &mut self.cpb2[v]
        } else {
            &mut self.cpb
        }
    }
    pub fn println(&mut self, args: fmt::Arguments<'_>){
        let res = std::fmt::format(args);//formats the string
        println!("{:?}", res);
        self.buffer.push_str(&res);
    }
}
pub struct Key {
    id: String,
    show: bool,
}
impl Key {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }
    pub fn show(&self) -> bool {
        self.show
    }
    pub fn show_mut(&mut self) -> &mut bool {
        &mut self.show
    }
}
pub fn get_str_raw(msg: &str) -> String {
    print!("{}", msg); //Prints out the message
    let _ = stdout().flush(); //Flushes output
    let mut s = String::new(); //Initializes string
    stdin().read_line(&mut s).expect("Something went horribly wrong!"); //Reads the line
    if let Some('\n') = s.chars().next_back() {
        s.pop(); //Removes newline character
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop(); //Removes carriage return character
    }
    s //Returns the string
} //Raw get string function. Used while creating a configuration. Don't mess with this.
fn get_from_input_raw<T>(msg: &str, err: &str) -> T
where
    T: FromStr, {
    loop {
        if let Ok(val) = T::from_str(&get_str_raw(msg)) {
            //Gets a string. If we can parse it...
            return val; //Return the result!
        }
        println!("{}{}", super::ansi::RED, err); //Displays error message, lets
                                                 // you try again
    }
} //Raw get T function. Don't mess with this.
