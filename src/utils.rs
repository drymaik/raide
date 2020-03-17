use ron::ser::PrettyConfig;
use std::path::{Path};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
/// Ron prettifier
pub fn get_pretty() -> PrettyConfig {
    PrettyConfig {
        depth_limit: 2,
        new_line: "\n".to_string(),
        indentor: "    ".to_string(),
        separate_tuple_members: true,
        enumerate_arrays: true,
    }
}

/// Tests the first line if it contains valid chars, if not returns that its invalid UTF-8.
/// Then the user can correct the errors
pub fn load_invalid_file(path: &Path) -> String {
    let file = File::open(path).expect("Can't load file from path");
    let mut reader = BufReader::new(file);
    let mut result = String::new();

    match reader.read_line(&mut result) {
        Ok(_) => {
            let data = fs::read_to_string(path).expect("Unable to read file");
            data
        }
        Err(_error) => {
            return "File is not encoded in UTF-8!".to_string();
        }
    }
}

pub fn load_good_file(path: &Path) -> String {
    let mut file = File::open(path).expect("Can't load file from path");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Reading failed");
    contents
}
