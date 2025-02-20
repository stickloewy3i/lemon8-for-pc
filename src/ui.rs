use gtk::{Button, Label, Box};

pub struct AppUI {
    pub window: gtk::Window,
    pub label: Label,
    pub button: Button,
}

impl AppUI {
    pub fn new() -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let label = Label::new(Some("Welcome to Lemon8 For PC!"));
        let button = Button::new_with_label("Click Me");

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&button, true, true, 0);

        window.add(&vbox);
        window.show_all();

        Self { window, label, button }
    }
}