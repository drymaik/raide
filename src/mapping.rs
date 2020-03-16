// (glob, lang)
// extensions should be splitted by ; and compared individually
pub static LANG_TYPES: &[(&str, &str)] = &[
    ("rs", "rust"),
    ("java", "java"),
    ("r", "r"),
    ("c", "c"),
    ("cpp", "cpp"),
    ("go", "go"),
    ("html", "html"),
    ("json", "json"),
    ("jl", "julia"),
    ("tex", "latex"),
    ("lua", "lua"),
    ("m", "matlab"),
    ("php", "php"),
    ("py", "python"),
    ("rb", "ruby"),
    ("toml", "toml"),
    ("xml", "xml"),
    ("md", "markdown"),
    ("ron", "markdown"),
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
