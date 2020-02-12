use std::fmt::Debug;

use failure::Error;

use exonum_cli::{
    command::{
        finalize::Finalize, generate_config::GenerateConfig, generate_template::GenerateTemplate,
        maintenance::Maintenance, run::Run, run_dev::RunDev, ExonumCommand, StandardResult,
    },
    structopt::StructOpt,
};

mod run_async;
mod run_async_as_master;

pub use {run_async::RunAsync, run_async_as_master::RunAsyncAsMaster};

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(author, about)]
pub enum Command {
    /// Initializes and runs node in concurrent manner
    #[structopt(name = "run-async")]
    RunAsync(RunAsync),
    /// Same as [run-async](enum.Command.html#variant.RunAsync) command with generating config template
    #[structopt(name = "run-async-as-master")]
    RunAsyncAsMaster(RunAsyncAsMaster),
    /// Generate common part of the nodes configuration.
    #[structopt(name = "generate-template")]
    GenerateTemplate(GenerateTemplate),
    /// Generate public and private configs of the node.
    #[structopt(name = "generate-config")]
    GenerateConfig(GenerateConfig),
    /// Generate final node configuration using public configs
    /// of other nodes in the network.
    #[structopt(name = "finalize")]
    Finalize(Finalize),
    /// Run the node with provided node config.
    #[structopt(name = "run")]
    Run(Run),
    /// Run the node with auto-generated config.
    #[structopt(name = "run-dev")]
    RunDev(RunDev),
    /// Perform different maintenance actions.
    #[structopt(name = "maintenance")]
    Maintenance(Maintenance),
}

impl ExonumCommand for Command {
    fn execute(self) -> Result<StandardResult, Error> {
        match self {
            Command::RunAsyncAsMaster(command) => command.execute(),
            Command::RunAsync(command) => command.execute(),
            Command::GenerateTemplate(command) => command.execute(),
            Command::GenerateConfig(command) => command.execute(),
            Command::Finalize(command) => command.execute(),
            Command::Run(command) => command.execute(),
            Command::RunDev(command) => command.execute(),
            Command::Maintenance(command) => command.execute(),
        }
    }
}
