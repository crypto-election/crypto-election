use std::{collections::HashMap, fmt::Debug, iter::FromIterator};

use exonum::{crypto::PublicKey, runtime::CallerAddress};
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

use crate::{
    model::{public_api::*, AdministrationAddress, Election, ElectionAddress},
    schema::SchemaImpl,
};

/// Election public web api
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    /// Plugs in all Public API methods
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/participants/info", Self::participant_info)
            .endpoint("v1/administrations/info", Self::administration_info)
            .endpoint("v1/elections/info", Self::election_info)
            .endpoint("v1/elections/active", Self::active_elections)
            .endpoint("v1/elections/result", Self::election_results);
    }

    /// Gets complete participant info
    ///
    /// ## API address
    /// `v1/participants/info`
    pub async fn participant_info(
        state: ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<ParticipantInfo> {
        let index_pair = {
            let schema = SchemaImpl::new(state.service_data());
            (schema.public.participants, schema.participant_history)
        };

        ProofedInfo::try_from_indexes(
            &state.data(),
            "participants",
            CallerAddress::from_key(query.key),
            index_pair,
        )
        .map_err(api::Error::internal)
    }

    /// Gets complete administration info
    ///
    /// ## API address
    /// `v1/administrations/info`
    pub async fn administration_info(
        state: ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<AdministrationInfo> {
        let index_pair = {
            let schema = SchemaImpl::new(state.service_data());
            (schema.public.administrations, schema.administration_history)
        };
        let key = CallerAddress::from_key(query.key);

        ProofedInfo::try_from_indexes(&state.data(), "administrations", key, index_pair)
            .map_err(api::Error::internal)
    }

    /// Gets complete election info
    ///
    /// ## API address
    /// `v1/elections/info`
    pub async fn election_info(
        state: ServiceApiState,
        query: KeyQuery<ElectionAddress>,
    ) -> api::Result<ElectionInfo> {
        let index_pair = {
            let schema = SchemaImpl::new(state.service_data());
            (schema.public.elections, schema.election_history)
        };

        ProofedInfo::try_from_indexes(&state.data(), "elections", query.key, index_pair)
            .map_err(api::Error::internal)
    }

    #[doc(hidden)]
    pub async fn all_elections(state: ServiceApiState, _: ()) -> api::Result<Vec<Election>> {
        Ok(SchemaImpl::new(state.service_data())
            .public
            .elections
            .values()
            .collect())
    }

    pub async fn active_elections(
        state: ServiceApiState,
        query: KeyQuery<AdministrationAddress>,
    ) -> api::Result<Vec<Election>> {
        let schema = SchemaImpl::new(state.service_data());

        let now = schema
            .config
            .get()
            .ok_or_else(|| {
                Box::new(exonum_merkledb::Error::new("Can not read service config"))
                    as Box<dyn failure::Fail>
            })
            .and_then(|config| {
                state
                    .data()
                    .service_schema(config.time_service_name.as_str())
                    .map_err(|e| Box::new(e) as Box<dyn failure::Fail>)
            })
            .and_then(|time_schema: exonum_time::TimeSchema<_>| {
                time_schema.time.get().ok_or_else(|| {
                    Box::new(exonum_merkledb::Error::new("Can not get time"))
                        as Box<dyn failure::Fail>
                })
            })
            .map_err(api::Error::internal)?;

        schema
            .public
            .active_elections(&query.key, now)
            .map(FromIterator::from_iter)
            .ok_or_else(api::Error::not_found)
    }

    pub async fn election_results(
        state: ServiceApiState,
        query: KeyQuery<ElectionAddress>,
    ) -> api::Result<HashMap<i32, u32>> {
        SchemaImpl::new(state.service_data())
            .public
            .election_results(query.key)
            .ok_or_else(api::Error::not_found)
    }
}
