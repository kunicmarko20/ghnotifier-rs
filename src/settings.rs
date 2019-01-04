use gtk::*;

pub struct Settings;

impl Settings {
    pub fn new() {
        let window = Window::new(WindowType::Popup);
        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title("Settings");
        header_bar.set_show_close_button(true);
        window.set_titlebar(&header_bar);
        window.set_title("Settings");
        window.set_size_request(200, 100);
        window.set_border_width(10);
        let vertical_box = Box::new(Orientation::Vertical, 6);
        window.add(&vertical_box);
        Self::add_access_token_input(&vertical_box);
        window.show_all();
    }

    fn add_access_token_input(vertical_box: &Box) {
        let label = Label::new("Access token:");
        vertical_box.add(&label);
        let entry = Entry::new();
        entry.set_text("123123123");
        vertical_box.pack_start(&entry, true, true, 0);
    }
}