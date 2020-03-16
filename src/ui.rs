use gtk::{Notebook, TreeSelection};
use sourceview::LanguageManager;
/// UI-Structure to hold the notebook, its tabs and the TreeView for the left side
/// The buffers should not be stored
pub struct UI {
    pub notebook: Notebook,
    pub tabs: Vec<gtk::Box>,
    pub lang: LanguageManager,
    pub tree_selection: TreeSelection,
}
