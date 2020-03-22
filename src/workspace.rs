use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::{Path};
use std::fs::{self, File};
use crate::utils::{get_pretty, load_good_file};
/// The workspace describes an environment,
/// which contains files and determines the location of commands
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Workspace {
    pub name: String,
    pub exclude_files: Vec<String>,
    pub commands: Vec<Runcommand>,
}

/// A Runcommand is a command, which will be run if it gets executed as a std::process::command
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Runcommand {
    pub name: String,
    pub has_button: bool,
    pub command: String,
    pub key_binding: Option<String>,
}

impl Runcommand {
    /// This function creates the command by adding the program and its arguments to the command object
    pub fn generate_command(&self) -> Option<Command> {
        let my_string = &self.command;
        let splitted: Vec<&str> = my_string.split(" ").collect();

        if splitted.is_empty() {
            None
        }
        // Now fill the commands
        else {
            let mut my_command = Command::new(splitted[0]);

            for (key, value) in splitted.iter().enumerate() {
                if key > 0 {
                    my_command.arg(value);
                }
            }
            Some(my_command)
        }
    }
    /// This function returns true if a Runcommand has {file} elements
    pub fn has_template(&self) -> bool {
        let my_string = &self.command;
        let splitted: Vec<&str> = my_string.split(" ").collect();

        if splitted.is_empty() {
            return false;
        }
        // Now fill the commands
        else {
            // let mut register = false;
            for value in splitted {
                if value == &"{file}".to_string() {
                    return true;
                }
            }
            return false;
        }
    }
    /// This function exchanges the file attribute with the correct file_string
    pub fn template_command(&mut self, file_tab: &str) {
        println!("Templating: ");
        let my_string = &mut self.command;
        let splitted: Vec<&str> = my_string.split(" ").collect();

        if splitted.is_empty() {
        } else {
            let mut copy_string = String::new();
            for value in splitted {
                let tab_clone = file_tab.clone();
                let mut val = value.clone();
                if value == &"{file}".to_string() {
                    // Substitute with current active tabs file path
                    val = tab_clone
                }
                copy_string.push_str(&format!("{} ", val));
            }
            // Remove ending white space as this is simpler than avoiding it
            copy_string = copy_string.trim_end().to_string();
            // Now overwrite the Command
            self.command = copy_string;
        }
    }
    /// This function generates a command object, which then gets executed.
    pub fn execute_command(the_cmd: Runcommand, cwd: &String) -> String {
        // Fetch from currently selected file aka Path from belonging Tab mapped to project root
        let mut my_cwd = the_cmd.generate_command().unwrap();
        let output = if cfg!(target_os = "windows") {
            my_cwd.current_dir(cwd).output().expect("failed to execute process")
        } else {
            my_cwd.current_dir(cwd).output().expect("failed to execute process")
        };
        let mut hello = output.stderr;
        if hello.is_empty() {
            hello = output.stdout;
        }
        String::from_utf8(hello).expect("Jey")
    }
}

/// Load a new workspace from a given folder, then returns the newly created workspace
/// Please provide the path without raide.ron as this expects an directory
pub fn load_workspace(path: &Path) -> Workspace {
    let build_command = Runcommand {
        name: "build".to_string(),
        has_button: true,
        command: "cargo build".to_string(),
        key_binding: None,
    };

    let run_command = Runcommand {
        name: "run".to_string(),
        has_button: true,
        command: "./file".to_string(),
        key_binding: None,
    };

    let format_command = Runcommand {
        name: "format".to_string(),
        has_button: true,
        command: "rustfmt {file}".to_string(),
        key_binding: None,
    };

    // Feature: start with welcome screen
    let my_ws : Workspace;
    let config_file = "raide.ron";

    println!("The path was: {}", path.as_os_str().to_str().expect("Neu"));
    let full_path_string = format!("{}/{}", path.to_str().expect("Should be working"), config_file);
    let full_path = Path::new(&full_path_string);
    let ws_there = full_path.exists();
    // If raide.ron is there use this else generate a new raide.ron
    if !ws_there {
        let file_name = path.file_name().expect("Couldn't extract dirname from it");
        // TODO check if file_name is dir

        my_ws = Workspace {
            name: file_name.to_str().expect("OS-String can't be converted to &str").to_string(),
            exclude_files: vec![],
            commands: vec![
                build_command.clone(),
                run_command.clone(),
                format_command.clone(),
            ],
        };
        let fp = full_path;
        File::create(&fp).expect(format!("Error at creating the file at path {:?}", fp).as_str());

        let pretty = get_pretty();
        // Stringify the workspace object into the file
        let mut file_string = String::new();
        file_string.push_str(
            ron::ser::to_string_pretty(&my_ws, pretty.clone())
                .expect("Serialization failed")
                .as_mut_str(),
        );

        fs::write(&fp, file_string).expect("Should write");
        my_ws
    }
    else {

        // Load the workspace from the given path (raide.ron)
        let ws_file = load_good_file(full_path);
        let ws_contents = ws_file;

        my_ws = ron::de::from_str(&ws_contents).expect("Writing file data into workspace object failed");
        my_ws
    }
}
