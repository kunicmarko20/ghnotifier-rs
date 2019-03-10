use super::output::Output;

pub trait Command {
    fn execute(&self, output: Box<Output>);
}

pub mod install;
pub mod run;
pub mod run_detached;
pub mod uninstall;
pub mod self_update;
