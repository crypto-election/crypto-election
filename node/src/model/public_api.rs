use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Utc};
use exonum::{
    blockchain::BlockProof,
    crypto::{Hash, PublicKey},
    messages::{AnyTx, Verified},
};
use exonum_merkledb::{
    BinaryKey, BinaryValue, Group, ListProof, MapProof, ProofListIndex, Snapshot,
};

use super::{
    wrappers::{RawKeyModeWrapper, TypeWrapper},
    Administration, AdministrationAddress, Election, ElectionAddress, ElectionOptionAddress,
    Participant, ParticipantAddress,
};
use crate::schema::IndexPair;
use exonum::blockchain::IndexProof;
use exonum::runtime::BlockchainData;
use exonum_merkledb::access::Access;
use exonum_merkledb::proof_map::ToProofPath;

pub type ParticipantInfo = ProofedInfo<ParticipantAddress, Participant, RawKeyModeWrapper>;
pub type AdministrationInfo = ProofedInfo<AdministrationAddress, Administration, RawKeyModeWrapper>;
pub type ElectionInfo = ProofedInfo<ElectionAddress, Election, RawKeyModeWrapper>;

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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ElectionGroup {
    pub organization_name: String,
    pub elections: Vec<ElectionConvert>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ElectionConvert {
    pub addr: ElectionAddress,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub options: Vec<ElectionOptionConvert>,
    pub is_cancelled: bool,
    pub is_voted_yet: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ElectionOptionConvert {
    pub id: ElectionOptionAddress,
    pub title: String,
    pub votes_count: Option<u32>,
}

impl ElectionConvert {
    pub fn set_voted(&mut self) {
        self.is_voted_yet = true;
    }

    fn from_metadata(
        election: super::Election,
        is_voted_yet: bool,
        results: Option<&HashMap<ElectionOptionAddress, u32>>,
    ) -> Self {
        let mut options = Vec::with_capacity(election.options.len());

        election
            .options
            .into_iter()
            .map(|option| match results {
                Some(res) => {
                    let result = res.get(&option.id).map(|v| *v).unwrap_or(0);
                    (option, result).into()
                }
                None => option.into(),
            })
            .for_each(|it| options.push(it));

        Self {
            addr: election.addr,
            name: election.name,
            start_date: election.start_date,
            finish_date: election.finish_date,
            options,
            is_cancelled: election.is_cancelled,
            is_voted_yet,
        }
    }
}

impl From<super::Election> for ElectionConvert {
    fn from(from: super::Election) -> Self {
        Self::from_metadata(from, false, None)
    }
}

impl From<(super::Election, bool)> for ElectionConvert {
    fn from(from: (super::Election, bool)) -> Self {
        Self::from_metadata(from.0, from.1, None)
    }
}

impl From<(super::Election, bool, &HashMap<ElectionOptionAddress, u32>)> for ElectionConvert {
    fn from(from: (super::Election, bool, &HashMap<ElectionOptionAddress, u32>)) -> Self {
        Self::from_metadata(from.0, from.1, Some(from.2))
    }
}

impl From<super::ElectionOption> for ElectionOptionConvert {
    fn from(from: super::ElectionOption) -> Self {
        Self {
            id: from.id,
            title: from.title,
            votes_count: None,
        }
    }
}

impl From<(super::ElectionOption, u32)> for ElectionOptionConvert {
    fn from(from: (super::ElectionOption, u32)) -> Self {
        Self {
            votes_count: Some(from.1),
            ..from.0.into()
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Node<K: Clone, V: Clone> {
    WithChildren {
        key: K,
        value: V,
        children: Vec<Node<K, V>>,
    },
    WithoutChildren {
        key: K,
        value: V,
    },
}
