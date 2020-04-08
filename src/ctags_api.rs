use crate::utils::{get_extension_from_filename, load_file_checked};
use ron::de::from_str;
use ron::ser::to_string_pretty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::fs::{create_dir, remove_file, write};
use std::io::prelude::*;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
// minimized ron files
use crate::mapping::file_extension_to_lang;
use crate::utils::get_pretty;
use ron::ser::PrettyConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagLine {
    pub tag: String,
    pub file: String,
    pub address: String,
    pub kind: String,
}

impl TagLine {
    pub fn new(tag: String, file: String, address: String, kind: String) -> TagLine {
        TagLine {
            tag,
            file,
            address,
            kind,
        }
    }
}

pub fn distribute_tags() {
    // Hashmap needed for mapping lang -> Vec
    let mut lang_to_vec = HashMap::<String, Vec<TagLine>>::new();

    let my_tags = load_file_checked(Path::new("tags"));
    let lines: Vec<&str> = my_tags.split("\n").collect();
    // tag datei pattern adresse kind
    let prepath = "lang_tags/";
    create_dir(prepath.clone());
    for line in lines {
        let elements: Vec<&str> = line.split("\t").collect();
        // Create files in output-folder
        if !elements.is_empty() {
            //    println!("{:?},",elements);
            if !elements[0].contains("TAG") {
                if elements[0] == "" {
                } else {
                    // Now fill in the languages
                    let i_s = elements[1].to_string().clone();
                    let lang_name = get_extension_from_filename(&i_s).expect("Can't get extension");
                    let lang_name = file_extension_to_lang(lang_name).expect("Can't get lang key");
                    // convert file_name to lang
                    File::create(format!("{}{}-tags", prepath, lang_name));
                    fs::remove_file(format!("{}{}-tags", prepath, lang_name)).expect("Problem");
                    File::create(format!("{}{}-tags", prepath, lang_name));

                    // Create now the folder
                    if elements.len() >= 4 {
                        // Now store in vector of the hashmap

                        // Now fill the Vector
                        let opt_vec = lang_to_vec.get_mut(&lang_name.to_string());
                        match opt_vec {
                            Some(there) => there.push(TagLine::new(
                                elements[0].to_string(),
                                elements[1].to_string(),
                                elements[2].to_string(),
                                elements[3].to_string(),
                            )),
                            None => {
                                // Create the vector
                                let mut my_vec = vec![];
                                my_vec.push(TagLine::new(
                                    elements[0].to_string(),
                                    elements[1].to_string(),
                                    elements[2].to_string(),
                                    elements[3].to_string(),
                                ));
                                lang_to_vec.insert(lang_name.to_string().clone(), my_vec);
                            }
                        }
                    }
                }
            }
        }
    }

    let pretty = get_pretty();
    // Write for every lang the serialized vector into the file
    for key in lang_to_vec.keys() {
        let mut tagline_data = String::new();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}{}-tags", prepath, key))
            .unwrap();

        tagline_data.push_str(
            to_string_pretty(
                &lang_to_vec.get(key).expect("Broken unwrap"),
                pretty.clone(),
            )
            .expect("Serialization failed")
            .as_mut_str(),
        );
        if let Err(e) = writeln!(file, "{}", tagline_data) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub fn load_tag(path: &Path) -> Vec<TagLine> {
    println!("Error is here");
    let file_string = load_file_checked(path);
    let my_vec: Vec<TagLine> = from_str(&file_string).unwrap();
    for i in &my_vec {
        println!("{:?}", i);
    }
    my_vec
}
