extern crate gtk_sys;
extern crate gtk;

mod indicator;
mod menu;
mod notifier;

fn main() {
    gtk::init().unwrap();
    let mut indicator = indicator::Indicator::new();
    let menu = menu::Menu::new();
    indicator.set_menu(menu);
    let notifier = notifier::Notifier::new(indicator);
    notifier.notify();
    gtk::main();
}
