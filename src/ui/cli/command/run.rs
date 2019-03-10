use structopt::StructOpt;
use super::Command;
use super::Output;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Run {
    #[structopt(short = "d", long = "detached")]
    /// Start application in its own process group
    pub detached: bool,
}

impl Command for Run {
    fn execute(&self, output: Box<Output>) {
        output.write("Starting Github Notifier.");
        crate::ui::gui::init();
    }
}
