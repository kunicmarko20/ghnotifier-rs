use structopt::StructOpt;
use super::output::{Output, OutputFactory};
use super::command;

pub struct Application;

impl Application {
    pub fn run() {
        let ghnotifier = GithubNotifier::from_args();

        Self::execute(
            Self::inner_command(ghnotifier.sub_command),
            OutputFactory::from_arg(ghnotifier.quiet)
        )
    }

    fn inner_command(sub_command: SubCommand) -> Box<command::Command> {
        match sub_command {
            SubCommand::Install(command) => return Box::new(command),
            SubCommand::Run(command) => {
                if !command.detached {
                    return Box::new(command);
                }

                return Box::new(command::run_detached::RunDetached{});
            },
            SubCommand::SelfUpdate(command) => return Box::new(command),
            SubCommand::Uninstall(command) => return Box::new(command),
        }
    }

    fn execute(command: Box<command::Command>, output: Box<Output>) {
        command.execute(output)
    }
}

#[derive(Debug, StructOpt)]
struct GithubNotifier {
    #[structopt(short = "q", long = "quiet")]
    /// Don't output anything to the console
    pub quiet: bool,
    #[structopt(subcommand)]
    pub sub_command: SubCommand
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
