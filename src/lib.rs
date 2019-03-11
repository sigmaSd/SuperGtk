use gtk::*;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::rc::Rc;

pub type RefGtk = Rc<RefCell<SUPERGTK>>;

type WidgetsMap<T> = Rc<RefCell<HashMap<&'static str, T>>>;

#[derive(Default)]
pub struct SUPERGTK {
    widgets_map: WidgetsMap<HashMap<&'static str, Widget>>,
}

impl SUPERGTK {
    pub fn new() -> RefGtk {
        gtk::init().expect("Error while initilizing gtk");
        Default::default()
    }

    pub fn run() {
        gtk::main();
    }

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

    // getter

    pub fn get_box(&self, id: &str) -> Box {
        self.widgets_map.borrow()[id]["Box"]
            .clone()
            .downcast::<Box>()
            .expect("Not a box")
    }
    pub fn get_label(&self, id: &str) -> Label {
        self.widgets_map.borrow()[id]["Label"]
            .clone()
            .downcast::<Label>()
            .expect("Not a label")
    }
    pub fn get_entry(&self, id: &str) -> Entry {
        self.widgets_map.borrow()[id]["Entry"]
            .clone()
            .downcast::<Entry>()
            .expect("Not a entry")
    }
    pub fn get_button(&self, id: &str) -> Button {
        self.widgets_map.borrow()[id]["Button"]
            .clone()
            .downcast::<Button>()
            .expect("Not a button")
    }
    pub fn get_win(&self, id: &str) -> Window {
        self.widgets_map.borrow()[id]["Window"]
            .clone()
            .downcast::<Window>()
            .expect("Not a win")
    }
    pub fn get_grid(&self, id: &str) -> Grid {
        self.widgets_map.borrow()[id]["Grid"]
            .clone()
            .downcast::<Grid>()
            .expect("Not a grid")
    }

    fn add_to_map<T: IsA<Widget>>(&mut self, id: &'static str, widget: T, w_type: &'static str) {
        let mut widgets_map = self.widgets_map.borrow_mut();
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
