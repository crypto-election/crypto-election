//! Election data models

pub mod transactions;

pub mod public_api;

pub mod geo;

pub mod wrappers;

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use exonum::crypto::{Hash, PublicKey};

use wrappers::OptionalPubKeyWrap;

use crate::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Participant")]
pub struct Participant {
    /// `PublicKey` of participant.
    pub pub_key: PublicKey,
    /// Name of participant.
    pub name: String,
    /// Email of participant.
    pub email: String,
    /// Personal phone number of participant.
    pub phone_number: String,
    /// Pass code of participant.
    pub pass_code: String,
    /// `Administration` pub_key, where participanti is resident.
    pub residence: OptionalPubKeyWrap,
    /// Length of the transactions history.
    pub history_len: u64,
    /// `Hash` of the transaction history.
    pub history_hash: Hash,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Administration")]
pub struct Administration {
    /// `PublicKey` of the administration.
    pub pub_key: PublicKey,
    pub name: String,
    pub principal_key: OptionalPubKeyWrap,
    pub area: geo::Polygon,
    pub administration_level: u32,
    pub history_len: u64,
    pub history_hash: Hash,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Election")]
pub struct Election {
    pub id: i64,
    pub author_key: PublicKey,
    pub name: String,
    pub is_cancelled: bool,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub options: Vec<ElectionOption>,
    pub history_len: u64,
    pub history_hash: Hash,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::ElectionOption")]
pub struct ElectionOption {
    pub id: i32,
    pub title: String,
}

impl Participant {
    /// Create a new `Participant`.
    pub fn new(
        &pub_key: &PublicKey,
        name: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
        residence: &Option<PublicKey>,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
            pass_code: pass_code.to_owned(),
            residence: OptionalPubKeyWrap(residence.clone()),
            history_len,
            history_hash: *history_hash,
        }
    }
    // Todo: Add methods for modification participant objects
}

impl Administration {
    pub fn new(
        &pub_key: &PublicKey,
        name: &str,
        principal_key: &OptionalPubKeyWrap,
        area: &geo::Polygon,
        administration_level: u32,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            principal_key: *principal_key,
            area: area.clone(),
            administration_level,
            history_len,
            history_hash: *history_hash,
        }
    }
}

impl Election {
    pub fn new(
        id: i64,
        author_key: &PublicKey,
        name: &str,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &Vec<ElectionOption>,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            id,
            author_key: *author_key,
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
