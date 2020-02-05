use crypto_election_node as election;
use exonum_cli::NodeBuilder;

fn main() {
    exonum::helpers::init_logger().unwrap();

    NodeBuilder::new()
        .with_service(exonum_time::TimeServiceFactory::default())
        .with_service(election::service::ElectionService)
        .run()
        .unwrap_or_else(|e| panic!("{}", e))
}
