use exonum::runtime::{ExecutionContext, ExecutionError};
use exonum_derive::{ServiceDispatcher, ServiceFactory};
use exonum_rust_runtime::{api::ServiceApiBuilder, Service};

use crate::{api::PublicApi, schema::SchemaImpl, tx_behavior::ElectionInterface};

#[derive(Debug, ServiceFactory, ServiceDispatcher)]
#[service_dispatcher(implements("ElectionInterface"))]
#[service_factory(proto_sources = "crate::proto")]
pub struct ElectionService;

impl Service for ElectionService {
    fn initialize(
        &self,
        context: ExecutionContext<'_>,
        _params: Vec<u8>,
    ) -> Result<(), ExecutionError> {
        SchemaImpl::new(context.service_data());
        Ok(())
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}
