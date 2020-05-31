use std::{collections::HashMap, fmt::Debug, iter::FromIterator};

use exonum::runtime::CallerAddress;
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

use crate::{
    model::{public_api::*, AdministrationAddress, Election, ElectionAddress},
    schema::SchemaImpl,
};
use chrono::{DateTime, Utc};

/// Election public web api
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    const MAX_RECURSION_DEPTH: u32 = 64;

    /// Plugs in all Public API methods
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/participants/info", Self::participant_info)
            .endpoint("v1/administrations/info", Self::administration_info)
            .endpoint("v1/administration/tree", Self::administrations_tree)
            .endpoint("v1/elections/info", Self::election_info)
            .endpoint("v1/elections/active", Self::active_elections)
            .endpoint("v1/elections/result", Self::election_results)
            .endpoint("v1/elections/suggested-for", Self::elections_suggested_for);
    }

    /// Gets complete participant info
    ///
    /// ## API address
    /// `v1/participants/info`
    pub async fn participant_info(
        state: ServiceApiState,
        query: PubKeyQuery,
    ) -> api::Result<ParticipantInfo> {
        let index_pair = {
            let schema = SchemaImpl::new(state.service_data());
            (schema.public.participants, schema.participant_history)
        };

        ProofedInfo::try_from_indexes(
            &state.data(),
            "participants",
            CallerAddress::from_key(query.key),
            index_pair,
        )
        .map_err(api::Error::internal)
    }

    /// Gets complete administration info
    ///
    /// ## API address
    /// `v1/administrations/info`
    pub async fn administration_info(
        state: ServiceApiState,
        query: PubKeyQuery,
    ) -> api::Result<AdministrationInfo> {
        let index_pair = {
            let schema = SchemaImpl::new(state.service_data());
            (schema.public.administrations, schema.administration_history)
        };
        let key = CallerAddress::from_key(query.key);

        ProofedInfo::try_from_indexes(&state.data(), "administrations", key, index_pair)
            .map_err(api::Error::internal)
    }

    /// Gets complete election info
    ///
    /// ## API address
    /// `v1/elections/info`
    pub async fn election_info(
        state: ServiceApiState,
        query: KeyQuery<ElectionAddress>,
    ) -> api::Result<ElectionInfo> {
        let index_pair = {
            let schema = SchemaImpl::new(state.service_data());
            (schema.public.elections, schema.election_history)
        };

        ProofedInfo::try_from_indexes(&state.data(), "elections", query.key, index_pair)
            .map_err(api::Error::internal)
    }

    #[doc(hidden)]
    pub async fn all_elections(state: ServiceApiState, _: ()) -> api::Result<Vec<Election>> {
        Ok(SchemaImpl::new(state.service_data())
            .public
            .elections
            .values()
            .collect())
    }

    pub async fn active_elections(
        state: ServiceApiState,
        query: KeyQuery<AdministrationAddress>,
    ) -> api::Result<Vec<Election>> {
        let schema = SchemaImpl::new(state.service_data());

        let now = Self::get_time(&state)?;

        schema
            .public
            .elections_available_at_moment(&query.key, now)
            .map(FromIterator::from_iter)
            .ok_or_else(api::Error::not_found)
    }

    fn get_time(state: &ServiceApiState) -> api::Result<DateTime<Utc>> {
        let schema = SchemaImpl::new(state.service_data());
        schema
            .config
            .get()
            .ok_or_else(|| {
                Box::new(exonum_merkledb::Error::new("Can not read service config"))
                    as Box<dyn failure::Fail>
            })
            .and_then(|config| {
                state
                    .data()
                    .service_schema(config.time_service_name.as_str())
                    .map_err(|e| Box::new(e) as Box<dyn failure::Fail>)
            })
            .and_then(|time_schema: exonum_time::TimeSchema<_>| {
                time_schema.time.get().ok_or_else(|| {
                    Box::new(exonum_merkledb::Error::new("Can not get time"))
                        as Box<dyn failure::Fail>
                })
            })
            .map_err(api::Error::internal)
    }

    pub async fn elections_suggested_for(
        state: ServiceApiState,
        query: PubKeyQuery,
    ) -> api::Result<(Vec<ElectionGroup>, DateTime<Utc>)> {
        let schema = SchemaImpl::new(state.service_data());

        let participant_addr = CallerAddress::from_key(query.key);
        let time = Self::get_time(&state)?;

        let administrations = schema
            .public
            .suggested_administrations_for(&participant_addr, time)
            .ok_or_else(api::Error::not_found)?;

        Ok((
            administrations
                .take(5)
                .map(|administration_addr| ElectionGroup {
                    organization_name: schema
                        .public
                        .administrations
                        .get(&administration_addr)
                        .unwrap()
                        .name,
                    elections: schema
                        .public
                        .elections_available_at_moment(&administration_addr, time)
                        .map(|it| Box::new(it) as Box<dyn Iterator<Item = _>>)
                        .unwrap_or_else(|| Box::new(std::iter::empty()))
                        .map(|election: Election| {
                            if schema.public.voted_yet(&election.addr, &participant_addr) {
                                let results =
                                    schema.public.election_results(&election.addr).unwrap();
                                (election, true, &results).into()
                            } else {
                                election.into()
                            }
                        })
                        .collect(),
                })
                .collect(),
            time,
        ))
    }

    pub async fn administrations_tree(
        state: ServiceApiState,
        _query: (),
    ) -> api::Result<Vec<Node<AdministrationAddress, String>>> {
        let schema = SchemaImpl::new(state.service_data());

        let mut administrations =
            HashMap::<Option<AdministrationAddress>, Vec<(AdministrationAddress, String)>>::new();

        for administration in schema.public.administrations.values() {
            match administrations.get_mut(&administration.principal_key.0) {
                Some(section) => section.push((administration.addr, administration.name)),
                None => {
                    administrations.insert(
                        administration.principal_key.0,
                        vec![(administration.addr, administration.name)],
                    );
                }
            }
        }

        fn map_recursively<'a, I: Iterator<Item = &'a (AdministrationAddress, String)>>(
            iterator: I,
            source: &'a HashMap<
                Option<AdministrationAddress>,
                Vec<(AdministrationAddress, String)>,
            >,
            depth: u32,
        ) -> api::Result<Vec<Node<AdministrationAddress, String>>> {
            if depth > PublicApi::MAX_RECURSION_DEPTH {
                Err(api::Error::internal("To much recursion depth"))
            } else {
                iterator
                    .map(|pair| match source.get(&Some(pair.0.to_owned())) {
                        Some(children) => Ok(Node::WithChildren {
                            key: pair.0.to_owned(),
                            value: pair.1.to_owned(),
                            children: map_recursively(children.iter(), source, depth + 1)?,
                        }),
                        None => Ok(Node::WithoutChildren {
                            key: pair.0.to_owned(),
                            value: pair.1.to_owned(),
                        }),
                    })
                    .collect()
            }
        }

        map_recursively(
            administrations
                .get(&None)
                .map(|it| Box::new(it.into_iter()) as Box<dyn Iterator<Item = _>>)
                .unwrap_or_else(|| Box::new(std::iter::empty())),
            &administrations,
            0,
        )
    }

    pub async fn election_results(
        state: ServiceApiState,
        query: KeyQuery<ElectionAddress>,
    ) -> api::Result<HashMap<i32, u32>> {
        SchemaImpl::new(state.service_data())
            .public
            .election_results(&query.key)
            .ok_or_else(api::Error::not_found)
    }
}
