use crypto_election_node as election;
use election::cli::Command;
use exonum_cli::NodeBuilder;

fn main() -> Result<(), failure::Error> {
    exonum::helpers::init_logger()?;

    NodeBuilder::new()
        .with_rust_service(exonum_time::TimeServiceFactory::default())
        .with_default_rust_service(election::service::ElectionService)
        .run_with_command_type::<Command>()
}
