use core::{
    constant::BLOCKCHAIN_SERVICE_ID,
    model::{public_api::*, Election},
    schema::ElectionSchema,
};

use either::*;

use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
};

use exonum::blockchain::Blockchain;
use exonum_merkledb::{BinaryKey, BinaryValue, IndexAccess, MapProof, ObjectHash, ProofListIndex};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    pub fn entity_info<A, DSC, K, E, PS, OS, HS>(
        master_schema_or_metadata: Either<
            &blockchain::Schema<A>,
            (&BlockProof, &MapProof<Hash, Hash>),
        >,
        database_schema: &DSC,
        blockchain: &Blockchain,
        object_selector: OS,
        proof_selector: PS,
        history_selector: HS,
    ) -> Info<K, E>
    where
        A: IndexAccess,
        K: BinaryKey + ObjectHash + Debug + Clone + Copy,
        E: BinaryValue + ObjectHash + Debug,
        OS: FnOnce(&DSC) -> Option<E>,
        PS: FnOnce(&DSC) -> MapProof<K, E>,
        HS: FnOnce(&DSC) -> ProofListIndex<A, Hash>,
    {
        let (block_proof, to_table) = master_schema_or_metadata.either(
            |master_schema| {
                let max_height = master_schema.block_hashes_by_height().len() - 1;
                (
                    master_schema
                        .block_and_precommits(Height(max_height))
                        .unwrap(),
                    master_schema.get_proof_to_service_table(BLOCKCHAIN_SERVICE_ID, 0),
                )
            },
            |(block_proof, to_table)| (block_proof.clone(), to_table.clone()),
        );

        let object_proof = Proof {
            to_table,
            to_object: proof_selector(database_schema),
        };

        let object = object_selector(database_schema);

        let history = object.map(|_| {
            let history = history_selector(database_schema);
            let explorer = BlockchainExplorer::new(blockchain);

            let transactions = history
                .iter()
                .map(|record| explorer.transaction_without_proof(&record).unwrap())
                .collect::<Vec<_>>();

            History {
                proof: history.get_range_proof(0..history.len()),
                transactions,
            }
        });

        Info {
            block_proof,
            object_proof,
            history,
        }
    }

    pub fn participant_info(
        state: &ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<ParticipantInfo> {
        let access = state.snapshot();

        Ok(Self::entity_info(
            Left(&blockchain::Schema::new(&access)),
            &ElectionSchema::new(&access),
            state.blockchain(),
            |schema| schema.participant(&query.key),
            |schema| schema.participants().get_proof(query.key),
            |schema| schema.participant_history(&query.key),
        ))
    }

    pub fn administration_info(
        state: &ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<AdministrationInfo> {
        let access = state.snapshot();

        Ok(Self::entity_info(
            Left(&blockchain::Schema::new(&access)),
            &ElectionSchema::new(&access),
            state.blockchain(),
            |schema| schema.administration(&query.key),
            |schema| schema.administrations().get_proof(query.key),
            |schema| schema.administration_history(&query.key),
        ))
    }

    pub fn election_info(
        state: &ServiceApiState,
        query: KeyQuery<i64>,
    ) -> api::Result<ElectionInfo> {
        let access = state.snapshot();
        Ok(Self::entity_info(
            Left(&blockchain::Schema::new(&access)),
            &ElectionSchema::new(&access),
            state.blockchain(),
            |schema| schema.elections().get(&query.key),
            |schema| schema.elections().get_proof(query.key),
            |schema| schema.election_history(query.key),
        ))
    }

    pub fn all_elections(state: &ServiceApiState, _: ()) -> api::Result<Vec<Election>> {
        Ok(ElectionSchema::new(&state.snapshot())
            .elections()
            .values()
            .collect())
    }

    pub fn active_elections(
        state: &ServiceApiState,
        query: KeyQuery<PublicKey>,
    ) -> api::Result<Vec<Election>> {
        ElectionSchema::new(&state.snapshot())
            .active_elections(&query.key)
            .ok_or_else(|| api::Error::NotFound("\"Administration not found\"".to_owned()))
    }

    pub fn election_results(
        state: &ServiceApiState,
        query: KeyQuery<i64>,
    ) -> api::Result<HashMap<i32, u32>> {
        ElectionSchema::new(&state.snapshot())
            .election_results(query.key)
            .ok_or_else(|| api::Error::NotFound("\"Election not found\"".to_owned()))
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
