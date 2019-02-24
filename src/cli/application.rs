use structopt::StructOpt;
use super::command;

pub struct Application;

impl Application {
    pub fn run() {
        match GithubNotifier::from_args().command {
            SubCommand::Install(command) => Self::execute(command),
            SubCommand::Run(command) => Self::execute(command),
            SubCommand::RunDetached(command) => Self::execute(command),
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
    /// Installs the needed things.
    Install(command::install::Install),
    #[structopt(name = "run")]
    /// Starts the Application that blocks the terminal
    Run(command::run::Run),
    #[structopt(name = "run-detached")]
    /// Starts the Application that doesn't blocks the terminal
    RunDetached(command::run_detached::RunDetached),
    #[structopt(name = "uninstall")]
    /// Removes everything
    Uninstall(command::uninstall::Uninstall),
}
