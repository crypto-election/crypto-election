#![allow(bare_trait_objects, renamed_and_removed_lints)]

pub use self::election::User;

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::proto::schema::*;
