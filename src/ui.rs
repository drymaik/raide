use gtk::{Notebook, Box};
use sourceview::{LanguageManager};
/// UI-Structure to hold the notebook, its tabs and the TreeView for the left side
/// The buffers should not be stored 
pub struct UI {
    pub notebook: Notebook,
    pub tabs: Vec<gtk::Box>,
    pub lang: LanguageManager,
}
// TODO Elemente auch noch speichern

// Do not implement, instead generate the struct at the end and put their the elements
