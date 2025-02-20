use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Lemon8 For PC");
    window.set_default_size(400, 200);

    let label = Label::new(Some("Welcome to Lemon8 For PC!"));
    let button = Button::new_with_label("Click Me");

    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, true, true, 0);

    window.add(&vbox);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}