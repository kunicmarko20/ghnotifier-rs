use structopt::StructOpt;
use super::command;

pub struct Application;

impl Application {
    pub fn run() {
        match GithubNotifier::from_args().command {
            SubCommand::Install(command) => Self::execute(command),
            SubCommand::Run(command) => {
                if !command.detached {
                    Self::execute(command);
                    return;
                }

                Self::execute(command::run_detached::RunDetached{})
            },
            SubCommand::SelfUpdate(command) => Self::execute(command),
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
    pub command: SubCommand
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "install")]
    /// Installs the needed things.
    Install(command::install::Install),
    #[structopt(name = "run")]
    /// Starts the Application
    Run(command::run::Run),
    #[structopt(name = "self-update")]
    /// Update application to a newer version
    SelfUpdate(command::self_update::SelfUpdate),
    #[structopt(name = "uninstall")]
    /// Removes everything
    Uninstall(command::uninstall::Uninstall),
}
