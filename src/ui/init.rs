use crate::{file::FilePresets, save::Package};
use super::io::get_str_raw;
use crate::file::file_object::FileObject;

pub fn start_game(presets: &FilePresets) -> Option<Package> {
    println!("Press n to start a new game. Press l to load a game. Press q to quit.");
    match &*get_str_raw("") {
        "n" => Some(new_game(presets)),
        "l" => load_game(presets),
        _ => None
    }
}
pub fn new_game(presets: &FilePresets) -> Package {
    println!("You will now be prompted to select the folders you want to use to create a game. ");
    println!("These folders should be like the base folder in the assets folder: all text files in the fileobject format. ");
    println!("Selecting a folder with invalid contents will result in a crash. ");
    println!("To select a folder, select a file directly inside the folder. ");
    let mut folders:Vec<String> = Vec::new();
    loop {
        let val = crate::file::get_file(&presets.path());
        if let Ok(val) = val {
            let mut temp:Vec<&str> = val.split("\\").collect();
            println!("{:?}", temp);
            temp.pop();
            println!("{:?}", temp);
            let mut path = temp.join("\\");
            path.push('\\');
            folders.push(path);
        }
        println!("Current folders: {:?}", folders);
        let val = get_str_raw("Do you want to add another?");
        if val == "n" || val == "false" {break;}
    }
    let folders:Vec<&str> = folders.iter().map(|x|&**x).collect();
    let file_res: FileObject = crate::init::load(folders);
    crate::init::generate_package(&file_res)
}
pub fn load_game(presets: &FilePresets) -> Option<Package> {
    match crate::file::get_file(&presets.path()) {
        Ok(val) => match crate::save::load(&val) {
            Ok(val) => return Some(val),
            Err(val) => println!("{:?}", val),
        },
        Err(val) => println!("{:?}", val),
    } 
    None
}