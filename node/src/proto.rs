#![allow(bare_trait_objects, renamed_and_removed_lints)]

pub use self::{
    db_models::{Administration, Election, ElectionOption, Participant},
    geo::{Coordinate, LineString, Polygon},
    transactions::{CreateAdministration, CreateParticipant, IssueElection, SubmitLocation, Vote},
    wrappers::{OptionalHash, VecI64Wrap},
};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::crypto::proto::*;
