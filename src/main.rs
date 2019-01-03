#[macro_use] extern crate serde_derive;

mod indicator;
mod menu;
mod notifier;
mod github_client;
mod config;

fn main() {
    gtk::init().unwrap();
    let mut indicator = indicator::Indicator::new();
    let menu = menu::Menu::new();
    indicator.set_menu(menu);
    let mut notifier = notifier::Notifier::new(indicator);
    notifier.execute();
    gtk::main();
}
