use exonum::{
    merkledb::BinaryValue,
    runtime::{CommonError, ExecutionContext, ExecutionError},
};
use exonum_derive::{ServiceDispatcher, ServiceFactory};
use exonum_rust_runtime::{api::ServiceApiBuilder, Service};
use exonum_supervisor::Configure;
use exonum_time::TimeSchema;

use crate::{
    api::PublicApi, model::transactions::Config, schema::SchemaImpl, tx_behavior::ElectionInterface,
};

#[derive(Debug, ServiceFactory, ServiceDispatcher)]
#[service_dispatcher(implements("ElectionInterface"))]
#[service_factory(proto_sources = "crate::proto")]
pub struct ElectionService;

fn verify_config(context: &ExecutionContext<'_>, config: &Config) -> Result<(), ExecutionError> {
    let _time_schema: TimeSchema<_> = context
        .data()
        .service_schema(config.time_service_name.as_str())?;

    Ok(())
}

impl Service for ElectionService {
    fn initialize(
        &self,
        context: ExecutionContext<'_>,
        params: Vec<u8>,
    ) -> Result<(), ExecutionError> {
        let config = Config::from_bytes(params.into()).map_err(CommonError::malformed_arguments)?;
        verify_config(&context, &config)?;

        SchemaImpl::new(context.service_data()).config.set(config);
        Ok(())
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}

impl Configure for ElectionService {
    type Params = Config;

    fn verify_config(
        &self,
        context: ExecutionContext<'_>,
        params: Self::Params,
    ) -> Result<(), ExecutionError> {
        verify_config(&context, &params)
    }

    fn apply_config(
        &self,
        context: ExecutionContext<'_>,
        params: Self::Params,
    ) -> Result<(), ExecutionError> {
        let mut schema = SchemaImpl::new(context.service_data());
        schema.config.set(params);
        Ok(())
    }
}
