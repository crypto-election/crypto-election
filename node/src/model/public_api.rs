use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use exonum::{
    blockchain::BlockProof,
    crypto::{Hash, PublicKey},
    messages::{AnyTx, Verified},
};
use exonum_merkledb::{
    proof_map::{Hashed, Raw},
    ListProof, MapProof,
};

use super::{Administration, AdministrationAddress, Election, Participant, ParticipantAddress};

pub type ParticipantInfo = Info<ParticipantAddress, Participant>;
pub type AdministrationInfo = Info<AdministrationAddress, Administration>;
pub type ElectionInfo = HashedInfo<i64, Election>;

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
    pub to_table: MapProof<String, Hash>,
    pub to_object: MapProof<K, E, Raw>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashedInfo<K, E>
where
    E: Debug,
{
    pub block_proof: BlockProof,
    pub object_proof: HashedProof<K, E>,
    pub history: Option<History>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashedProof<K, E>
where
    E: Debug,
{
    pub to_table: MapProof<String, Hash>,
    pub to_object: MapProof<K, E>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub proof: ListProof<Hash>,
    pub transactions: Vec<Verified<AnyTx>>,
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
