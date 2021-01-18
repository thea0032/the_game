use std::fs;
use std::{
    fs::File,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
};

use io::BufReader;

pub fn read_file(path: String) -> Vec<String> {
    // File hosts must exist in current path before this produces output

    read_lines(path)
}
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut result: Vec<String> = vec![];
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
pub fn write<T>(file: &File, contents: T)
where
    T: ToString, {
    let mut f = BufWriter::new(file);
    f.write_all(contents.to_string().as_bytes())
        .expect("Unable to write data");
    f.flush().unwrap();
}
pub fn flush(file: &str) -> File {
    File::create(file).unwrap()
}
pub fn cp(orig: &str, new: &str) {
    fs::copy(orig, new).expect("Couldn't copy files for some reason");
}
