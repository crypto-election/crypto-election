use std::{collections::HashMap, fmt::Debug, iter::FromIterator};

use exonum::{
    blockchain::IndexProof,
    crypto::{Hash, PublicKey},
    runtime::{BlockchainData, CallerAddress},
};
use exonum_merkledb::{
    access::RawAccess, indexes::proof_map::ToProofPath, BinaryKey, BinaryValue, Group, ObjectHash,
    ProofListIndex, ProofMapIndex, Snapshot,
};
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

use crate::{
    model::{public_api::*, AdministrationAddress, Election},
    schema::SchemaImpl,
};
use exonum_merkledb::access::Access;

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
    pub fn participant_info(
        state: &ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<ParticipantInfo> {
        let schema = SchemaImpl::new(state.service_data());
        let address = CallerAddress::from_key(query.key);

        Ok(Self::entity_info(
            &state.data(),
            "participants",
            &schema.public.participants,
            address,
            &schema.participant_history,
        ))
    }

    pub fn administration_info(
        state: &ServiceApiState<'_>,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<AdministrationInfo> {
        let schema = SchemaImpl::new(state.service_data());
        let address = CallerAddress::from_key(query.key);

        Ok(Self::entity_info(
            &state.data(),
            "administrations",
            &schema.public.administrations,
            address,
            &schema.administration_history,
        ))
    }

    pub fn election_info(
        state: &ServiceApiState,
        query: KeyQuery<i64>,
    ) -> api::Result<ElectionInfo> {
        let schema = SchemaImpl::new(state.service_data());

        Ok(Self::entity_info(
            &state.data(),
            "elections",
            &schema.public.elections,
            query.key,
            &schema.election_history,
        ))
    }

    pub fn all_elections(state: &ServiceApiState, _: ()) -> api::Result<Vec<Election>> {
        Ok(SchemaImpl::new(state.service_data())
            .public
            .elections
            .values()
            .collect())
    }

    pub fn active_elections(
        state: &ServiceApiState,
        query: KeyQuery<AdministrationAddress>,
    ) -> api::Result<Vec<Election>> {
        let schema = SchemaImpl::new(state.service_data());

        let config = schema.config.get().expect("Can't read service config");

        let time_schema: exonum_time::TimeSchema<_> = state
            .data()
            .service_schema(config.time_service_name.as_str())
            .unwrap();

        let now = time_schema.time.get().expect("can not get time");

        schema
            .public
            .active_elections(&query.key, now)
            .map(FromIterator::from_iter)
            .ok_or_else(api::Error::not_found)
    }

    pub fn election_results(
        state: &ServiceApiState,
        query: KeyQuery<i64>,
    ) -> api::Result<HashMap<i32, u32>> {
        SchemaImpl::new(state.service_data())
            .public
            .election_results(query.key)
            .ok_or_else(api::Error::not_found)
    }

    fn entity_info<
        A: Access,
        K: BinaryKey + ObjectHash + Debug + ToOwned,
        V: BinaryValue + ObjectHash + Debug,
        KMW: KeyModeWrapper + Debug,
    >(
        blockchain_data: &BlockchainData<&dyn Snapshot>,
        idx_name: &str,
        object_index: &ProofMapIndex<A::Base, K, V, KMW::KeyMode>,
        key: K,
        history_index: &Group<A, K, ProofListIndex<A::Base, Hash>>,
    ) -> Info<K::Owned, V, KMW>
    where
        KMW::KeyMode: ToProofPath<K> + Debug,
    {
        let IndexProof {
            block_proof,
            index_proof,
            ..
        } = blockchain_data.proof_for_service_index(idx_name).unwrap();

        let proof = Proof {
            to_table: index_proof,
            to_object: object_index.get_proof(key.to_owned()),
        };

        let history = object_index.get(&key).map(|_| {
            let history = history_index.get(&key);
            let proof = history.get_range_proof(..);

            let transactions = blockchain_data.for_core().transactions();
            let transactions = history
                .iter()
                .map(|record| transactions.get(&record).unwrap())
                .collect();

            History {
                proof,
                transactions,
            }
        });

        Info {
            block_proof,
            object_proof: proof,
            history,
        }
    }
}
