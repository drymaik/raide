use gio::prelude::*;
use glib::clone;
use glib::{TypedValue, Value};
use gtk::prelude::*;
use gtk::{
    Adjustment, Box, Button, CellRendererText, ListStore, Menu, MenuBar, MenuItem, Orientation,
    Paned, ScrolledWindow, TextBuffer, TextIter, TextView, ToolButton, Toolbar, TreeStore,
    TreeStoreExt, TreeView, TreeViewColumn, Widget,
};
use sourceview::{
    Buffer, Completion, CompletionExt, Language, LanguageManager, LanguageManagerBuilder,
    LanguageManagerExt, View, ViewExt,
};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::rc::Rc;
macro_rules! clone {
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
        win.set_default_size(640, 480);
        win.set_title("RUST-IDE");

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
        let my_obj = textview.get_completion();
        println!("Completion: {:?}", my_obj);

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
            save_it(&textbuffer);
            let my_string = build_file();
            outputbuffer.set_text(my_string.as_str());
        });
        let textbuffer_clone_clone = textbuffer_clone.clone();
        run_button.connect_clicked(move |_| {
            save_it(&textbuffer_clone);
            let my_string = run_file();
            outputbuffer_clone.set_text(my_string.as_str());
        });
        let textbuffer_clone_clone_clone = textbuffer_clone_clone.clone();

        save_button.connect_clicked(move |_| {
            save_it(&textbuffer_clone_clone);
        });

        format_button.connect_clicked(move |_| {
            format_it(&textbuffer_clone_clone_clone);
        });

        let mut paned = Paned::new(Orientation::Vertical);
        paned.add1(&text_window);
        paned.add2(&console_window);

        let treestore = TreeStore::new(&[String::static_type()]);
        let treeview = TreeView::new();

        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        treeview.append_column(&column);

        treeview.set_model(Some(&treestore));
        let row1 = treestore.insert(None, 0);
        treestore.set_value(&row1, 0, &"My".to_value());

        treestore.insert_with_values(Some(&row1), None, &[0], &[&"child"]);
        let mut second_paned = Paned::new(Orientation::Horizontal);
        second_paned.add1(&treeview);
        second_paned.add2(&paned);

        gridbox.add(&second_paned);

        win.add(&gridbox);

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());

    Ok(())
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
