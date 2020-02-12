use crypto_election_node as election;
use exonum_cli::NodeBuilder;

use failure::Error;

fn main() -> Result<(), Error> {
    exonum::helpers::init_logger()?;

    NodeBuilder::new()
        .with_rust_service(exonum_time::TimeServiceFactory::default())
        .with_default_rust_service(election::service::ElectionService)
        .run()
}
