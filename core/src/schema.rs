use std::collections::HashMap;

use chrono::{DateTime, Utc};
use time::Duration;

use exonum::{
    crypto::{Hash, PublicKey},
    exonum_merkledb::{IndexAccess, ObjectHash, ProofListIndex, ProofMapIndex},
};
use exonum_time::schema::TimeSchema;

use crate::model::wrappers::OptionalPubKeyWrap;
use crate::{constant::BLOCKCHAIN_SERVICE_NAME as SERVICE_NAME, model::*};

#[derive(Debug)]
pub struct ElectionSchema<T> {
    access: T,
}

#[derive(Debug)]
struct PrincipalIterator<T>
where
    T: IndexAccess,
{
    index: ProofMapIndex<T, PublicKey, Administration>,
    key: Option<PublicKey>,
}

impl<T> Iterator for PrincipalIterator<T>
where
    T: IndexAccess,
{
    type Item = Administration;

    fn next(&mut self) -> Option<Self::Item> {
        self.key.map(|key| {
            let principal = self
                .index
                .get(&key)
                .expect("Unable to find administration by public key.");
            self.key = principal.principal_key.0;
            principal
        })
    }
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
            self.elections().object_hash(),
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
            Participant::new(
                key,
                name,
                email,
                phone_number,
                pass_code,
                history.len(),
                &history_hash,
            )
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

    pub fn create_administration(
        &mut self,
        key: &PublicKey,
        name: &str,
        principal: &OptionalPubKeyWrap,
        transaction: &Hash,
    ) {
        let administration = {
            let mut history = self.administration_history(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            Administration::new(key, name, principal, history.len(), &history_hash)
        };
        self.administrations().put(key, administration);

        let election_id_wrapper = {
            let mut history = self.election_ids_of_administrations_history(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            wrappers::VecI64::new(&[], history.len(), &history_hash)
        };
        self.election_ids_of_administrations()
            .put(key, election_id_wrapper);
    }

    pub fn iter_principals(&self, key: &PublicKey) -> Option<impl Iterator<Item = Administration>> {
        let administrations = self.administrations();
        administrations.get(key).map(|object| PrincipalIterator {
            key: object.principal_key.0,
            index: administrations,
        })
    }

    pub fn iter_principals_from_current(
        &self,
        key: &PublicKey,
    ) -> Option<impl Iterator<Item = Administration>> {
        let administrations = self.administrations();
        administrations.get(key).map(|_| PrincipalIterator {
            key: Some(*key),
            index: administrations,
        })
    }
    //endregion

    //region Elections
    fn election_ids_of_administrations(&self) -> ProofMapIndex<T, PublicKey, wrappers::VecI64> {
        ProofMapIndex::new(
            format!("{}.election_ids_of_administrations", SERVICE_NAME),
            self.access.clone(),
        )
    }

    fn election_ids_of_administrations_history(
        &self,
        pub_key: &PublicKey,
    ) -> ProofListIndex<T, Hash> {
        ProofListIndex::new_in_family(
            format!("{}.election_ids_of_administrations_history", SERVICE_NAME),
            pub_key,
            self.access.clone(),
        )
    }

    pub fn elections(&self) -> ProofMapIndex<T, i64, Election> {
        ProofMapIndex::new(format!("{}.elections", SERVICE_NAME), self.access.clone())
    }

    pub fn election_history(&self, id: i64) -> ProofListIndex<T, Hash> {
        ProofListIndex::new_in_family(
            format!("{}.election_history", SERVICE_NAME),
            &id,
            self.access.clone(),
        )
    }

    fn elections_of_administration(
        &self,
        administration_pub_key: &PublicKey,
    ) -> Option<impl Iterator<Item = Election>> {
        self.election_ids_of_administrations()
            .get(administration_pub_key)
            .map(|ids| {
                let elections = self.elections();
                ids.into_iter().filter_map(move |id| elections.get(&id))
            })
    }

    fn get_elections_in_range<'a>(
        &'a self,
        administration_pub_key: &'a PublicKey,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Option<impl Iterator<Item = Election> + 'a> {
        self.iter_principals_from_current(administration_pub_key)
            .map(move |administrations| {
                administrations
                    .flat_map(move |principal| {
                        self.elections_of_administration(&principal.pub_key)
                            .expect("Unable to find elections of administration.")
                    })
                    .filter(move |election| {
                        !election.is_cancelled
                            && election.start_date <= to
                            && election.finish_date > from
                    })
            })
    }

    pub fn active_elections(&self, administration_pub_key: &PublicKey) -> Option<Vec<Election>> {
        self.administration(administration_pub_key).map(|_| {
            let now = TimeSchema::new(self.access.clone())
                .time()
                .get()
                .expect("can not get time");

            self.get_elections_in_range(administration_pub_key, now, now)
                .unwrap()
                .collect()
        })
    }

    pub fn available_elections(&self, administration_pub_key: &PublicKey) -> Option<Vec<Election>> {
        self.administration(administration_pub_key).map(|_| {
            let now = TimeSchema::new(self.access.clone())
                .time()
                .get()
                .expect("can not get time");

            let this_week = now + Duration::weeks(1);

            self.get_elections_in_range(administration_pub_key, now, this_week)
                .unwrap()
                .collect()
        })
    }

    pub fn election_votes(&self, election_id: i64) -> ProofMapIndex<T, PublicKey, i32> {
        ProofMapIndex::new_in_family(
            format!("{}.election_votes", SERVICE_NAME),
            &election_id,
            self.access.clone(),
        )
    }

    pub fn issue_election(
        &mut self,
        name: &str,
        author_key: &PublicKey,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &[String],
        transaction: &Hash,
    ) -> i64 {
        let mut elections = self.elections();

        let index = elections.keys().max().map_or(0, |i| i + 1);

        let election = {
            let mut history = self.election_history(index);
            history.push(*transaction);
            let history_hash = history.object_hash();
            let mut option_counter = 0;
            Election::new(
                index,
                author_key,
                name,
                start_date,
                finish_date,
                &(options
                    .iter()
                    .map(|n| ElectionOption {
                        id: {
                            let cur_idx = option_counter;
                            option_counter += 1;
                            cur_idx
                        },
                        title: n.to_owned(),
                    })
                    .collect()),
                history.len(),
                &history_hash,
            )
        };
        elections.put(&index, election);

        let mut id_map = self.election_ids_of_administrations();
        let election_id_collection = {
            let mut history = self.election_ids_of_administrations_history(author_key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            id_map
                .get(author_key)
                .unwrap()
                .append(index, history.len(), &history_hash)
        };
        id_map.put(author_key, election_id_collection);

        index
    }

    pub fn vote(
        &mut self,
        election_id: i64,
        participant_key: &PublicKey,
        option_id: i32,
        transaction: &Hash,
    ) {
        let mut history = self.election_history(election_id);
        history.push(*transaction);
        let history_hash = history.object_hash();
        let old_election = self.elections().get(&election_id).unwrap();
        self.elections().put(
            &election_id,
            Election {
                history_len: old_election.history_len + 1,
                history_hash,
                ..old_election
            },
        );
        self.election_votes(election_id)
            .put(participant_key, option_id);
    }

    pub fn election_results(&self, election_id: i64) -> Option<HashMap<i32, u32>> {
        self.elections().get(&election_id).map(|e| {
            let mut sum: HashMap<i32, u32> = e.options.iter().map(|o| (o.id, 0)).collect();
            self.election_votes(election_id).iter().for_each(|(_, v)| {
                if let Some(counter) = sum.get_mut(&v) {
                    *counter += 1;
                }
            });
            sum
        })
    }

    //endregion
}
