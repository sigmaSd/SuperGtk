use gtk::*;
use supergtk::{RefGtk, SUPERGTK};

fn main() {
    let sk = SUPERGTK::new();

    create_gui(&sk);
    connect_signals(&sk);

    SUPERGTK::run();
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
        let sk_cc = sk_c.clone();
        let name = entry.get_text().unwrap().to_string();
        let name = string_to_static_str(&name);

        let table = sk_c.borrow().get_grid("MainTable");

        let button = sk_c.borrow_mut().create_button(name);
        button.set_label(name);
        button.connect_clicked(move |_| open_client_window(&sk_cc, name));

        table.attach_next_to(
            &button,
            table.get_children().iter().next(),
            PositionType::Bottom,
            10,
            10,
        );
        table.show_all();

        win.destroy();
    });
}

fn open_client_window(sk: &RefGtk, name: &'static str) {
    let sk_c = sk.clone();
    let mut sk = sk.borrow_mut();

    let win = sk.create_win("ClientWin");
    let vbox = sk.create_box("ClientWin", Orientation::Vertical, 10);

    let headerbar = sk.create_headerbar("ClientWin");
    let entry = sk.create_entry("ClientWin");
    let save_btn = sk.create_button("ClientWin");

    vbox.add(&headerbar);
    vbox.add(&entry);
    vbox.add(&save_btn);

    win.add(&vbox);
    win.show_all();

    save_btn.connect_clicked(move |_| {
        let sk = sk_c.borrow();

        let table = sk.get_grid("MainTable");
        let name_btn = sk.get_button(name);

        let text = entry.get_text().unwrap().to_string();
        let text = string_to_static_str(&text);
        let label = Label::new(text);

        table.attach_next_to(&label, &name_btn, PositionType::Right, 10, 10);
        table.show_all();

        win.destroy();
    });
}

fn string_to_static_str(s: &str) -> &'static str {
    std::boxed::Box::leak(s.to_string().into_boxed_str())
}
