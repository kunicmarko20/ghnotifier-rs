use structopt::StructOpt;
use super::Command;
use crate::{menu, indicator, config, worker, github_client};
use arc_guard::ArcGuard;
use super::Output;
use crate::notifier::NotifierFactory;
use crate::logger::FileLogger;

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
        gtk::init().unwrap();

        let config = ArcGuard::new(config::Config::new());

        let menu = menu::Menu::new(config.clone());

        let indicator = ArcGuard::new(indicator::Indicator::new(menu));

        let github_client = github_client::GithubClient::new(
            config.clone()
        );

        let mut worker = worker::Worker::new(
            github_client,
            config.clone(),
            indicator,
            NotifierFactory::new(),
                Box::new(FileLogger::new())
        );

        let config = config.clone();
        std::thread::spawn(move || {
            loop {
                &worker.execute();
                let refresh_time = config.execute(|config| -> u64 {
                    let config = config.lock().unwrap();
                    config.get("refresh_time").unwrap().parse::<u64>().unwrap()
                });

                std::thread::sleep(
                    std::time::Duration::from_secs(
                        refresh_time
                    )
                );
            }
        });

        gtk::main();
    }
}
