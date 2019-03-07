pub trait Output {
    fn write(&self, text: &str);
}

pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write(&self, text: &str) {
        println!("{}", text);
    }
}

pub struct NullOutput;

impl Output for NullOutput {
    fn write(&self, _text: &str) {}
}

pub struct OutputFactory;

impl OutputFactory {
    pub fn from_arg(quiet: bool) -> Box<Output> {
        if quiet {
            return Box::new(NullOutput{});
        }

        Box::new(ConsoleOutput{})
    }
}
