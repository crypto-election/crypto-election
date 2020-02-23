use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use exonum::{
    blockchain::BlockProof,
    crypto::{Hash, PublicKey},
    messages::{AnyTx, Verified},
};
use exonum_merkledb::{
    BinaryKey, BinaryValue, Group, ListProof, MapProof, ProofListIndex, Snapshot,
};

use super::{
    wrappers::{HashedKeyModeWrapper, RawKeyModeWrapper, TypeWrapper},
    Administration, AdministrationAddress, Election, Participant, ParticipantAddress,
};
use crate::schema::IndexPair;
use exonum::blockchain::IndexProof;
use exonum::runtime::BlockchainData;
use exonum_merkledb::access::Access;
use exonum_merkledb::proof_map::ToProofPath;

pub type ParticipantInfo = ProofedInfo<ParticipantAddress, Participant, RawKeyModeWrapper>;
pub type AdministrationInfo = ProofedInfo<AdministrationAddress, Administration, RawKeyModeWrapper>;
pub type ElectionInfo = ProofedInfo<i64, Election, HashedKeyModeWrapper>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofedInfo<K, V: Debug, KeyMode: TypeWrapper + Debug>
where
    KeyMode::Type: Debug,
{
    pub block_proof: BlockProof,
    pub object_proof: Proof<K, V, KeyMode>,
    pub history: Option<History>,
}

impl<K: ToOwned + BinaryKey, V: Debug + BinaryValue, KeyMode: TypeWrapper + Debug>
    ProofedInfo<K, V, KeyMode>
where
    KeyMode::Type: Debug,
{
    pub(crate) fn try_from_indexes<A: Access>(
        blockchain_data: &BlockchainData<&dyn Snapshot>,
        idx_name: &str,
        key: K,
        index_pair: IndexPair<A, K, V, KeyMode::Type>,
    ) -> Result<ProofedInfo<K::Owned, V, KeyMode>, exonum_merkledb::Error>
    where
        KeyMode::Type: ToProofPath<K>,
    {
        let IndexProof {
            block_proof,
            index_proof,
            ..
        } = blockchain_data
            .proof_for_service_index(idx_name)
            .ok_or_else(|| {
                exonum_merkledb::Error::new(format!("No such index with name '{}'", idx_name))
            })?;

        let (object_index, history_index) = index_pair;

        Ok(ProofedInfo {
            block_proof,
            object_proof: Proof::new(index_proof, object_index.get_proof(key.to_owned())),
            history: object_index
                .get(&key)
                .map(|_| History::from_indexes(blockchain_data, history_index, key)),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof<K, V: Debug, KeyMode: TypeWrapper> {
    pub to_table: MapProof<String, Hash>,
    pub to_object: MapProof<K, V, KeyMode::Type>,
}

impl<K, V: Debug, KeyMode: TypeWrapper> Proof<K, V, KeyMode> {
    pub fn new(
        to_table: MapProof<String, Hash>,
        to_object: MapProof<K, V, KeyMode::Type>,
    ) -> Proof<K, V, KeyMode> {
        Self {
            to_table,
            to_object,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub proof: ListProof<Hash>,
    pub transactions: Vec<Verified<AnyTx>>,
}

impl History {
    pub(crate) fn from_indexes<A: Access, K: BinaryKey>(
        blockchain_data: &BlockchainData<&dyn Snapshot>,
        history_index: Group<A, K, ProofListIndex<A::Base, Hash>>,
        key: K,
    ) -> Self {
        let history = history_index.get(&key);
        let proof = history_index.get(&key).get_range_proof(..);

        let transactions = blockchain_data.for_core().transactions();

        let transactions = history
            .iter()
            .map(|record| transactions.get(&record).unwrap())
            .collect();

        Self {
            proof,
            transactions,
        }
    }
}

pub type PubKeyQuery = KeyQuery<PublicKey>;
pub type I64Query = KeyQuery<i64>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct KeyQuery<K>
where
    K: Debug + Clone + Copy,
{
    pub key: K,
}
