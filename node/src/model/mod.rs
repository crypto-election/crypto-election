//! Election data models

pub mod transactions;

pub mod public_api;

pub mod geo;

pub mod wrappers;

use chrono::{DateTime, Utc};

use exonum::{crypto::Hash, runtime::CallerAddress as Address};

use exonum_derive::{BinaryValue, ObjectHash};
use exonum_proto::ProtobufConvert;

use wrappers::OptionalContainer;

use crate::proto;

pub type ParticipantAddress = Address;

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Participant", serde_pb_convert)]
pub struct Participant {
    /// `Address` of participant.
    pub addr: ParticipantAddress,
    /// Name of participant.
    pub name: String,
    /// Email of participant.
    pub email: String,
    /// Personal phone number of participant.
    pub phone_number: String,
    /// Pass code of participant.
    pub pass_code: String,
    /// `Administration` pub_key, where participant is resident.
    /// *Optional*.
    pub residence: OptionalContainer<AdministrationAddress>,
    /// Length of the transactions history.
    pub history_len: u64,
    /// `Hash` of the transaction history.
    pub history_hash: Hash,
}

pub type AdministrationAddress = Address;

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Administration", serde_pb_convert)]
pub struct Administration {
    /// `Address` of the administration.
    pub addr: AdministrationAddress,
    pub name: String,
    pub principal_key: OptionalContainer<AdministrationAddress>,
    pub area: geo::Polygon,
    pub administration_level: u32,
    pub history_len: u64,
    pub history_hash: Hash,
}

pub type ElectionAddress = i64;

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Election", serde_pb_convert)]
pub struct Election {
    pub addr: ElectionAddress,
    pub issuer: AdministrationAddress,
    pub name: String,
    pub is_cancelled: bool,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub options: Vec<ElectionOption>,
    pub history_len: u64,
    pub history_hash: Hash,
}

pub type ElectionOptionAddress = i32;

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::ElectionOption", serde_pb_convert)]
pub struct ElectionOption {
    pub id: ElectionOptionAddress,
    pub title: String,
}

impl Participant {
    /// Create a new `Participant`.
    pub fn from_transaction(
        &addr: &ParticipantAddress,
        transaction: transactions::CreateParticipant,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            addr,
            name: transaction.name,
            email: transaction.email,
            phone_number: transaction.phone_number,
            pass_code: transaction.pass_code,
            residence: transaction.residence,
            history_len,
            history_hash: *history_hash,
        }
    }
    // Todo: Add methods for modification participant objects
}

impl Administration {
    pub fn new(
        &addr: &Address,
        name: &str,
        principal_key: &Option<AdministrationAddress>,
        area: &geo::Polygon,
        administration_level: u32,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            addr,
            name: name.to_owned(),
            principal_key: principal_key.to_owned().into(),
            area: area.clone(),
            administration_level,
            history_len,
            history_hash: *history_hash,
        }
    }
}

impl Election {
    pub fn new(
        addr: ElectionAddress,
        issuer: &AdministrationAddress,
        name: &str,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &Vec<ElectionOption>,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            addr,
            issuer: *issuer,
            name: name.to_owned(),
            start_date: *start_date,
            finish_date: *finish_date,
            options: options.clone(),
            is_cancelled: false,
            history_len,
            history_hash: *history_hash,
        }
    }

    pub fn is_active(&self, moment: DateTime<Utc>) -> bool {
        !self.is_cancelled && self.start_date <= moment && self.finish_date > moment
    }

    pub fn not_started_yet(&self, moment: DateTime<Utc>) -> bool {
        !self.is_cancelled && self.start_date > moment
    }
}
