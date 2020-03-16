use serde::{Deserialize, Serialize};
use std::process::Command;

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
    pub fn execute_command(the_cmd: Runcommand) -> String {
        let mut my_cmd = the_cmd.generate_command().unwrap();
        let output = if cfg!(target_os = "windows") {
            my_cmd.output().expect("failed to execute process")
        } else {
            my_cmd.output().expect("failed to execute process")
        };
        let mut hello = output.stderr;
        if hello.is_empty() {
            hello = output.stdout;
        }
        String::from_utf8(hello).expect("Jey")
    }
}
