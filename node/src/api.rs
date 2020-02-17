use crate::{
    constant,
    model::{public_api::*, AdministrationAddress, Election},
    schema::SchemaImpl,
};
use std::{collections::HashMap, fmt::Debug, iter::FromIterator};

use exonum::{
    blockchain::IndexProof,
    crypto::{Hash, PublicKey},
    runtime::{BlockchainData, Caller},
};
use exonum_merkledb::{
    access::RawAccess, proof_map::Raw, BinaryKey, BinaryValue, MapProof, ObjectHash,
    ProofListIndex, Snapshot,
};
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    pub fn entity_info<A, K, E, HS>(
        blockchain_data: &BlockchainData<&dyn Snapshot>,
        idx_name: &str,
        object: Option<E>,
        object_proof: MapProof<K, E, Raw>,
        history_selector: HS,
    ) -> Info<K, E>
    where
        A: RawAccess,
        K: BinaryKey + ObjectHash + Debug + Clone + Copy,
        E: BinaryValue + ObjectHash + Debug,
        HS: FnOnce() -> ProofListIndex<A, Hash>,
    {
        let IndexProof {
            block_proof,
            index_proof,
            ..
        } = blockchain_data.proof_for_service_index(idx_name).unwrap();

        let proof = Proof {
            to_table: index_proof,
            to_object: object_proof,
        };

        let history = object.map(|_| {
            let history = history_selector();
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

    pub fn participant_info(
        state: &ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<ParticipantInfo> {
        let schema = SchemaImpl::new(state.service_data());
        let address = Caller::Transaction { author: query.key }.address();

        Ok(Self::entity_info(
            &state.data(),
            "participants",
            schema.public.participants.get(&address),
            schema.public.participants.get_proof(address),
            || schema.participant_history.get(&address),
        ))
    }

    pub fn administration_info(
        state: &ServiceApiState<'_>,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<AdministrationInfo> {
        let schema = SchemaImpl::new(state.service_data());
        let address = Caller::Transaction { author: query.key }.address();

        Ok(Self::entity_info(
            &state.data(),
            "administrations",
            schema.public.administrations.get(&address),
            schema.public.administrations.get_proof(address),
            || schema.administration_history.get(&address),
        ))
    }

    pub fn election_info(
        state: &ServiceApiState,
        query: KeyQuery<i64>,
    ) -> api::Result<ElectionInfo> {
        let schema = SchemaImpl::new(state.service_data());

        let idx_name = "elections";

        let IndexProof {
            block_proof,
            index_proof,
            ..
        } = state.data().proof_for_service_index(idx_name).unwrap();

        let proof = HashedProof {
            to_table: index_proof,
            to_object: schema.public.elections.get_proof(query.key),
        };

        let history = schema.public.elections.get(&query.key).map(|_| {
            let history = schema.election_history.get(&query.key);
            let proof = history.get_range_proof(..);

            let transactions = state.data().for_core().transactions();
            let transactions = history
                .iter()
                .map(|record| transactions.get(&record).unwrap())
                .collect();

            History {
                proof,
                transactions,
            }
        });

        Ok(HashedInfo {
            block_proof,
            object_proof: proof,
            history,
        })
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
        let time_schema: exonum_time::TimeSchema<_> = state
            .data()
            .service_schema(constant::TIME_SERVICE_NAME)
            .unwrap();
        let now = time_schema.time.get().expect("can not get time");

        let schema = SchemaImpl::new(state.service_data());

        let iter = schema.public.active_elections(&query.key, now);
        let opt_vec = iter.map(FromIterator::from_iter);
        opt_vec.ok_or_else(api::Error::not_found)
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

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/participants/info", Self::participant_info)
            .endpoint("v1/administrations/info", Self::administration_info)
            .endpoint("v1/elections/info", Self::election_info)
            .endpoint("v1/elections/active", Self::active_elections)
            .endpoint("v1/elections/result", Self::election_results);
    }
}
