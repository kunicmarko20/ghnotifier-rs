use crate::ui::cli::output::Output;

pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write(&self, text: &str) {
        println!("{}", text);
    }
}
