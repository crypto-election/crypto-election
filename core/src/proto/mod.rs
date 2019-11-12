#![allow(bare_trait_objects, renamed_and_removed_lints)]

pub use self::election::{
    Administration, CoordinateDef, CreateAdministration, CreateParticipant, Election,
    ElectionOption, IssueElection, LineStringDef, Participant, PolygonDef, VecI64Wrap,
};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::proto::schema::*;
