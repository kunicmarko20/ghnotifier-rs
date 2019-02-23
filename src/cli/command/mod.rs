pub mod install;
pub mod start;
pub mod uninstall;

pub trait Command {
    fn execute(&self);
}
