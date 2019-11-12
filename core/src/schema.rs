use exonum_merkledb::{
    BinaryValue, IndexAccess, ListIndex, ObjectHash, ProofListIndex, ProofMapIndex,
};

use exonum::crypto::{Hash, PublicKey};

use crate::{constant::BLOCKCHAIN_SERVICE_NAME as SERVICE_NAME, model::*};
use std::hash::Hasher;
use std::iter::Filter;
use std::{borrow::Cow, mem};

#[derive(Debug)]
pub struct ElectionSchema<T> {
    access: T,
}

impl<T> ElectionSchema<T>
where
    T: IndexAccess,
{
    pub fn new(access: T) -> Self {
        Self { access }
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![
            self.participants().object_hash(),
            self.administrations().object_hash(),
        ]
    }

    //region Participants
    pub fn participants(&self) -> ProofMapIndex<T, PublicKey, Participant> {
        ProofMapIndex::new(
            format!("{}.participants", SERVICE_NAME),
            self.access.clone(),
        )
    }

    pub fn participant(&self, pub_key: &PublicKey) -> Option<Participant> {
        self.participants().get(pub_key)
    }

    pub fn participant_history(&self, pub_key: &PublicKey) -> ProofListIndex<T, Hash> {
        ProofListIndex::new_in_family(
            format!("{}.participant_history", SERVICE_NAME),
            pub_key,
            self.access.clone(),
        )
    }

    pub fn create_participant(
        &mut self,
        key: &PublicKey,
        name: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
        transaction: &Hash,
    ) {
        let participant = {
            let mut history = self.participant_history(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            Participant::new(key, name, email, phone_number, pass_code)
        };
        self.participants().put(key, participant);
    }
    //endregion

    //region Administrations
    pub fn administrations(&self) -> ProofMapIndex<T, PublicKey, Administration> {
        ProofMapIndex::new(
            format!("{}.administrations", SERVICE_NAME),
            self.access.clone(),
        )
    }

    pub fn administration(&self, pub_key: &PublicKey) -> Option<Administration> {
        self.administrations().get(pub_key)
    }

    pub fn administration_history(&self, pub_key: &PublicKey) -> ProofListIndex<T, Hash> {
        ProofListIndex::new_in_family(
            format!("{}.administration_history", SERVICE_NAME),
            pub_key,
            self.access.clone(),
        )
    }

    pub fn create_administration(&mut self, key: &PublicKey, name: &str, transaction: &Hash) {
        let administration = {
            let mut history = self.administration_history(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            Administration::new(key, name)
        };
        self.administrations().put(key, administration);
    }
    //endregion

    fn election_ids_of_administrations(&self) -> ProofMapIndex<T, PublicKey, VecI64Wrap> {
        ProofMapIndex::new(
            format!("{}.election_ids_of_administrations", SERVICE_NAME),
            self.access.clone(),
        )
    }

    fn elections(&self) -> ProofMapIndex<T, i64, Election> {
        ProofMapIndex::new(format!("{}.elections", SERVICE_NAME), self.access.clone())
    }

    pub fn elections_of_administration(
        &self,
        administration_pub_key: &PublicKey,
    ) -> Option<Vec<Election>> {
        self.election_ids_of_administrations()
            .get(administration_pub_key)
            .and_then(|ids| {
                ids._0
                    .iter()
                    .map(|id| self.elections().get(id))
                    .filter(|el| el.is_some())
                    .collect()
            })
    }
}
