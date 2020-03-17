use std::convert::TryInto;
use gio::{ApplicationFlags};
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{
    BoxExt, CellRendererText, ContainerExt, IconSize, Notebook, NotebookExt, Orientation, Paned,
    ReliefStyle, ScrolledWindow, ScrolledWindowExt, TextBuffer, ToolButton, Toolbar, TreeIter,
    TreeSelectionExt, TreeStore, TreeStoreExt, TreeView, TreeViewColumn, TreeViewExt, Widget,
    WidgetExt,Image, GtkWindowExt,Label,ResponseType, ApplicationWindow, FileChooserDialog,SelectionMode,
    FileChooserAction,DialogExt,Button,
};
use raide::ctags_api::read;
use raide::mapping::get_by_left;
use raide::ui::UI;
use raide::utils::{load_invalid_file};
use raide::workspace::{Runcommand, load_workspace};
use sourceview::{Buffer, LanguageManager, LanguageManagerExt, View, ViewExt};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::{metadata};
use std::io::{Error};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

pub fn open_project(path: &Path, my_store: &mut TreeStore) {
    // This needs loading of the workspace
    // than adding tabs at the left side

    let my_ws = load_workspace(path);
    let my_parent = my_store.insert_with_values(
        None,
        None,
        &[0, 1],
        &[
            &my_ws.name.to_value(),
            &"Project settings".to_value(),
        ],
    );

    // TODO Move to user defined open directory
    let mut paths = fs::read_dir(path)
        .expect("Can't read workspace path given by user")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()
        .expect("Cannot collect workspace data into vector");
    paths.sort();

    // TODO add files that should be ignored
    // let mut exclude_list = Vec::<String>::new();
    // exclude_list.push("target".to_owned());


    for path in paths {
        add_node(&my_store, &path, Some(&my_parent));
    }

    // Now adding the tabs

}

fn main() -> std::io::Result<()> {
    // Testing ctags api is in progress
    // read();



    // Load image

    let mut my_dir = env::args()
            .nth(1)
            .unwrap_or_else(|| ".".to_string());

            // Converts . to the long version of current working directory
            if my_dir == ".".to_string() {
                my_dir = std::env::current_dir().expect("Can't cast to current working directory").to_str().expect("Can't cast back to str").to_string();
            }

    let uiapp = gtk::Application::new(
        Some("org.gtkrsnotes.demo"),
        //ApplicationFlags::HANDLES_COMMAND_LINE,
        ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    // GTK closure that is home to all Gtk-Elements and Widgets
    uiapp.connect_activate(move |app| {

    let project_dir = my_dir.clone();
    let raide_dir = my_dir.clone();

    // Store the shown filename and the full path in the treestore
    let treestore = TreeStore::new(&[String::static_type(), String::static_type()]);
    let treeview = TreeView::new();

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    // For displaying text
    column.add_attribute(&cell, "text", 0);
    treeview.append_column(&column);

    let path_column = TreeViewColumn::new();
    let path_cell = CellRendererText::new();
    path_column.pack_start(&path_cell, true);
    path_column.add_attribute(&path_cell, "text", 1);
    // Hidden column as hack to know the corresponding file
    path_column.set_visible(false);
    treeview.append_column(&path_column);
    treeview.set_model(Some(&treestore));

    let _my_welcome = treestore.insert_with_values(
        None,
        None,
        &[0, 1],
        &[
            //&open_content.name.to_value(),
            &"Welcome",
            &"Welcome".to_value(),
        ],
    );

    let open_content = load_workspace(Path::new(&raide_dir));

    let my_commands = open_content.commands;
    // Toolbar contains at least a save button for a file
    let tool_bar = Toolbar::new();
        let save_button = ToolButton::new::<Widget>(None, Some("Save"));
        tool_bar.insert(&save_button, 0);

        let manager = LanguageManager::new();

        // We create the main window
        let win = gtk::ApplicationWindow::new(app);
        win.set_icon_from_file("pictures/small.png").expect("Can't open icon file small.png");
        win.set_position(gtk::WindowPosition::Center);
        win.set_default_size(1024, 768);
        win.set_title("Raide");

        // Storing the editor in this main widget
        let gridbox = gtk::Box::new(Orientation::Vertical, 5);
        gridbox.add(&tool_bar);

        // Console with read only source view
        let console_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        console_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        console_window.set_min_content_height(500);

        let outputview = View::new();
        console_window.add(&outputview);
        outputview.set_property("editable", &false).expect("property editable couldn't be set to false");
        outputview.set_property("cursor-visible", &false).expect("property cursor-visible couldn't be set to false");



        let my_parent = treestore.insert_with_values(
            None,
            None,
            &[0, 1],
            &[
                &open_content.name.to_value(),
                &"Project settings".to_value(),
            ],
        );

        // TODO Move to user defined open directory
        let mut paths = fs::read_dir(project_dir)
            .expect("Can't read workspace path given by user")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, Error>>()
            .expect("Cannot collect workspace data into vector");
        paths.sort();

        // TODO add files that should be ignored
        // let mut exclude_list = Vec::<String>::new();
        // exclude_list.push("target".to_owned());


        for path in paths {
            add_node(&treestore, &path, Some(&my_parent));
        }


        let paned = Paned::new(Orientation::Vertical);
        let tree_selection = treeview.get_selection().clone();

        let notebook = Notebook::new();

        // Notebook as container for editor tabs
        notebook.popup_enable();
        notebook.set_scrollable(true);
        notebook.set_show_border(true);


        // Paned is used to shift the different views like left sidebar and Notebook or outputview
        let second_paned = Paned::new(Orientation::Horizontal);
        let tree_selection = tree_selection.clone();
        let project_pane = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        // Left treeview is now complete
        project_pane.add(&treeview);
        second_paned.add1(&project_pane);
        let tabs = Vec::<gtk::Box>::new();


        let my_ui = Rc::new(RefCell::new(UI {lang: manager.clone(), notebook: notebook.clone(), tabs: tabs.clone(), tree_selection: tree_selection.clone() }));

        // Scoping hack to use my_ui multiple times for consuming closures
        {
         let my_ui = my_ui.clone();
         for i in my_commands {

        // Determine when registering should happen of Button function
        let register = i.has_template();
        let custom_button = ToolButton::new::<Widget>(None, Some(&i.name));

        // TODO outputview should have the same highlighting language as the open tab
        // Check if command is valid
        let outputview = outputview.clone();
        let outlang = manager.get_language("rust").expect("Language Rust is not available in Language Manager. Have you installed Gtk3-dev and gtksourceview3?");
        let fake_buffer = Buffer::new_with_language(&outlang.clone());

       {
        let my_ui = my_ui.clone();
        // Real magic happens here
        custom_button.connect_clicked(move |_|  {

      // Checks if active tab should be considered while executing command
       if register {
         let notebook = &my_ui.deref().borrow_mut().notebook;
            let content = notebook.get_focus_child();
            match content {
                None => {
                    println!("Tab is not selected");
                }
                Some(value) => {
                    // Get the path stored inside the label
                    let label_text = notebook.get_menu_label_text(&value);
                    let window = value.downcast::<ScrolledWindow>().expect("Can't cast window to a scrolled window");
                    let inside_view = window.get_child().expect("The child of the window is empty").downcast::<View>().expect("Can't cast Widget as view");
                    let inside_buffer = inside_view.get_buffer().expect("Buffer is not accessible inside view");

                    let text_iter_start = inside_buffer.get_start_iter();
                    let text_iter_end = inside_buffer.get_end_iter();
                    let the_text = inside_buffer.get_text(&text_iter_start, &text_iter_end, true);
                    let plain_text = the_text.expect("Plain text from buffer doesn't exist").to_string();

                    let wrapped = label_text.expect("Text from label doesn't exist");
                    println!("Before");
                    println!("Wrapper: {}", wrapped);
                   // Save to the file using the path
                    fs::write(Path::new(&wrapped), plain_text).expect("Should write");
                    let mut clone_i = i.clone();
                     clone_i.template_command(&Path::new(&wrapped).to_str().expect("The path can't be cast to a string"));
                     println!("I: {:?}",clone_i);

                     let output = Runcommand::execute_command(clone_i.clone());
            if !output.is_empty() {
                // Something to display
                fake_buffer.set_text(&output);
                outputview.set_buffer(Some(&fake_buffer));
            }
            println!("Output is: {}", output);
        }
             }
         }

            // Displaying without registering
            let output = Runcommand::execute_command(i.clone());
            if !output.is_empty() {

                fake_buffer.set_text(&output);
                outputview.set_buffer(Some(&fake_buffer));
            }
            println!("Output is: {}", output);
        });

        }
        tool_bar.add(&custom_button);
    }

        save_button.connect_clicked(move |_| {
            let notebook = &my_ui.deref().borrow_mut().notebook;

            let content = notebook.get_focus_child();
            match content {
                None => {
                    println!("Tab is not selected");
                }
                Some(value) => {
                    // Use path information from label
                    let label_text = notebook.get_menu_label_text(&value);
                    let window = value.downcast::<ScrolledWindow>().expect("Can't cast the Widget to a ScrolledWindow");
                    let inside_view = window.get_child().expect("Can't get a child out of the window").downcast::<View>().expect("Can't cast the widget to a view");
                    let inside_buffer = inside_view.get_buffer().expect("Can't access the buffer of the inside view");

                    let text_iter_start = inside_buffer.get_start_iter();
                    let text_iter_end = inside_buffer.get_end_iter();
                    let the_text = inside_buffer.get_text(&text_iter_start, &text_iter_end, true);
                    let plain_text = the_text.expect("Can't access the text of the inside buffer").to_string();

                    let wrapped = label_text.expect("Can't access the text of the label");
                    println!("Wrapper: {}", wrapped);
                    fs::write(Path::new(&wrapped), plain_text).expect("Should write");
                }
            }
        });
          }
    let my_ui = my_ui.clone();
    tree_selection.set_mode(SelectionMode::Browse);
        tree_selection.connect_changed(clone!(@weak tree_selection => move |_| {

            let notebook = &my_ui.deref().borrow_mut().notebook;
            let (my_model, my_iter) = tree_selection.get_selected().expect("Cannot access the selected element");
            let path_string = my_model.get_value(&my_iter, 1).get::<String>().expect("First unbox of path string failed").expect("Second unbox of path string failed");
            let last_string = my_model.get_value(&my_iter, 0).get::<String>().expect("First unbox of last string failed").expect("Second unbox of last string failed");
            println!("{}", path_string);

            let my_path = Path::new(path_string.as_str());
            if my_path.exists() {
                // Check again, a directory should not open a tag.
                if !metadata(my_path).map(|m| m.is_dir()).unwrap_or(false) {
                    // It is not a folder, continue normally
                    let my_file = load_invalid_file(my_path);
                    let contents = my_file;
                    let mut extension = get_extension_from_filename(path_string.as_str());
                    let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
                    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
                    scrolled_window.set_min_content_height(500);

                    // Generate a new view for the new tab
                    let my_view = View::new();
                    my_view.set_highlight_current_line(true);
                    my_view.set_auto_indent(true);
                    my_view.set_indent_on_tab(true);
                    my_view.set_insert_spaces_instead_of_tabs(true);
                    my_view.set_show_line_marks(true);
                    my_view.set_show_line_numbers(true);

                    let text_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
                    text_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
                    text_window.set_min_content_height(500);

                    // Set the buffers display extension .rs means Rust for example
                    match extension {
                        None => {
                            // Set to markdown for displaying text
                            extension = Some("markdown");
                            let my_buffer = Buffer::new_with_language(&manager.get_language(extension.expect("Failed retrieving md highlighting from extension I")).expect("Can't call get_language from extension I"));
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
                                    extension = Some("markdown");
                                    let my_buffer = Buffer::new_with_language(&manager.get_language(extension.expect("Failed retrieving md highlighting from extension 2")).expect("Can't call get_language from extension 2"));
                                    my_buffer.set_text(contents.as_str());
                                    my_view.set_buffer(Some(&my_buffer));
                                    scrolled_window.add(&my_view);
                                    let mut tabs = tabs.clone();
                                    create_tab(&notebook, &mut tabs, last_string.as_str(),path_string.as_str(), scrolled_window.upcast());
                                }
                                // matched language string
                                Some(lang) => {
                                    let my_buffer = Buffer::new_with_language(&manager.get_language(lang).expect("Existing language can't be used to instantiate buffer"));
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
                // It is a dir

            }
            // It must be the welcome screen
            else if my_path.to_str().expect("Can't convert path to str") == "Welcome" {
                let my_vbox = gtk::Box::new(Orientation::Vertical, 5);
                let my_label = Label::new(Some(&"Raide"));
                let my_image = Image::new_from_file("pictures/normal.png");
                let scroller = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

                let my_button = Button::new_with_label("Open");

                {
                let mut treestore = treestore.clone();
                my_button.connect_clicked(move |_| {
                    let mut treestore = treestore.clone();
                    let my_file_dialog = FileChooserDialog::with_buttons::<ApplicationWindow>(Some(&"Open Folder"), None, FileChooserAction::SelectFolder, &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)]);
                    my_file_dialog.set_select_multiple(true);
                    my_file_dialog.run();
                    let files = my_file_dialog.get_filenames();
                    println!("{:?}", files);
                    my_file_dialog.destroy();
                    for element in files {
                        open_project(&element, &mut treestore);
                        println!("Open project at {:?}", element);
                    }
                });

            }

                my_vbox.add(&my_label);
                my_vbox.add(&my_image);
                my_vbox.add(&my_button);

                scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
                scroller.set_min_content_height(500);
                scroller.add(&my_vbox);
                let mut tabs = tabs.clone();
                create_tab(&notebook, &mut tabs, "Welcome","./Welcome", scroller.upcast());
            }
        }));

        paned.add1(&notebook);
        paned.add2(&console_window);
        second_paned.add2(&paned);

        gridbox.add(&second_paned);

        win.add(&gridbox);

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&vec![]);
    // uiapp.run(&env::args().collect::<Vec<_>>());

    Ok(())
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

// Recursive function to fill a TreeStore mode
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

// create tab based on ui elements
pub fn create_tab(
    notebook: &Notebook,
    tabs: &mut Vec<gtk::Box>,
    title: &str,
    my_path: &str,
    widget: Widget,
) -> u32 {
    let children = notebook.get_children();
    for (key,value) in children.iter().enumerate() {
        let label_text = notebook.get_menu_label_text(value);
        let wrapped = label_text.expect("Text from label doesn't exist");
        if wrapped == my_path {
            // match, do not show
            // switch to it
            notebook.set_current_page(Some(key.try_into().expect("Switching page error")));
            return 0;
        }
    }

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

    let mut my_page = notebook.get_current_page();
    match my_page {
        None => {
            my_page = Some(0);
        }
        Some(number) => {
            my_page = Some(number);
        }
    }
    let my_page = my_page.unwrap() + 1;
    let index = notebook.insert_page(&widget, Some(&tab), Some(my_page));

    notebook.set_menu_label_text(&widget, my_path);

    button.connect_clicked(clone!(@weak notebook => move |_| {
        let index = notebook
            .page_num(&widget)
            .expect("Couldn't get page_num from notebook");
        notebook.remove_page(Some(index));
    }));

    tabs.push(tab);
    notebook.show_all();
    notebook.next_page();
    index
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

pub fn select_it() -> bool {
    true
}
