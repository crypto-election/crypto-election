use std::{collections::HashMap, ops::RangeBounds};

use chrono::{DateTime, Utc};
use time::Duration;

use exonum::{
    crypto::{Hash, PublicKey},
    merkledb::{
        access::{Access, FromAccess, RawAccessMut},
        Group, ObjectHash, ProofListIndex, RawProofMapIndex,
    },
    runtime::CallerAddress as Address,
};
use exonum_derive::{FromAccess, RequireArtifact};
use exonum_time::TimeSchema;

use crate::{
    constant::BLOCKCHAIN_SERVICE_NAME as SERVICE_NAME, model::wrappers::OptionalContainer, model::*,
};

mod iter;

binary_value_tuple_impls! {
    TupleContainer {
        (DateTime<Utc>, PublicKey),
    }
}

/// Database schema for elections.
#[derive(Debug, FromAccess)]
pub(crate) struct SchemaImpl<T: Access> {
    /// Public part of schema.
    #[from_access(flatten)]
    pub public: Schema<T>,
    /// History for specific participants.
    pub participant_history: Group<T, ParticipantAddress, ProofListIndex<T::Base, Hash>>,
    /// History for specific administrations.
    pub administration_history: Group<T, AdministrationAddress, ProofListIndex<T::Base, Hash>>,
    /// History for specific elections.
    pub election_history: Group<T, ElectionAddress, ProofListIndex<T::Base, Hash>>,
    /// History for specific elections.
    pub election_history: Group<T, ElectionAddress, ProofListIndex<T::Base, Hash>>,
}

#[derive(Debug, FromAccess, RequireArtifact)]
pub struct Schema<T: Access> {
    pub participants: RawProofMapIndex<T::Base, ParticipantAddress, Participant>,
    pub participant_location_history: Group<
        T,
        ParticipantAddress,
        ProofListIndex<T::Base, TupleContainer<(DateTime<Utc>, PublicKey)>>,
    >,
    pub administrations: RawProofMapIndex<T::Base, AdministrationAddress, Administration>,
    pub elections: RawProofMapIndex<T::Base, ElectionAddress, Election>,
}

impl<T: Access> SchemaImpl<T> {
    pub fn new(access: T) -> Self {
        Self::from_root(access).unwrap()
    }
}

impl<T> SchemaImpl<T>
where
    T: Access,
    T::Base: RawAccessMut,
{
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![
            self.participants.object_hash(),
            self.administrations.object_hash(),
            self.elections.object_hash(),
        ]
    }

    //#region Participants
    pub fn create_participant(
        &mut self,
        key: &ParticipantAddress,
        name: &str,
        email: &str,
        phone_number: &str,
        residence: &Option<AdministrationAddress>,
        pass_code: &str,
        transaction: &Hash,
    ) {
        let participant = {
            let mut history = self.participant_history.get(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            Participant::new(
                key,
                name,
                email,
                phone_number,
                pass_code,
                residence,
                history.len(),
                &history_hash,
            )
        };
        self.participants.put(key, participant);
    }

    pub fn submit_paticipant_location(
        &mut self,
        participant: &ParticipantAddress,
        date: DateTime<Utc>,
        &location: &AdministrationAddress,
    ) {
        self.participant_location_history
            .get(participant)
            .push((date, location).into());
    }
    //endregion

    //#region Administrations
    pub fn create_administration(
        &mut self,
        key: &AdministrationAddress,
        name: &str,
        principal: &OptionalContainer<AdministrationAddress>,
        area: &geo::Polygon,
        transaction: &Hash,
    ) {
        let administration = {
            let mut history = self.administration_history.get(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            let administration_level = principal
                .map(|addr| {
                    self.administrations
                        .get(&addr)
                        .unwrap()
                        .administration_level
                        + 1
                })
                .unwrap_or(0);

            Administration::new(
                key,
                name,
                principal,
                area,
                administration_level,
                history.len(),
                &history_hash,
            )
        };
        self.administrations.put(key, administration);

        let election_id_wrapper = {
            let mut history = self.election_ids_of_administrations_history(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            wrappers::VecI64::new(&[], history.len(), &history_hash)
        };
        self.election_ids_of_administrations()
            .put(key, election_id_wrapper);
    }

    pub fn iter_principals(
        &self,
        key: &AdministrationAddress,
    ) -> Option<impl Iterator<Item = Administration>> {
        let administrations = self.administrations();
        administrations
            .get(key)
            .map(|object| iter::PrincipalIterator {
                key: object.principal_key.0,
                index: administrations,
            })
    }

    pub fn iter_principals_from_current(
        &self,
        key: &AdministrationAddress,
    ) -> Option<impl Iterator<Item = Administration>> {
        self.administrations
            .get(key)
            .map(|_| iter::PrincipalIterator {
                key: Some(*key),
                index: self.administrations,
            })
    }
    //endregion

    //#region Elections
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
        range: impl RangeBounds<DateTime<Utc>>,
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
                            && election.start_date <= range.end_bound()
                            && election.finish_date > range.start_bound()
                    })
            })
    }

    pub fn active_elections(
        &self,
        adm_addr: &AdministrationAddress,
    ) -> Option<impl Iterator<Item = Election>> {
        self.administration(adm_addr).map(|_| {
            let now = TimeSchema::new(self.access.clone())
                .time()
                .get()
                .expect("can not get time");

            self.get_elections_in_range(administration_pub_key, now..=now)
                .unwrap()
        })
    }

    pub fn available_elections(
        &self,
        adm_addr: &AdministrationAddress,
    ) -> Option<impl Iterator<Item = Election>> {
        self.administration(adm_addr).map(|_| {
            let now = TimeSchema::new(self.access.clone())
                .time()
                .get()
                .expect("can not get time");

            let this_week = now + Duration::weeks(1);

            self.get_elections_in_range(administration_pub_key, now..this_week)
                .unwrap()
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
            Election::new(
                index,
                author_key,
                name,
                start_date,
                finish_date,
                &(options
                    .iter()
                    .scan(0, |counter, t| {
                        Some(ElectionOption {
                            id: {
                                let cur_idx = *counter;
                                *counter += 1;
                                cur_idx
                            },
                            title: y.to_owned(),
                        })
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
