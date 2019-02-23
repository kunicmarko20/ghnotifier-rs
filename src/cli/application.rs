use structopt::StructOpt;
use super::command;

pub struct Application;

impl Application {
    pub fn run() {
        match GithubNotifier::from_args().command {
            SubCommand::Install(command) => Self::execute(command),
            SubCommand::Start(command) => Self::execute(command),
            SubCommand::Uninstall(command) => Self::execute(command),
        }
    }

    fn execute<T: command::Command>(command: T) {
        command.execute()
    }
}

#[derive(Debug, StructOpt)]
struct GithubNotifier {
    #[structopt(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "install")]
    Install(command::install::Install),
    #[structopt(name = "start")]
    Start(command::start::Start),
    #[structopt(name = "uninstall")]
    Uninstall(command::uninstall::Uninstall),
}
