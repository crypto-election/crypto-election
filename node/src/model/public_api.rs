use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use exonum::{
    blockchain::BlockProof,
    crypto::{Hash, PublicKey},
    messages::{AnyTx, Verified},
};
use exonum_merkledb::{proof_map::Raw, ListProof, MapProof};

use super::{Administration, AdministrationAddress, Election, Participant, ParticipantAddress};
use exonum_merkledb::indexes::proof_map::Hashed;

pub type ParticipantInfo = Info<ParticipantAddress, Participant, RawKeyModeWrapper>;
pub type AdministrationInfo = Info<AdministrationAddress, Administration, RawKeyModeWrapper>;
pub type ElectionInfo = Info<i64, Election, HashedKeyModeWrapper>;

/// KeyMode type container. Used for serializable storing KeyMode type.
pub trait KeyModeWrapper: Serialize {
    type KeyMode;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawKeyModeWrapper;

impl KeyModeWrapper for RawKeyModeWrapper {
    type KeyMode = Raw;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashedKeyModeWrapper;

impl KeyModeWrapper for HashedKeyModeWrapper {
    type KeyMode = Hashed;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info<K, E: Debug, KMW: KeyModeWrapper + Debug>
where
    KMW::KeyMode: Debug,
{
    pub block_proof: BlockProof,
    pub object_proof: Proof<K, E, KMW>,
    pub history: Option<History>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof<K, E, KMW: KeyModeWrapper>
where
    E: Debug,
{
    pub to_table: MapProof<String, Hash>,
    pub to_object: MapProof<K, E, KMW::KeyMode>,
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
