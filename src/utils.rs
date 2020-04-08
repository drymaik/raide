use ron::ser::PrettyConfig;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
/// Ron prettifier
pub fn get_pretty() -> PrettyConfig {
    PrettyConfig {
        depth_limit: 2,
        new_line: "\n".to_string(),
        indentor: "".to_string(),
        separate_tuple_members: true,
        enumerate_arrays: true,
    }
}

/// Tests the first line if it contains valid chars, if not returns that it's invalid UTF-8
pub fn load_file_checked(path: &Path) -> String {
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

// https://stackoverflow.com/questions/45291832/extracting-a-file-extension-from-a-given-path-in-rust-idiomatically
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
