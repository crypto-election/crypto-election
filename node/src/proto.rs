#![allow(bare_trait_objects)]

pub use self::{db_models::*, geo::*, transactions::*, wrappers::*};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::crypto::proto::*;
