use crate::ui::cli::output::Output;
use crate::infrastructure::noop::noop_output::NoopOutput;
use crate::infrastructure::console::console_output::ConsoleOutput;

pub struct OutputFactory;

impl OutputFactory {
    pub fn from_arg(quiet: bool) -> Box<Output> {
        if quiet {
            return Box::new(NoopOutput);
        }

        Box::new(ConsoleOutput)
    }
}
