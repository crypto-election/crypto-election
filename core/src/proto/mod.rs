#![allow(bare_trait_objects, renamed_and_removed_lints)]

pub use self::{
    db_models::{Administration, Election, ElectionOption, Participant},
    geo::{Coordinate, LineString, Polygon},
    transactions::{CreateAdministration, CreateParticipant, IssueElection, Vote},
    wrappers::{OptionalPubKey, VecI64Wrap},
};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::proto::schema::*;
