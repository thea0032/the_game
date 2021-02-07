use io::{BufReader};
use std::{fs, iter::Zip, slice::Iter};
use std::{
    fs::File,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
};
#[derive(Clone, Debug, PartialEq)]
pub struct FileObject {
    name: String,
    contents: Vec<FileObject>,
    names: Vec<String>,
}
const WHITESPACE: &str = "    ";
const WHITESPACE_LEN: usize = 4;
impl FileObject {
    pub fn read_from(file: Vec<String>, name: String, tabs:usize) -> FileObject {
        let mut contents: Vec<FileObject> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        let mut buffer: Vec<String> = Vec::new();
        let mut buffer_name: String = "".to_string();
        for mut line in file {
            if line.len() >= WHITESPACE_LEN && &line[0..WHITESPACE_LEN] == WHITESPACE {
                //If an indent exists...
                line.replace_range(0..WHITESPACE_LEN, ""); //Removes the indent
                buffer.push(line); //Adds the line
            } else {
                //If there's no indent...
                let temp: Vec<&str> = line.split(':').collect();
                if temp.len() == 1 {
                    contents.push(FileObject::read_from(buffer, buffer_name, tabs + 1));
                    names.push(temp[0].to_string());
                    buffer_name = temp[0].to_string();
                } else {
                    contents.push(FileObject::read_from(buffer, buffer_name, tabs + 1));
                    buffer_name = temp[1].to_string();
                    names.push(temp[0].to_string());
                }
                buffer = Vec::new();
            }
        }
        if !contents.is_empty() {
            contents.push(FileObject::read_from(buffer, buffer_name, tabs + 1)); //Adds the last bit
            contents.remove(0); //Removes the first object (which doesn't contain anything; this is out of sync)
        }
        FileObject { name, contents, names }
    }
    pub fn blank(name: String) -> FileObject {
        FileObject {
            name,
            contents: Vec::new(),
            names: Vec::new(),
        }
    }
    pub fn merge(&mut self, other: FileObject) {
        for (name, object) in other.names.into_iter().zip(other.contents.into_iter()) {
            if let Some(val) = self.names.iter().position(|x| **x == name) {
                self.contents[val].merge(object);
            } else {
                self.names.push(name.clone());
                self.contents.push(object.clone());
            }
        }
    }
    pub fn get(&self, name: &str) -> Option<&FileObject> {
        for (i, line) in self.names.iter().enumerate() {
            if line == name {
                return Some(&self.contents[i]);
            }
        }
        None
    }
    pub fn grab_contents(&self) -> Zip<Iter<String>, Iter<FileObject>> {
        let v = self.names.iter().zip(self.contents.iter());
        v
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}
#[derive(Debug, Clone)]
pub struct FilePresets {
    asset_path: String,
}
impl FilePresets {
    pub fn new(asset_path: String) -> FilePresets {
        FilePresets { asset_path }
    }
}
pub fn read_file(path: &str, presets: &FilePresets) -> Vec<String> {
    let mut real_path = presets.asset_path.to_string();
    real_path.push_str(path);
    let mut result = read_lines(real_path);
    remove_extras(&mut result);
    result
}
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut result: Vec<String> = Vec::new();
    loop {
        let mut s: String = "".to_string();
        let line = reader.read_line(&mut s);
        result.push(s.clone());
        if line.is_err() {
            return result;
        }
        if s.is_empty() {
            return result;
        }
    }
}
pub fn ensure_file_exists(path: &str, presets: &FilePresets) {
    if File::open(path).is_err() {
        File::create(path).unwrap();
    }
}
pub fn write<T>(file: &File, contents: T, presets: &FilePresets)
where
    T: ToString, {
    let mut f = BufWriter::new(file);
    f.write_all(contents.to_string().as_bytes()).expect("Unable to write data");
    f.flush().unwrap();
}
pub fn flush(file: &str, presets: &FilePresets) -> File {
    File::create(file).unwrap()
}
pub fn cp(orig: &str, new: &str, presets: &FilePresets) {
    fs::copy(orig, new).expect("Couldn't copy files for some reason");
}
pub fn read_folder(presets: &FilePresets, name: &str) -> Vec<Vec<String>> {
    let mut combination: String = "".to_string();
    combination.push_str(&presets.asset_path);
    combination.push_str(name);
    let val = fs::read_dir(combination).expect("Couldn't do this for some reason");
    let mut result = val.map(|x| read_lines(x.unwrap().path())).collect();
    for line in &mut result {
        remove_extras(line);
    }
    result
}
pub fn remove_extras(v: &mut Vec<String>) {
    for s in v {
        if let Some('\n') = s.chars().next_back() {
            s.pop(); //Removes newline character
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop(); //Removes carriage return character
        }
    }
}
