use gio::prelude::*;
use glib::{clone, TypedValue, Value};
use gtk::prelude::*;
use gtk::{
    Adjustment, Button, CellRendererText, ContainerExt, IconSize, Label, LabelExt, ListStore, Menu,
    MenuBar, MenuItem, Notebook, NotebookExt, Orientation, Paned, ReliefStyle, ScrolledWindow,ScrolledWindowExt,
    TextBuffer, TextIter, TextView, ToolButton, Toolbar, TreeIter, TreeSelection, TreeSelectionExt,
    TreeStore, TreeStoreExt, TreeView, TreeViewColumn, TreeViewExt, Widget,WidgetExt, EventBox, EventBoxExt,BoxExt
};
use raide::mapping::{get_by_left, get_by_right};
use raide::ui::UI;
use sourceview::{
    Buffer, Completion, CompletionExt, Language, LanguageManager, LanguageManagerBuilder,
    LanguageManagerExt, View, ViewExt,
};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::Error;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use ron;
use ron::ser::PrettyConfig;

/// The workspace config
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Workspace {
    pub name: String,
    pub exclude_files: Vec<String>,
    pub commands: Vec<Runcommand>
}

/// Commands
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Runcommand {
    pub name: String,
    pub has_button: bool,
    pub command: String,
    pub key_binding: Option<String>,
}


pub fn get_pretty() -> PrettyConfig {
    PrettyConfig {
        depth_limit: 2,
        new_line: "\n".to_string(),
        indentor: "    ".to_string(),
        separate_tuple_members: true,
        enumerate_arrays: true,
    }
}

pub fn execute_command (the_cmd: Runcommand) -> String {
  let mut my_cmd = generate_command(&the_cmd).unwrap();
  let output = if cfg!(target_os = "windows") {
    my_cmd.output()
            .expect("failed to execute process")
} else {
    my_cmd.output()
            .expect("failed to execute process")
};
let mut hello = output.stderr;
if hello.is_empty() {
    hello = output.stdout; 
}
String::from_utf8(hello).expect("Jey")
}

pub fn generate_command(run_cmd: &Runcommand) -> Option<Command> {
    let my_string = &run_cmd.command;
    let splitted : Vec<&str> = my_string.split(" ").collect();
    
    if splitted.is_empty() {
        None
    }
    // Now fill the commands
    else {
    let mut my_command = Command::new(splitted[0]);

    for (key,value) in splitted.iter().enumerate() {
        if key > 0 {
        my_command.arg(value);
        }
        
        
    }
    Some(my_command)
    }
}

pub fn has_template(run_cmd: &Runcommand) -> bool {
let my_string = &run_cmd.command;
    let splitted : Vec<&str> = my_string.split(" ").collect();
    
    if splitted.is_empty() {
        return false;
    }
    // Now fill the commands
    else {
   // let mut register = false;
    for (key, mut value) in splitted.iter().enumerate() {
        if value == &"{file}".to_string() {
            return true;
            
            }
    }
    return false;
}
}
// Interpret {file} attribute as given tab
// Returns if it should be registered later or now
pub fn template_command(run_cmd: &mut Runcommand, file_tab: &str) {
println!("Templating: ");
    let my_string = &run_cmd.command;
    let splitted : Vec<&str> = my_string.split(" ").collect();
    
    if splitted.is_empty() {
    }
    // Now fill the commands
    else {
        let len = splitted.len();
    let mut my_command = Command::new(splitted[0]);

    let mut copy_string = String::new();
    let mut register = false;
    for (key, mut value) in splitted.iter().enumerate() {
        let tab_clone = file_tab.clone();
        let mut val = value.clone();
        if value == &"{file}".to_string() {
            // Substitute with current active tab
            
            val = tab_clone;
          //  register = true;
        }
        copy_string.push_str(&format!("{} ", val));
    }
    copy_string = copy_string.trim_end().to_string();
    // Now overwrite the Command
    run_cmd.command = copy_string;
   // return register;
}
}
/*
pub fn build_file () -> String {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
} else {
    Command::new("rustc")
            .arg("file.rs")
            .output()
            .expect("failed to execute process")
};

let hello = output.stderr;
String::from_utf8(hello).expect("Jey")
}
*/

/*
pub fn format_file () -> String {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
} else {
    Command::new("rustfmt")
            .arg("file.rs")
            .output()
            .expect("failed to execute process")
};

let hello = output.stderr;
String::from_utf8(hello).expect("Jey")
}
*/
//use serde_derive::{Deserialize,Serialize};
pub fn load_workspace_file() {
    
}

/*
{
  "name": "Kate"
, "files": [ { "directory": "./", "filters": ["*.rs", "*.toml", "*.json", "*.ron" ], "recursive": 1 } ]
, "build": {
    "directory": "./"
  , "build": "cargo build"
  , "clean": "cargo run"
  , "install": "cargo run --release"
  }
, "index": true
}

*/


fn main() -> std::io::Result<()> {

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

let my_ws = Workspace {
    name: "raide".to_string(),
    exclude_files: vec![],
    commands: vec![build_command.clone(), run_command.clone(), format_command.clone()],
};
let f_string = my_ws.name.clone() + ".ron";

let ws_there = std::path::Path::new(&f_string).exists();
// Do nothing
if ws_there {

}
else {

//let f_string = format!("{}", (my_ws.name.clone() + ".ron"));
let fp = Path::new(&f_string);
let mut file = File::create(&fp);

let pretty = get_pretty();

    let mut file_string = String::new();
    file_string.push_str(
        ron::ser::to_string_pretty(&my_ws, pretty.clone())
            .expect("Serialization failed")
            .as_mut_str(),
    );

fs::write(&fp, file_string).expect("Should write");

}

// Create now the buttons

    // let mut file = File::open("file.rs")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    let uiapp = gtk::Application::new(
        Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    uiapp.connect_activate(move |app| {

    let mut ws_file = File::open(Path::new(&f_string)).unwrap();
    
    let mut ws_contents = String::new();
    ws_file.read_to_string(&mut ws_contents).unwrap();
    let open_content: Workspace = ron::de::from_str(&ws_contents).unwrap();
    
    
    let my_commands = open_content.commands;
    let tool_bar = Toolbar::new();
        let save_button = ToolButton::new::<Widget>(None, Some("Save"));
        tool_bar.insert(&save_button, 0);
    
    
    
    //planning_tree = ron::de::from_str(&my_message.content).unwrap();
        // At start no tab is open
        let mut manager = LanguageManager::new();
        // let mut buffer = Buffer::new_with_language(&manager.get_language("rust").unwrap());

        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);
        win.set_position(gtk::WindowPosition::Center);
        win.set_default_size(1024, 768);
        win.set_title("Raide");
        
        
       // let build_button = ToolButton::new::<Widget>(None, Some("Build"));
       // let run_button = ToolButton::new::<Widget>(None, Some("Run"));
      //  let format_button = ToolButton::new::<Widget>(None, Some("Format"));

        
       // tool_bar.insert(&build_button, 1);
       // tool_bar.insert(&run_button, 2);
       // tool_bar.insert(&format_button, 3);

        let gridbox = gtk::Box::new(Orientation::Vertical, 5);
        gridbox.add(&tool_bar);

        let console_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        console_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        console_window.set_min_content_height(500);
        // console_window.set_max_content_height(500);
        let outputview = View::new();
        console_window.add(&outputview);
        outputview.set_property("editable", &false);
        outputview.set_property("cursor-visible", &false);
        let outputbuffer = outputview.get_buffer().unwrap();

        
       
       // textbuffer.set_text(&contents);

       /*
        let outputbuffer_clone = outputbuffer.clone();
        let textbuffer_clone = textbuffer.clone();
        build_button.connect_clicked(move |_| {
            // Query selected tab
            save_it(&textbuffer_clone);
            let my_string = build_file();
            outputbuffer.set_text(my_string.as_str());
        });
        let textbuffer_clone = textbuffer.clone();
        run_button.connect_clicked(move |_| {
            // Query selected tab
            save_it(&textbuffer_clone);
            let my_string = run_file();
            outputbuffer_clone.set_text(my_string.as_str());
        });
        let textbuffer_clone = textbuffer.clone();

        save_button.connect_clicked(move |_| {
            save_it(&textbuffer_clone);
        });

        let textbuffer_clone = textbuffer.clone();

        format_button.connect_clicked(move |_| {
            format_it(&textbuffer_clone);
        });

        */

        // Store the shown filename and the full path
        let treestore = TreeStore::new(&[String::static_type(), String::static_type()]);
        let treeview = TreeView::new();

        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        treeview.append_column(&column);

        let path_column = TreeViewColumn::new();
        let path_cell = CellRendererText::new();
        path_column.pack_start(&path_cell, true);
        path_column.add_attribute(&path_cell, "text", 1);
        path_column.set_visible(false);
        treeview.append_column(&path_column);

        treeview.set_model(Some(&treestore));

        //let current_dir = env::current_dir().unwrap().path().into_os_string().into_string().unwrap();
        treestore.insert_with_values(
            None,
            None,
            &[0, 1],
            &[
                &"Project".to_value(),
                &"Placeholder for Overview".to_value(),
            ],
        );

        // TODO Move to user defined open directory
        let mut paths = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, Error>>()
            .unwrap();
        paths.sort();

        // files that should be ignored
        // let mut exclude_list = Vec::<String>::new();
        // exclude_list.push("target".to_owned());
        for path in paths {
            add_node(&treestore, &path, None);
        }

        let mut paned = Paned::new(Orientation::Vertical);
        let mut tree_selection = treeview.get_selection().clone();
        
        let mut notebook = Notebook::new();

        notebook.popup_enable();
        notebook.set_scrollable(true);
        notebook.set_show_border(true);


     //   let (my_model, my_iter) = tree_selection.get_selected().unwrap();
        

        let mut second_paned = Paned::new(Orientation::Horizontal);
        let mut tree_selection = tree_selection.clone();
        let project_pane = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        project_pane.add(&treeview);
        second_paned.add1(&project_pane);
        let mut tabs = Vec::<gtk::Box>::new();
        let my_ui = Rc::new(RefCell::new(UI {lang: manager.clone(), notebook: notebook.clone(), tabs: tabs.clone(), tree_selection: tree_selection.clone() }));
        
        
        {
            let my_ui = my_ui.clone();
        // Fetch the text buffer from the view
       //  let mut last_i : Runcommand = my_commands[0].clone();
         for mut i in my_commands {
         
   //     last_i = i.clone();
        let register = has_template(&i);
        let custom_button = ToolButton::new::<Widget>(None, Some(&i.name));
        
        /*
        if register {
         let strings: Vec<&str> = i.command.split(" ").collect();
            for my_val in strings {
                if my_val == "{file}" {
           
                }
            }
        
        }
        */
        
        
        // Check if command is valid
        let outputbuffer = outputbuffer.clone();
        let outputview = outputview.clone();
        let outlang = manager.get_language("rust").unwrap();
        let fake_buffer = Buffer::new_with_language(&outlang.clone());
        
       {
        let my_ui = my_ui.clone();
      //  let mut last_i = last_i.clone();
     //   let mut last_i = i.clone();
        custom_button.connect_clicked(move |_|  {
        
        // Now check, if a tab is selected 
        
       // let mut last_i = last_i.clone();
       if register {
         let notebook = &my_ui.deref().borrow_mut().notebook;
          //  let tabs = &my_ui.deref().borrow_mut().tabs;
            
            let content = notebook.get_focus_child();
            let mut to_file_string :Option<&str> = None;
            match content {
                None => {
                    println!("Tab is not selected");
                    // continue;
                }
                Some(value) => {
                   // println!("{:?}",notebook.get_tab_label_text(&my_ui.deref().borrow_mut().tabs[0]).unwrap());
                    let label_text = notebook.get_menu_label_text(&value);
                    
                    
                    let window = value.downcast::<ScrolledWindow>().unwrap();
                    let inside_view = window.get_child().unwrap().downcast::<View>().unwrap();
                    let inside_buffer = inside_view.get_buffer().unwrap();

                    let text_iter_start = inside_buffer.get_start_iter();
                    let text_iter_end = inside_buffer.get_end_iter();
                    let the_text = inside_buffer.get_text(&text_iter_start, &text_iter_end, true);
                    let plain_text = the_text.unwrap().to_string();
                    
                    let wrapped = label_text.unwrap();
                    println!("Before");
                    println!("Wrapper: {}", wrapped);
                    to_file_string = Some(&wrapped);
                   // Save
                    fs::write(Path::new(&wrapped), plain_text).expect("Should write");
                    let mut clone_i = i.clone();
                     template_command(&mut clone_i,&Path::new(&wrapped).to_str().unwrap());
                     println!("I: {:?}",clone_i);
                     //last_i = clone_i;
                     
                     let output = execute_command(clone_i.clone());
            if !output.is_empty() {
                
                fake_buffer.set_text(&output);
                outputview.set_buffer(Some(&fake_buffer));
                // outputbuffer.set_text(&output);
            }
            println!("Output is: {}", output);
                     }
                     
                     
                     }
                     
                     
                     
                     
                     }
          
            let output = execute_command(i.clone());
            if !output.is_empty() {
                
                fake_buffer.set_text(&output);
                outputview.set_buffer(Some(&fake_buffer));
                // outputbuffer.set_text(&output);
            }
            println!("Output is: {}", output);
        });
   
        }
        tool_bar.add(&custom_button);
    }
        
        save_button.connect_clicked(move |_| {
            let notebook = &my_ui.deref().borrow_mut().notebook;
          //  let tabs = &my_ui.deref().borrow_mut().tabs;
            
            let content = notebook.get_focus_child();
            match content {
                None => {
                    println!("Tab is not selected");
                }
                Some(value) => {
                   // println!("{:?}",notebook.get_tab_label_text(&my_ui.deref().borrow_mut().tabs[0]).unwrap());
                    let label_text = notebook.get_menu_label_text(&value);
                    let window = value.downcast::<ScrolledWindow>().unwrap();
                    let inside_view = window.get_child().unwrap().downcast::<View>().unwrap();
                    let inside_buffer = inside_view.get_buffer().unwrap();

                    let text_iter_start = inside_buffer.get_start_iter();
                    let text_iter_end = inside_buffer.get_end_iter();
                    let the_text = inside_buffer.get_text(&text_iter_start, &text_iter_end, true);
                    let plain_text = the_text.unwrap().to_string();
                    
                    let wrapped = label_text.unwrap();
                    println!("Wrapper: {}", wrapped);
                   // let childs = wrapped.downcast::<gtk::Box>();
                  //  let mys = childs.unwrap().get_center_widget();
                  //  println!("This: {:?}", mys);
                 //   let label_text = notebook.get_menu_label_text(&window);
                  //  println!("Label is: {:?}", label);
                    // Write to
                    fs::write(Path::new(&wrapped), plain_text).expect("Should write");
                }
            }
            
           // window.
            // save_it(&textbuffer_clone);
        });
          }
    let my_ui = my_ui.clone();
        // Insert
        tree_selection.connect_changed(clone!(@weak tree_selection => move |_| {
            // That works
            let notebook = &my_ui.deref().borrow_mut().notebook;
            let (my_model, my_iter) = tree_selection.get_selected().unwrap();
            let path_string = my_model.get_value(&my_iter, 1).get::<String>().unwrap().unwrap();
            let last_string = my_model.get_value(&my_iter, 0).get::<String>().unwrap().unwrap();
            println!("{}", path_string);
           
            let my_path = Path::new(path_string.as_str());
            if my_path.exists() {
                let mut my_file = File::open(my_path).unwrap();
                let mut contents = String::new();
                my_file.read_to_string(&mut contents).unwrap();
                
                let mut extension = get_extension_from_filename(path_string.as_str());
                println!("{:?}", extension);
    
                let mut scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
                scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
                scrolled_window.set_min_content_height(500);
                //let mut my_buffer = Buffer::new_with_language(&manager.get_language("md").unwrap());
    
                //my_buffer.set_text(contents.as_str());
                
                let mut my_view = View::new();
                my_view.set_highlight_current_line(true);
                my_view.set_auto_indent(true);
                my_view.set_indent_on_tab(true);
                my_view.set_insert_spaces_instead_of_tabs(true);
                my_view.set_show_line_marks(true);
                my_view.set_show_line_numbers(true);

                let text_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
                text_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
                text_window.set_min_content_height(500);
        
                
    
                match extension {
                    None => {
                        // Set to markdown for displaying text
                        extension = Some("md");
                        let mut my_buffer = Buffer::new_with_language(&manager.get_language(extension.unwrap()).unwrap());
                        my_buffer.set_text(contents.as_str());    
                        my_view.set_buffer(Some(&my_buffer));
                        scrolled_window.add(&my_view);
                        let mut tabs = tabs.clone();
                        create_tab(&notebook, &mut tabs, last_string.as_str(),path_string.as_str(), scrolled_window.upcast());
    
                    }
                    Some(ext) => {
                        let lookup = get_by_left(ext);
                        match lookup {
                            // Non programming language extension
                            None => {
                                extension = Some("md");
                                let mut my_buffer = Buffer::new_with_language(&manager.get_language(extension.unwrap()).unwrap());
                                my_buffer.set_text(contents.as_str());
                                // TODO my_buffer.set_text() from file content if file not loaded    
                                my_view.set_buffer(Some(&my_buffer));
                                scrolled_window.add(&my_view);
                                let mut tabs = tabs.clone();
                                create_tab(&notebook, &mut tabs, last_string.as_str(),path_string.as_str(), scrolled_window.upcast());
                            }
                            // matched lang string
                            Some(lang) => {
                                
                                let mut my_buffer = Buffer::new_with_language(&manager.get_language(lang).unwrap());
                                my_buffer.set_text(contents.as_str());    
                                my_view.set_buffer(Some(&my_buffer));
                                scrolled_window.add(&my_view);
                                let mut tabs = tabs.clone();
                                create_tab(&notebook, &mut tabs, last_string.as_str(), path_string.as_str(), scrolled_window.upcast());
                            }
                        }
                    }
                }    
            }
            
    
            // println!("File exists? {}", Path::new(path_string.as_str()).exists());
        }));

        paned.add1(&notebook);
        paned.add2(&console_window);
        second_paned.add2(&paned);

        gridbox.add(&second_paned);

        win.add(&gridbox);

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());

    Ok(())
}

pub fn load_file(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

// https://stackoverflow.com/questions/45291832/extracting-a-file-extension-from-a-given-path-in-rust-idiomatically
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

// https://github.com/oakes/SolidOak/blob/master/src/ui.rs
pub fn path_sorter(a: &PathBuf, b: &PathBuf) -> Ordering {
    if let Some(a_os_str) = a.deref().file_name() {
        if let Some(b_os_str) = b.deref().file_name() {
            return a_os_str.cmp(&b_os_str);
        }
    }
    Ordering::Equal
}

pub fn add_node(tree_store: &TreeStore, node: &Path, parent: Option<&TreeIter>) {
    if let Some(full_path_str) = node.to_str() {
        if let Some(leaf_os_str) = node.file_name() {
            if let Some(leaf_str) = leaf_os_str.to_str() {
                if !(leaf_str.starts_with(".") || leaf_str.starts_with("target")) {
                    let iter = tree_store.append(parent);
                    tree_store.set(
                        &iter,
                        &[0, 1],
                        &[&String::from(leaf_str), &String::from(full_path_str)],
                    );

                    // Fetch metadata of node
                    if metadata(node).map(|m| m.is_dir()).unwrap_or(false) {
                        match fs::read_dir(node) {
                            Ok(child_iter) => {
                                let mut child_vec = Vec::new();
                                for child in child_iter {
                                    if let Ok(dir_entry) = child {
                                        child_vec.push(dir_entry.path());
                                    }
                                }
                                child_vec.sort_by(path_sorter);
                                for child in child_vec.iter() {
                                    add_node(tree_store, child.deref(), Some(&iter));
                                }
                            }
                            Err(e) => println!("Error updating tree: {}", e),
                        }
                    }
                }
            }
        }
    }
}

// Dependent on current tab
pub fn save_it(textbuffer: &TextBuffer) {
    let text_iter_start = textbuffer.get_start_iter();
    let text_iter_end = textbuffer.get_end_iter();
    let the_text = textbuffer.get_text(&text_iter_start, &text_iter_end, true);
    let other_text = the_text.expect("Hey").to_string();
    fs::write("file.rs", other_text).expect("Should write");
}

// Dependent on current tab
pub fn format_it(textbuffer: &TextBuffer) -> std::io::Result<()> {
    save_it(&textbuffer);
    format_file();

    // TODO reload file from selecters full file name
    let mut file = File::open("file.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    textbuffer.set_text(&contents);
    Ok(())
}

// create tab based on ui elements
pub fn create_tab(
    notebook: &Notebook,
    tabs: &mut Vec<gtk::Box>,
    title: &str,
    my_path: &str,
    widget: Widget,
) -> u32 {
    let close_image = gtk::Image::new_from_icon_name(Some("window-close"), IconSize::Button);
    let button = gtk::Button::new();
    let label = gtk::Label::new(Some(title));
  //  let info = gtk::Label::new(Some(my_path));
    let event_box = EventBox::new();
    // event_box.set_above_child;
    let tab = gtk::Box::new(Orientation::Horizontal, 0);

    button.set_relief(ReliefStyle::None);
    button.set_focus_on_click(false);
    button.add(&close_image);
    
    

    tab.pack_start(&label, false, false, 0);
    tab.pack_start(&button, false, false, 0);
   // tab.pack_start(&info, false, false, 0);
    tab.show_all();

    let index = notebook.append_page(&widget, Some(&tab));
    notebook.set_menu_label_text(&widget,my_path);

    button.connect_clicked(clone!(@weak notebook => move |_| {
        let index = notebook
            .page_num(&widget)
            .expect("Couldn't get page_num from notebook");
        notebook.remove_page(Some(index));
    }));
/*
    label.connect_clicked(clone!(@weak notebook => move |_| {
        let index = notebook
            .page_num(&widget)
            .expect("Couldn't get page_num from notebook");
        notebook.remove_page(Some(index));
    }));
*/
    tabs.push(tab);
    notebook.show_all();
  //  info.hide();
  //  notebook.set_tab_label_text("");
  //  notebook.set_focus_child(Some(&widget.clone()));

    index
}

// Based on ui elements, removing method head
pub fn click_listener(
    tree_selection: &TreeSelection,
    manager: &LanguageManager,
    notebook: Notebook,
    tabs: &mut Vec<gtk::Box>,
) {
    let mut tabs = tabs.clone();
    tree_selection.connect_changed(clone!(@weak tree_selection, @weak manager  => move |_| {
        let (my_model, my_iter) = tree_selection.get_selected().unwrap();
        let path_string = my_model.get_value(&my_iter, 1).get::<String>().unwrap().unwrap();
        let last_string = my_model.get_value(&my_iter, 0).get::<String>().unwrap().unwrap();
        println!("{}", path_string);
       
        let my_path = Path::new(path_string.as_str());
        if my_path.exists() {

            let mut contents = load_file(&my_path);
            let mut my_file = File::open(my_path).unwrap();
            let mut contents = String::new();
            my_file.read_to_string(&mut contents).unwrap();
            
            let mut extension = get_extension_from_filename(path_string.as_str());
            println!("{:?}", extension);

            let mut scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
            //let mut my_buffer = Buffer::new_with_language(&manager.get_language("md").unwrap());

            //my_buffer.set_text(contents.as_str());
            
            let mut my_view = View::new();
            

            match extension {
                None => {
                    // Set to markdown for displaying text
                    extension = Some("md");
                    let mut my_buffer = Buffer::new_with_language(&manager.get_language(extension.unwrap()).unwrap());
                    
                    // set text from file
                    my_buffer.set_text(&contents);
                    my_view.set_buffer(Some(&my_buffer));
                    scrolled_window.add(&my_view);
                    let mut tabs = tabs.clone();
                    create_tab(&notebook, &mut tabs, last_string.as_str(), path_string.as_str(), scrolled_window.upcast());

                }
                Some(ext) => {
                    let lookup = get_by_left(ext);
                    match lookup {
                        // Non programming language extension
                        None => {
                            extension = Some("md");
                            let mut my_buffer = Buffer::new_with_language(&manager.get_language(extension.unwrap()).unwrap());    
                            my_view.set_buffer(Some(&my_buffer));
                            scrolled_window.add(&my_view);
                            let mut tabs = tabs.clone();
                            create_tab(&notebook, &mut tabs, last_string.as_str(), path_string.as_str(), scrolled_window.upcast());
                        }
                        // matched lang string
                        Some(lang) => {
                            
                            let mut my_buffer = Buffer::new_with_language(&manager.get_language(lang).unwrap());    
                            my_view.set_buffer(Some(&my_buffer));
                            scrolled_window.add(&my_view);
                            let mut tabs = tabs.clone();
                            create_tab(&notebook, &mut tabs, last_string.as_str(), path_string.as_str(), scrolled_window.upcast());
                        }
                    }
                }
            }    
        }
        

        // println!("File exists? {}", Path::new(path_string.as_str()).exists());
    }));
}

// Build file depending on selected tab -> project relation with custom position
pub fn build_file() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("rustc")
            .arg("file.rs")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stderr;
    String::from_utf8(hello).expect("Jey")
}

// Format file depending on selected tab -> file relation
pub fn format_file() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("rustfmt")
            .arg("file.rs")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stderr;
    String::from_utf8(hello).expect("Jey")
}

// Run project depending on selected tab -> file relation
pub fn run_file() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("./file")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    String::from_utf8(hello).expect("Jey")
}
