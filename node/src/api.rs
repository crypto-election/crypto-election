use core::{
    constant::BLOCKCHAIN_SERVICE_ID,
    model::{Administration, Participant},
    schema::ElectionSchema,
};
use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
};

use exonum_merkledb::{ListProof, MapProof};

#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantInfo {
    pub block_proof: BlockProof,
    pub participant_proof: ParticipantProof,
    pub participant_history: Option<ParticipantHistory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantProof {
    pub to_table: MapProof<Hash, Hash>,
    pub to_participant: MapProof<PublicKey, Participant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantHistory {
    pub proof: ListProof<Hash>,
    pub transactions: Vec<TransactionMessage>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ParticipantQuery {
    /// Public key of the queried wallet.
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdministrationInfo {
    pub block_proof: BlockProof,
    pub administration_proof: AdministrationProof,
    pub administration_history: Option<AdministrationHistory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdministrationProof {
    pub to_table: MapProof<Hash, Hash>,
    pub to_administration: MapProof<PublicKey, Administration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdministrationHistory {
    pub proof: ListProof<Hash>,
    pub transactions: Vec<TransactionMessage>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct AdministrationQuery {
    /// Public key of the queried wallet.
    pub pub_key: PublicKey,
}

impl PublicApi {
    pub fn participant_info(
        state: &ServiceApiState,
        query: ParticipantQuery,
    ) -> api::Result<ParticipantInfo> {
        let snapshot = state.snapshot();

        let general_schema = blockchain::Schema::new(&snapshot);
        let election_schema = ElectionSchema::new(&snapshot);

        let max_height = general_schema.block_hashes_by_height().len() - 1;

        let block_proof = general_schema
            .block_and_precommits(Height(max_height))
            .unwrap();

        let to_table: MapProof<Hash, Hash> =
            general_schema.get_proof_to_service_table(BLOCKCHAIN_SERVICE_ID, 0);

        let to_participant: MapProof<PublicKey, Participant> =
            election_schema.participants().get_proof(query.pub_key);

        let participant_proof = ParticipantProof {
            to_table,
            to_participant,
        };

        let participant = election_schema.participant(&query.pub_key);

        let participant_history = participant.map(|_| {
            let history = election_schema.participant_history(&query.pub_key);
            let proof = history.get_range_proof(0..history.len());
            let explorer = BlockchainExplorer::new(state.blockchain());

            let transactions = history
                .iter()
                .map(|record| explorer.transaction_without_proof(&record).unwrap())
                .collect::<Vec<_>>();

            ParticipantHistory {
                proof,
                transactions,
            }
        });

        Ok(ParticipantInfo {
            block_proof,
            participant_proof,
            participant_history,
        })
    }

    pub fn administration_info(
        state: &ServiceApiState,
        query: AdministrationQuery,
    ) -> api::Result<AdministrationInfo> {
        let snapshot = state.snapshot();

        let general_schema = blockchain::Schema::new(&snapshot);
        let election_schema = ElectionSchema::new(&snapshot);

        let max_height = general_schema.block_hashes_by_height().len() - 1;

        let block_proof = general_schema
            .block_and_precommits(Height(max_height))
            .unwrap();

        let to_table: MapProof<Hash, Hash> =
            general_schema.get_proof_to_service_table(BLOCKCHAIN_SERVICE_ID, 0);

        let to_administration: MapProof<PublicKey, Administration> =
            election_schema.administrations().get_proof(query.pub_key);

        let administration_proof = AdministrationProof {
            to_table,
            to_administration,
        };

        let administration = election_schema.participant(&query.pub_key);

        let administration_history = administration.map(|_| {
            let history = election_schema.administration_history(&query.pub_key);
            let proof = history.get_range_proof(0..history.len());
            let explorer = BlockchainExplorer::new(state.blockchain());

            let transactions = history
                .iter()
                .map(|record| explorer.transaction_without_proof(&record).unwrap())
                .collect::<Vec<_>>();

            AdministrationHistory {
                proof,
                transactions,
            }
        });

        Ok(AdministrationInfo {
            block_proof,
            administration_proof,
            administration_history,
        })
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/participants/info", Self::participant_info)
            .endpoint("v1/administrations/info", Self::administration_info);
    }
}
