use std::{collections::HashMap, ops::RangeBounds};

use chrono::{DateTime, Duration, Utc};

use exonum::{
    crypto::Hash,
    merkledb::{
        access::{Access, FromAccess, RawAccessMut},
        Entry, Group, ObjectHash, ProofListIndex, ProofMapIndex, RawProofMapIndex,
    },
};
use exonum_derive::{FromAccess, RequireArtifact};

use crate::model::{
    geo,
    transactions::{Config, CreateParticipant},
    wrappers, *,
};

pub use administration_repository::AdministrationRepository;
pub use election_repository::ElectionRepository;
pub use participant_repository::ParticipantRepository;
pub use repository::Repository;

mod iter;

mod administration_repository;
mod election_repository;
mod participant_repository;
mod repository;

binary_value_tuple_impls! {
    #[derive(Debug)]
    pub TupleContainer {
        (DateTime<Utc>, AdministrationAddress),
    }
}

/// Database schema for elections.
#[derive(Debug, FromAccess)]
pub(crate) struct SchemaImpl<T: Access> {
    /// Public part of schema.
    #[from_access(flatten)]
    pub public: Schema<T>,
    /// Configuration
    pub config: Entry<T::Base, Config>,
    /// History for specific participants.
    pub participant_history: Group<T, ParticipantAddress, ProofListIndex<T::Base, Hash>>,
    /// History for specific administrations.
    pub administration_history: Group<T, AdministrationAddress, ProofListIndex<T::Base, Hash>>,
    /// History for specific elections.
    pub election_history: Group<T, ElectionAddress, ProofListIndex<T::Base, Hash>>,
}

type TimePositionInfo = TupleContainer<(DateTime<Utc>, AdministrationAddress)>;
pub(crate) type IndexPair<A, K, V, KeyMode> = (
    ProofMapIndex<<A as Access>::Base, K, V, KeyMode>,
    Group<A, K, ProofListIndex<<A as Access>::Base, Hash>>,
);

#[derive(Debug, FromAccess, RequireArtifact)]
pub struct Schema<T: Access> {
    pub participants: RawProofMapIndex<T::Base, ParticipantAddress, Participant>,
    pub participant_location_history:
        Group<T, ParticipantAddress, ProofListIndex<T::Base, TimePositionInfo>>,
    pub administrations: RawProofMapIndex<T::Base, AdministrationAddress, Administration>,
    pub elections: RawProofMapIndex<T::Base, ElectionAddress, Election>,
    /// Elections of specific administrations.
    pub administration_elections:
        Group<T, AdministrationAddress, ProofListIndex<T::Base, ElectionAddress>>,
    pub election_votes: Group<
        T,
        ElectionAddress,
        RawProofMapIndex<T::Base, ParticipantAddress, ElectionOptionAddress>,
    >,
}

impl<T: Access> SchemaImpl<T> {
    pub fn new(access: T) -> Self {
        Self::from_root(access).unwrap()
    }
}

impl<T: Access> Schema<T> {
    pub fn participant_repository(&self) -> ParticipantRepository<T> {
        ParticipantRepository::new(&self.participants)
    }

    pub fn administration_repository(&self) -> AdministrationRepository<T> {
        AdministrationRepository::new(&self.administrations)
    }

    pub fn election_repository(&self) -> ElectionRepository<T> {
        ElectionRepository::new(&self.elections)
    }

    pub fn iter_principals<'a>(
        &'a self,
        key: &'a AdministrationAddress,
    ) -> Option<impl Iterator<Item = Administration> + 'a> {
        self.administrations
            .get(key)
            .map(|object| iter::PrincipalIterator::<'a, T> {
                key: object.principal_key.0,
                index: &self.administrations,
            })
    }

    pub fn iter_principals_from_current<'a>(
        &'a self,
        key: &AdministrationAddress,
    ) -> Option<impl Iterator<Item = Administration> + 'a> {
        self.administrations
            .get(key)
            .map(|_| iter::PrincipalIterator::<'a, T> {
                key: Some(*key),
                index: &self.administrations,
            })
    }

    /// Selects all elections of administration by given address
    pub fn election_ids_of_administration<'a>(
        &'a self,
        addr: &'a AdministrationAddress,
    ) -> Option<Vec<ElectionAddress>> {
        self.administrations
            .get(addr)
            .map(|_| self.administration_elections.get(addr).iter().collect())
    }

    pub fn elections_of_administration_hierarchically<'a>(
        &'a self,
        addr: &'a AdministrationAddress,
    ) -> Option<impl Iterator<Item = Election> + 'a> {
        self.iter_principals_from_current(addr)
            .map(move |administrations| {
                administrations.flat_map(move |principal| {
                    self.election_ids_of_administration(&principal.addr)
                        .map(|iter| {
                            iter.into_iter()
                                .map(move |id| self.elections.get(&id).unwrap())
                        })
                        .expect("Unable to find elections of administration.")
                })
            })
    }

    pub fn all_elections_of_administration<'a>(
        &'a self,
        addr: &'a AdministrationAddress,
    ) -> Option<impl Iterator<Item = Election> + 'a> {
        self.election_ids_of_administration(addr)
            .map(move |elections| {
                elections
                    .into_iter()
                    .map(move |id| self.elections.get(&id).unwrap())
            })
    }

    fn filter_elections<'a, P>(
        &'a self,
        addr: &'a AdministrationAddress,
        predicate: P,
    ) -> Option<impl Iterator<Item = Election> + 'a>
    where
        P: FnMut(&Election) -> bool + 'a,
    {
        self.all_elections_of_administration(addr)
            .map(|elections| elections.filter(predicate))
    }

    pub fn elections_available_on_time_range<'a>(
        &'a self,
        addr: &'a AdministrationAddress,
        range: impl RangeBounds<DateTime<Utc>> + 'a,
    ) -> Option<impl Iterator<Item = Election> + 'a> {
        self.filter_elections(addr, move |election| {
            !election.is_cancelled
                && (range.contains(&election.start_date) || range.contains(&election.finish_date))
        })
    }

    pub fn elections_available_at_moment<'a>(
        &'a self,
        addr: &'a AdministrationAddress,
        moment: DateTime<Utc>,
    ) -> Option<impl Iterator<Item = Election> + 'a> {
        self.filter_elections(addr, move |election| {
            !election.is_cancelled
                && ((election.start_date..election.finish_date).contains(&moment))
        })
    }

    pub fn voted_yet(&self, election_addr: &ElectionAddress, address: &ParticipantAddress) -> bool {
        self.election_votes
            .get(election_addr)
            .get(address)
            .is_some()
    }

    pub fn available_elections<'a>(
        &'a self,
        address: &'a AdministrationAddress,
        now: DateTime<Utc>,
    ) -> Option<impl Iterator<Item = Election> + 'a> {
        self.administrations.get(address).map(|_| {
            let this_week = now + Duration::weeks(1);

            self.elections_available_on_time_range(address, now..this_week)
                .unwrap()
        })
    }

    pub fn election_results(
        &self,
        election_id: &ElectionAddress,
    ) -> Option<HashMap<ElectionOptionAddress, u32>> {
        self.elections.get(election_id).map(|e| {
            let mut sum: HashMap<ElectionOptionAddress, u32> =
                e.options.iter().map(|o| (o.id, 0)).collect();
            self.election_votes
                .get(election_id)
                .iter()
                .for_each(|(_, v)| {
                    if let Some(counter) = sum.get_mut(&v) {
                        *counter += 1;
                    }
                });
            sum
        })
    }

    pub fn suggested_administrations_for<'a>(
        &'a self,
        participant_addr: &'a ParticipantAddress,
        now: DateTime<Utc>,
    ) -> Option<impl Iterator<Item = AdministrationAddress> + 'a> {
        self.participants.get(participant_addr).map(|participant| {
            let month_ago = now - Duration::days(28);
            let locations = {
                let locations_for_last_month: Vec<_> = self
                    .participant_location_history
                    .get(&participant.addr)
                    .iter()
                    .skip_while(|loc| (loc.0).0 < month_ago)
                    .map(|loc| (loc.0).1)
                    .collect();
                let last_locations = locations_for_last_month.into_iter().rev().take(30);

                let root_administration_addresses = self
                    .administrations
                    .values()
                    .filter(|adm| adm.principal_key.0.is_none())
                    .map(|adm| adm.addr);

                last_locations
                    .chain(participant.residence.0)
                    .chain(root_administration_addresses)
            };

            let mut picked_administrations: HashMap<AdministrationAddress, u32> = HashMap::new();

            for addr in locations {
                match picked_administrations.get_mut(&addr) {
                    Some(counter) => *counter += 1,
                    None => {
                        picked_administrations.insert(addr, 1);
                    }
                }
            }

            let mut rated_administrations = Vec::with_capacity(picked_administrations.len());
            rated_administrations.extend(picked_administrations.into_iter());
            rated_administrations.sort_by(|(_, rank1), (_, rank2)| rank1.cmp(rank2));
            rated_administrations.into_iter().map(|(adm, _)| adm)
        })
    }
}

impl<T> SchemaImpl<T>
where
    T: Access,
    T::Base: RawAccessMut,
{
    //#region Participants
    pub fn create_participant(
        &mut self,
        key: &ParticipantAddress,
        participant: CreateParticipant,
        transaction: &Hash,
    ) {
        let participant = {
            let mut history = self.participant_history.get(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            Participant::from_transaction(key, participant, history.len(), &history_hash)
        };
        self.public.participants.put(key, participant);
    }

    pub fn submit_participant_location(
        &mut self,
        participant_addr: &ParticipantAddress,
        date: DateTime<Utc>,
        &location: &AdministrationAddress,
        transaction: &Hash,
    ) {
        self.public
            .participant_location_history
            .get(participant_addr)
            .push((date, location).into());
        let participant = {
            let mut history = self.participant_history.get(&participant_addr);
            history.push(*transaction);

            let history_hash = history.object_hash();
            let participant = self.public.participants.get(&participant_addr).unwrap();
            Participant {
                history_len: history.len(),
                history_hash,
                ..participant
            }
        };
        self.public.participants.put(&participant_addr, participant);
    }
    //endregion

    //#region Administrations
    pub fn create_administration(
        &mut self,
        addr: &AdministrationAddress,
        name: &str,
        principal: &wrappers::OptionalContainer<AdministrationAddress>,
        area: &geo::Polygon,
        transaction: &Hash,
    ) {
        let administration = {
            let mut history = self.administration_history.get(addr);
            history.push(*transaction);
            let history_hash = history.object_hash();
            let administration_level = principal
                .0
                .map(|addr| {
                    self.public
                        .administrations
                        .get(&addr)
                        .unwrap()
                        .administration_level
                        + 1
                })
                .unwrap_or(0);

            Administration::new(
                addr,
                name,
                &principal.0,
                area,
                administration_level,
                history.len(),
                &history_hash,
            )
        };
        self.public.administrations.put(addr, administration);
    }
    //endregion

    //#region Elections
    pub fn issue_election(
        &mut self,
        name: &str,
        election_address: ElectionAddress,
        author_key: &AdministrationAddress,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &[String],
        transaction: &Hash,
    ) {
        let election = {
            let mut history = self.election_history.get(&election_address);
            history.push(*transaction);
            let history_hash = history.object_hash();

            let options: Vec<ElectionOption> = options
                .iter()
                .scan(0, |counter, t| {
                    *counter += 1;
                    Some(ElectionOption {
                        id: *counter,
                        title: t.to_owned(),
                    })
                })
                .collect();

            Election {
                addr: election_address,
                name: name.to_owned(),
                issuer: *author_key,
                start_date: *start_date,
                finish_date: *finish_date,
                options,
                history_len: history.len(),
                history_hash,
                is_cancelled: false,
            }
        };

        self.public.elections.put(&election_address, election);

        self.public
            .administration_elections
            .get(author_key)
            .push(election_address);
    }

    pub fn vote(
        &mut self,
        election_id: ElectionAddress,
        participant_key: &ParticipantAddress,
        option_id: i32,
        transaction: &Hash,
    ) {
        let mut history = self.election_history.get(&election_id);
        history.push(*transaction);
        let history_hash = history.object_hash();
        let old_election = self.public.elections.get(&election_id).unwrap();
        self.public.elections.put(
            &election_id,
            Election {
                history_len: old_election.history_len + 1,
                history_hash,
                ..old_election
            },
        );
        self.public
            .election_votes
            .get(&election_id)
            .put(participant_key, option_id);
    }
    //endregion
}
