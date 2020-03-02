//pub mod mapping;
use std::collections::HashMap;

pub static LANG_TYPES: &[(&str, &str)] = &[
    ("rs", "rust"),
    ("java", "java")
];

pub fn get_by_left(glob: &str) -> Option<&str> {
    for i in LANG_TYPES {
        if glob == i.0 {
            return Some(i.1);
        } 
    }
    None
}
pub fn get_by_right(lang: &str) -> Option<&str> {
    for j in LANG_TYPES {
        if lang == j.1 {
            return Some(j.0);
        }
    }
    None
}