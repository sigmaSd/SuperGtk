use gtk::*;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::rc::Rc;

pub type RefGtk = Rc<RefCell<SUPERGTK>>;

/// Use it to create and get widgets, and to start gtk
///
/// Any widget created by it will be internally saved so you can retreive it with its id
#[derive(Default)]
pub struct SUPERGTK {
    widgets_map: HashMap<&'static str, HashMap<&'static str, Widget>>,
}

impl SUPERGTK {
    /// Init gtk and creates a Rc<RefCell\<SUPERGTK\>> for ease of use
    ///
    /// Use it at the start of the program
    pub fn new() -> RefGtk {
        gtk::init().expect("Error while initilizing gtk");
        Default::default()
    }

    /// Run the app
    ///
    /// Use it at the end of the program
    pub fn run() {
        gtk::main();
    }
}

/// Create widgets with specified id
///
/// Same widget, kind same id -> will be overwritten
///
/// Diffrent widget, same id -> will be added to that id's widget group
impl SUPERGTK {
    pub fn create_win(&mut self, id: &'static str) -> Window {
        let win = Window::new(WindowType::Toplevel);
        let win_c = win.clone();
        self.add_to_map(id, win, "Window");
        win_c
    }

    pub fn create_grid(&mut self, id: &'static str) -> Grid {
        let grid = Grid::new();
        let grid_c = grid.clone();
        self.add_to_map(id, grid, "Grid");
        grid_c
    }

    pub fn create_button(&mut self, id: &'static str) -> Button {
        let button = Button::new();
        let button_c = button.clone();
        self.add_to_map(id, button, "Button");
        button_c
    }

    pub fn create_label(&mut self, id: &'static str) -> Label {
        let label = Label::new_with_mnemonic(None);
        let label_c = label.clone();
        self.add_to_map(id, label, "Label");
        label_c
    }

    pub fn create_entry(&mut self, id: &'static str) -> Entry {
        let entry = Entry::new();
        let entry_c = entry.clone();
        self.add_to_map(id, entry, "Entry");
        entry_c
    }

    pub fn create_box(&mut self, id: &'static str, orientation: Orientation, spacing: i32) -> Box {
        let gtk_box = Box::new(orientation, spacing);
        let gtk_box_c = gtk_box.clone();
        self.add_to_map(id, gtk_box, "Box");
        gtk_box_c
    }

    pub fn create_headerbar(&mut self, id: &'static str) -> HeaderBar {
        let header_bar = HeaderBar::new();
        let header_bar_c = header_bar.clone();
        self.add_to_map(id, header_bar, "HeaderBar");
        header_bar_c
    }
}

/// Functions to get the widgets back by id
impl SUPERGTK {
    pub fn get_win(&self, id: &str) -> Window {
        self.widgets_map[id]["Window"]
            .clone()
            .downcast::<Window>()
            .expect("Not a win")
    }
    pub fn get_grid(&self, id: &str) -> Grid {
        self.widgets_map[id]["Grid"]
            .clone()
            .downcast::<Grid>()
            .expect("Not a grid")
    }
    pub fn get_button(&self, id: &str) -> Button {
        self.widgets_map[id]["Button"]
            .clone()
            .downcast::<Button>()
            .expect("Not a button")
    }
    pub fn get_label(&self, id: &str) -> Label {
        self.widgets_map[id]["Label"]
            .clone()
            .downcast::<Label>()
            .expect("Not a label")
    }
    pub fn get_entry(&self, id: &str) -> Entry {
        self.widgets_map[id]["Entry"]
            .clone()
            .downcast::<Entry>()
            .expect("Not a entry")
    }
    pub fn get_box(&self, id: &str) -> Box {
        self.widgets_map[id]["Box"]
            .clone()
            .downcast::<Box>()
            .expect("Not a box")
    }
    pub fn get_headerbar(&self, id: &str) -> HeaderBar {
        self.widgets_map[id]["HeaderBar"]
            .clone()
            .downcast::<HeaderBar>()
            .expect("Not a headerbar")
    }
}

// Private functions
impl SUPERGTK {
    fn add_to_map<T: IsA<Widget>>(&mut self, id: &'static str, widget: T, w_type: &'static str) {
        let widgets_map = &mut self.widgets_map;
        match widgets_map.entry(id) {
            hash_map::Entry::Occupied(widgets_group) => {
                widgets_group.into_mut().insert(w_type, widget.upcast());
            }
            hash_map::Entry::Vacant(_) => {
                let mut widget_group = HashMap::new();
                widget_group.insert(w_type, widget.upcast());
                widgets_map.insert(id, widget_group);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ()
    }
}
