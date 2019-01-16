use gtk::*;
use super::config::Config;

pub struct Settings;

impl Settings {
    pub fn new() {
        let window = Self::build_window();
        let vertical_box = Box::new(Orientation::Vertical, 6);
        let config = Config::new();
        let access_token_field = Self::build_access_token_field(&vertical_box, &config);
        let refresh_time_field = Self::build_refresh_time_field(&vertical_box, &config);
        let button = Self::build_save_button(&vertical_box);
        window.add(&vertical_box);
        window.show_all();
        button.connect_clicked(move |_| {
            let mut config = Config::new();
            config.set("access_token", access_token_field.get_text().unwrap());
            config.set("refresh_time", String::from(
                match refresh_time_field.get_active() {
                    1 => "30",
                    2 => "60",
                    3 => "300",
                    _ => "10"
                }
            ));
            window.close();
        });
    }

    fn build_window() -> Window {
        let window = Window::new(WindowType::Toplevel);
        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title("Settings");
        header_bar.set_show_close_button(true);
        window.set_titlebar(&header_bar);
        window.set_size_request(600, 400);
        window.set_border_width(10);
        window.set_resizable(false);
        window.set_position(WindowPosition::CenterAlways);
        window
    }

    fn build_access_token_field(vertical_box: &Box, config: &Config) -> Entry {
        vertical_box.add(&Label::new("Access token:"));
        let access_token_field = Entry::new();
        access_token_field.set_text(&config.get("access_token").unwrap());
        vertical_box.pack_start(&access_token_field, true, true, 0);
        access_token_field
    }

    fn build_refresh_time_field(vertical_box: &Box, config: &Config) -> ComboBoxText {
        vertical_box.add(&Label::new("Refresh time:"));
        let refresh_time = ComboBoxText::new();
        refresh_time.append(Some("10"), "10 seconds");
        refresh_time.append(Some("30"), "30 seconds");
        refresh_time.append(Some("60"), "60 seconds");
        refresh_time.append(Some("300"), "300 seconds");
        refresh_time.set_active(
            match config.get("refresh_Time").unwrap().as_str() {
                "30" => 1,
                "60" => 2,
                "300" => 3,
                _ => 0
            }
        );
        vertical_box.pack_start(&refresh_time, true, true, 0);
        refresh_time
    }

    fn build_save_button(vertical_box: &Box) -> Button {
        let button = Button::new_with_label("Save");
        vertical_box.pack_start(&button, true, true, 0);
        button
    }
}
