# SuperGtk
Wrapper over Gtk-rs for(maybe) more ergonomic use

**Example:**
```rust
use gtk::*;
use supergtk::{RefGtk, SuperGtk};

fn main() {
    let sk = SuperGtk::new();

    create_gui(&sk);
    connect_signals(&sk);

    SuperGtk::run();
}

fn create_gui(sk: &RefGtk) {
    let mut sk = sk.borrow_mut();

    let win = sk.create_win("MainWin");
    let big_box = sk.create_box("BigBox", Orientation::Vertical, 10);

    let header_bar = sk.create_headerbar("HeaderBar");
    header_bar.add(&sk.create_button("AddBtn"));
    header_bar.add(&sk.create_button("RmBtn"));

    let main_table = sk.create_grid("MainTable");

    big_box.add(&header_bar);
    big_box.add(&main_table);

    win.add(&big_box);
    win.show_all();
}

fn connect_signals(sk: &RefGtk) {
    connect_main_win(sk);
    connect_header_bar(sk);
}

fn connect_main_win(sk: &RefGtk) {
    let sk = sk.borrow();

    let win = sk.get_win("MainWin");
    win.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}

fn connect_header_bar(sk: &RefGtk) {
    let sk_c = sk.clone();
    let sk = sk.borrow();

    let add_button = sk.get_button("AddBtn");
    let _rm_button = sk.get_button("RmBtn");

    add_button.connect_clicked(move |_| {
        create_entry_win(&sk_c);
    });
}

fn create_entry_win(sk: &RefGtk) {
    let sk_c = sk.clone();
    let mut sk = sk.borrow_mut();

    let win = sk.create_win("EntryWin");
    let entry = sk.create_entry("EntryEntry");

    win.add(&entry);
    win.show_all();

    entry.connect_activate(move |entry| {
        let name = entry.get_text().unwrap().to_string();
        let table = sk_c.borrow().get_grid("MainTable");

        let label = sk_c.borrow_mut().create_label(string_to_static_str(&name));
        label.set_text(&name);

        table.attach_next_to(
            &label,
            table.get_children().iter().next(),
            PositionType::Bottom,
            10,
            10,
        );
        table.show_all();

        win.destroy();
    });
}

fn string_to_static_str(s: &str) -> &'static str {
    std::boxed::Box::leak(s.to_string().into_boxed_str())
}
```
