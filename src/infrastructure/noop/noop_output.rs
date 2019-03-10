use crate::ui::cli::output::Output;

pub struct NoopOutput;

impl Output for NoopOutput {
    fn write(&self, _text: &str) {}
}
