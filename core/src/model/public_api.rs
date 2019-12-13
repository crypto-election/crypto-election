use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use exonum::{
    blockchain::{BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    exonum_merkledb::{ListProof, MapProof},
};

use super::{Administration, Election, Participant};

pub type ParticipantInfo = Info<PublicKey, Participant>;
pub type AdministrationInfo = Info<PublicKey, Administration>;
pub type ElectionInfo = Info<i64, Election>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info<K, E>
where
    E: Debug,
{
    pub block_proof: BlockProof,
    pub object_proof: Proof<K, E>,
    pub history: Option<History>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof<K, E>
where
    E: Debug,
{
    pub to_table: MapProof<Hash, Hash>,
    pub to_object: MapProof<K, E>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub proof: ListProof<Hash>,
    pub transactions: Vec<TransactionMessage>,
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
