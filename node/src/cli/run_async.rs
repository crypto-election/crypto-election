use std::{
    collections::HashSet, fmt::Debug, fs, io, net::SocketAddr, path::PathBuf, thread::sleep,
    time::Duration,
};

use failure::Error;

use exonum_cli::command::ExonumCommand;
use exonum_cli::{
    command::{finalize::Finalize, generate_config::GenerateConfig, run::Run, StandardResult},
    password::PassInputMethod,
    structopt::StructOpt,
};

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct RunAsync {
    /// Path to directory with `common.toml` and public configurations of nodes.
    #[structopt(long, short = "p")]
    pub public_path: PathBuf,
    /// Maximal number of retries to get configuration of other nodes.
    #[structopt(long, default_value = "64")]
    pub attempts_number: u16,
    /// Delay between retries (ms). By default - 200.
    #[structopt(long, default_value = "200")]
    pub attempt_delay: u16,
    /// Instance label.
    #[structopt(long, short = "l")]
    pub label: String,
    /// All instances labels (optionally including current).
    #[structopt(long)]
    pub labels: Vec<String>,
    /// External IP address of the node used for communications between nodes.
    ///
    /// If no port is provided, the default Exonum port 6333 is used.
    #[structopt(
        long,
        short = "a",
        parse(try_from_str = GenerateConfig::parse_external_address)
    )]
    pub peer_address: SocketAddr,
    /// Listen IP address of the node used for communications between nodes.
    ///
    /// If not provided it combined from all-zeros (0.0.0.0) IP address and
    /// the port number of the `peer-address`.
    #[structopt(long, short = "l")]
    pub listen_address: Option<SocketAddr>,
    /// Don't prompt for passwords when generating private keys.
    #[structopt(long, short = "n")]
    pub no_password: bool,
    /// Passphrase entry method for master key.
    ///
    /// Possible values are: `stdin`, `env{:ENV_VAR_NAME}`, `pass:PASSWORD`.
    /// Default Value is `stdin`.
    /// If `ENV_VAR_NAME` is not specified `$EXONUM_MASTER_PASS` is used
    /// by default.
    #[structopt(long)]
    pub master_key_pass: Option<PassInputMethod>,
    /// Path to the master key file. If empty, file will be placed to <output_dir>.
    #[structopt(long)]
    pub master_key_path: Option<PathBuf>,
    /// Path to a directory where public and private node configuration files
    /// will be saved. By default - same directory, where program is running.
    #[structopt(long, short = "o", default_value = ".")]
    pub output_dir: PathBuf,
    /// Path to a node configuration file which will be created after
    /// running this command.
    #[structopt(long)]
    pub final_config_path: PathBuf,
    /// Listen address for node public API.
    ///
    /// Public API is used mainly for sending API requests to user services.
    #[structopt(long)]
    pub public_api_address: Option<SocketAddr>,
    /// Listen address for node private API.
    ///
    /// Private API is used by node administrators for node monitoring and control.
    #[structopt(long)]
    pub private_api_address: Option<SocketAddr>,
    /// Cross-origin resource sharing options for responses returned by public API handlers.
    #[structopt(long)]
    pub public_allow_origin: Option<String>,
    /// Cross-origin resource sharing options for responses returned by private API handlers.
    #[structopt(long)]
    pub private_allow_origin: Option<String>,
    /// Path to a database directory.
    #[structopt(long, short = "d")]
    pub db_path: PathBuf,
}

impl RunAsync {
    pub fn wait_for_common(&self) -> Result<(), Error> {
        let common_path = self.public_path.join("common.toml");
        for _ in 0..self.attempts_number {
            if common_path.is_file() {
                return Ok(());
            }
            sleep(Duration::from_millis(self.attempt_delay as u64));
        }
        Err(io::Error::new(io::ErrorKind::TimedOut, "Number of attempts is over.").into())
    }

    pub fn generate_config(&self) -> Result<PathBuf, Error> {
        let cmd = GenerateConfig {
            common_config: self.public_path.join("common.toml"),
            output_dir: self.output_dir.clone(),
            peer_address: self.peer_address,
            listen_address: self.listen_address,
            no_password: self.no_password,
            master_key_pass: self.master_key_pass.clone(),
            master_key_path: self.master_key_path.clone(),
        };

        if let StandardResult::GenerateConfig {
            public_config_path,
            private_config_path,
            ..
        } = cmd.execute()?
        {
            fs::copy(public_config_path, self.get_pub_file_path(&self.label))?;
            return Ok(private_config_path);
        }
        unreachable!();
    }

    pub fn wait_for_rest_nodes(&self) -> Result<Vec<PathBuf>, Error> {
        let mut public_cfg_paths: HashSet<_> = self
            .labels
            .iter()
            .map(|l| self.get_pub_file_path(l))
            .collect();

        let public_configs = {
            public_cfg_paths.insert(self.get_pub_file_path(&self.label));
            let result = public_cfg_paths.iter().map(ToOwned::to_owned).collect();
            public_cfg_paths.remove(&self.get_pub_file_path(&self.label));
            result
        };

        for _ in 0..self.attempts_number {
            let paths_iter = public_cfg_paths.clone().into_iter();
            for path in paths_iter {
                if path.is_file() {
                    public_cfg_paths.remove(&path);
                }
            }
            if public_cfg_paths.is_empty() {
                return Ok(public_configs);
            }
            sleep(Duration::from_millis(self.attempt_delay as u64));
        }
        Err(io::Error::new(io::ErrorKind::TimedOut, "Number of attempts is over.").into())
    }

    pub fn finalize(
        &self,
        private_config_path: PathBuf,
        public_configs: Vec<PathBuf>,
    ) -> Result<(), Error> {
        let cmd = Finalize {
            private_config_path,
            output_config_path: self.final_config_path.to_owned(),
            public_configs,
            public_api_address: self.public_api_address,
            private_api_address: self.private_api_address,
            public_allow_origin: self.public_allow_origin.to_owned(),
            private_allow_origin: self.private_allow_origin.to_owned(),
        };

        cmd.execute()?;

        Ok(())
    }

    fn get_pub_file_path(&self, label: &str) -> PathBuf {
        self.public_path.join(format!("pub_{}.toml", label))
    }

    pub fn run(&self) -> Result<StandardResult, Error> {
        let cmd = Run {
            private_api_address: self.private_api_address,
            public_api_address: self.public_api_address,
            master_key_pass: self.master_key_pass.clone(),
            db_path: self.db_path.to_owned(),
            node_config: self.final_config_path.to_owned(),
        };
        cmd.execute()
    }
}

impl ExonumCommand for RunAsync {
    fn execute(self) -> Result<StandardResult, Error> {
        self.wait_for_common()?;
        let private_config = self.generate_config()?;
        let public_configs = self.wait_for_rest_nodes()?;
        self.finalize(private_config, public_configs)?;
        self.run()
    }
}
