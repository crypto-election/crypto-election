use crypto_election_node as election;
use exonum_cli::{NodeBuilder, Spec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    exonum::helpers::init_logger()?;

    NodeBuilder::new()
        .with(Spec::new(exonum_time::TimeServiceFactory::default()))
        .with(Spec::new(election::service::ElectionService))
        .run()
        .await
}
