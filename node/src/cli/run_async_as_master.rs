use failure::Error;

use exonum_cli::{
    command::{generate_template::GenerateTemplate, ExonumCommand, StandardResult},
    structopt::StructOpt,
};
use exonum_supervisor::mode::Mode as SupervisorMode;

use super::RunAsync;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct RunAsyncAsMaster {
    #[serde(flatten)]
    #[structopt(flatten)]
    run_async: RunAsync,
    /// Number of validators in the network.
    #[structopt(long)]
    pub validators_count: u32,
    /// Supervisor service mode. Possible options are "simple" and "decentralized".
    #[structopt(long, default_value = "simple")]
    pub supervisor_mode: SupervisorMode,
}

impl RunAsyncAsMaster {
    pub fn generate_template(&self) -> Result<(), Error> {
        let cmd = GenerateTemplate {
            common_config: self.run_async.public_path.join("config.toml"),
            validators_count: self.validators_count,
            supervisor_mode: self.supervisor_mode.to_owned(),
        };

        cmd.execute()?;
        Ok(())
    }
}

impl ExonumCommand for RunAsyncAsMaster {
    fn execute(self) -> Result<StandardResult, Error> {
        self.generate_template()?;
        self.run_async.execute()
    }
}
