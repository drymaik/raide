use raide::mapping::{get_by_left,get_by_right};
use gio::prelude::*;
use glib::clone;
use glib::{TypedValue, Value};
use gtk::prelude::*;
use gtk::{
    Adjustment, Box, Button, CellRendererText, ListStore, Menu, MenuBar, MenuItem, Orientation,
    Paned, ScrolledWindow, TextBuffer, TextIter, TextView, ToolButton, Toolbar, TreeIter,
    TreeSelection, TreeSelectionExt, TreeStore, TreeStoreExt, TreeView, TreeViewColumn,
    TreeViewExt, Widget,Notebook,NotebookExt,Label,LabelExt,
};
use gtk::{IconSize, ReliefStyle};
use sourceview::{
    Buffer, Completion, CompletionExt, Language, LanguageManager, LanguageManagerBuilder,
    LanguageManagerExt, View, ViewExt,
};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::ffi::OsStr;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::Error;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

/*
macro_rules! myclone {
  (@param _) => ( _ );
  (@param $x:ident) => ( $x );
  ($($n:ident),+ => move || $body:expr) => (
      {
          $( let $n = $n.clone(); )+
          move || $body
      }
  );
  ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
      {
          $( let $n = $n.clone(); )+
          move |$(clone!(@param $p),)+| $body
      }
  );
}
*/

pub struct TabPanel {
    notebook: Notebook,
    tabs: Vec<gtk::Box>,
}

impl TabPanel {
    fn new() -> TabPanel {
        TabPanel {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new(),
        }
    }

    fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let close_image = gtk::Image::new_from_icon_name(Some("window-close"), IconSize::Button);
        let button = gtk::Button::new();
        let label = gtk::Label::new(Some(title));
        let tab = gtk::Box::new(Orientation::Horizontal, 0);

        button.set_relief(ReliefStyle::None);
        button.set_focus_on_click(false);
        button.add(&close_image);

        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();

        let index = self.notebook.append_page(&widget, Some(&tab));


        button.connect_clicked(clone!(@weak self.notebook as notebook => move |_| {
            let index = notebook
                .page_num(&widget)
                .expect("Couldn't get page_num from notebook");
            notebook.remove_page(Some(index));
        }));

        self.tabs.push(tab);

        index
    }
}

pub struct FileData {
    pub display_name: String,
    pub path: String,
}
pub fn save(other_text: String) {
    fs::write("file.rs", other_text).expect("Should write");
}

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
fn main() -> std::io::Result<()> {
    let mut file = File::open("file.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let uiapp = gtk::Application::new(
        Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    uiapp.connect_activate(move |app| {
        let mut manager = LanguageManager::new();
        let mut buffer = Buffer::new_with_language(&manager.get_language("rust").unwrap());

        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);
        win.set_position(gtk::WindowPosition::Center);
        win.set_default_size(1024, 768);
        win.set_title("Raide");
        
        let tool_bar = Toolbar::new();
        let save_button = ToolButton::new::<Widget>(None, Some("Save"));
        let build_button = ToolButton::new::<Widget>(None, Some("Build"));
        let run_button = ToolButton::new::<Widget>(None, Some("Run"));
        let format_button = ToolButton::new::<Widget>(None, Some("Format"));

        tool_bar.insert(&save_button, 0);
        tool_bar.insert(&build_button, 1);
        tool_bar.insert(&run_button, 2);
        tool_bar.insert(&format_button, 3);

        let gridbox = Box::new(Orientation::Vertical, 5);
        gridbox.add(&tool_bar);

        let text_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        text_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        text_window.set_min_content_height(500);
        //  text_window.set_max_content_height(500);
        let textview = View::new();
        textview.set_highlight_current_line(true);
        textview.set_auto_indent(true);
        textview.set_indent_on_tab(true);
        textview.set_insert_spaces_instead_of_tabs(true);
        textview.set_show_line_marks(true);
        textview.set_show_line_numbers(true);
        //textview.set_show_right_margin(true);
        // textview.set_smart_backspace(true);
        // let my_obj = textview.get_completion();
        // println!("Completion: {:?}", my_obj);

        text_window.add(&textview);
        textview.set_buffer(Some(&buffer));
        let textbuffer = textview.get_buffer().unwrap();

        let console_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        console_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        console_window.set_min_content_height(500);
        // console_window.set_max_content_height(500);
        let outputview = TextView::new();
        console_window.add(&outputview);
        outputview.set_property("editable", &false);
        outputview.set_property("cursor-visible", &false);
        let outputbuffer = outputview.get_buffer().unwrap();

        textbuffer.set_text(&contents);

        let outputbuffer_clone = outputbuffer.clone();
        let textbuffer_clone = textbuffer.clone();
        build_button.connect_clicked(move |_| {
            // Hier
            save_it(&textbuffer_clone);
            let my_string = build_file();
            outputbuffer.set_text(my_string.as_str());
        });
        let textbuffer_clone = textbuffer.clone();
        run_button.connect_clicked(move |_| {
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

        let mut paned = Paned::new(Orientation::Vertical);
        // Now adding a notebook and inside the tabs with text_windows
        //paned.add1(&text_window);
        let mut notebook = TabPanel::new();

       // let mut label = Label::new(Some(&"Text"));
      //  let mut other_label = Label::new(Some(&"Other"));
        let mut other_scrolled = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let mut my_test = Label::new(Some(&"Hello-Content"));
        other_scrolled.add(&my_test);
       // notebook.append_page(&text_window,Some(&label));
       // notebook.append_page(&text_window,Some(&other_label));
        
        notebook.create_tab("First", text_window.upcast());
        notebook.create_tab("Second", other_scrolled.upcast());

        notebook.notebook.popup_enable();
        notebook.notebook.set_scrollable(true);
        notebook.notebook.set_show_border(true);

        //notebook.connect_switch_page(move || {

      //  });
        paned.add1(&notebook.notebook);
        paned.add2(&console_window);

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
        let textbuffer_clone = textbuffer.clone();
        let mut tree_selection = treeview.get_selection();
        tree_selection.connect_changed(move |tree_selection| {
            let (my_model, my_iter) = tree_selection.get_selected().unwrap();
            let path_string = my_model.get_value(&my_iter, 1).get::<String>().unwrap().unwrap();
            println!("{}", path_string);
            // Exchange view
            let my_path = Path::new(path_string.as_str());
            if my_path.exists() {
                let mut my_file = File::open(my_path).unwrap();
                let mut contents = String::new();
                my_file.read_to_string(&mut contents).unwrap();
                // Reset language based on file extension
                let mut extension = get_extension_from_filename(path_string.as_str());
                println!("{:?}", extension);
                match extension {
                    None => {
                        // Set to markdown for displaying text
                        extension = Some("md");
                    }
                    Some(ext) => {
                        let lookup = get_by_left(ext);
                        match lookup {
                            // Non programming language extension
                            None => {
                                extension = Some("md");
                            }
                            // matched lang string
                            Some(lang) => {
                                
                                /*
                                // Only Buffer instead of TextBuffer has the language setting method
                                let buffer_hack = Buffer::new_with_language(&manager.get_language(lang).unwrap());
                                let text_iter_start = textbuffer_clone.get_start_iter();
                                let text_iter_end = textbuffer_clone.get_end_iter();
                                let the_text = textbuffer_clone.get_text(&text_iter_start, &text_iter_end, true);
                                buffer_hack.set_text(the_text.unwrap().as_str());
                                // Now replacing the views buffer
                                textview.set_buffer(Some(&buffer_hack));
                               
                               */
                            }
                        }
                    }
                }
                
                // Needs extension manager
                textbuffer_clone.set_text(&contents);

                //println!("Contents: {}", contents);    
            }
            

            // println!("File exists? {}", Path::new(path_string.as_str()).exists());
        });

        let mut second_paned = Paned::new(Orientation::Horizontal);

        let project_pane = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        project_pane.add(&treeview);
        second_paned.add1(&project_pane);
        second_paned.add2(&paned);

        gridbox.add(&second_paned);

        win.add(&gridbox);

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());

    Ok(())
}

// https://stackoverflow.com/questions/45291832/extracting-a-file-extension-from-a-given-path-in-rust-idiomatically
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
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

pub fn save_it(textbuffer: &TextBuffer) {
    let text_iter_start = textbuffer.get_start_iter();
    let text_iter_end = textbuffer.get_end_iter();
    let the_text = textbuffer.get_text(&text_iter_start, &text_iter_end, true);
    let other_text = the_text.expect("Hey").to_string();
    fs::write("file.rs", other_text).expect("Should write");
}

pub fn format_it(textbuffer: &TextBuffer) -> std::io::Result<()> {
    save_it(&textbuffer);
    format_file();

    // reload
    let mut file = File::open("file.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    textbuffer.set_text(&contents);
    Ok(())
}
