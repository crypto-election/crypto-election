//! Backend application for election based on blockchain technologies (such as Exonum, Hyperladger).

#![deny(
    missing_debug_implementations,
    missing_docs,
    unsafe_code,
    bare_trait_objects 
)]

#[macro_use]
extern crate exonum_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

use exonum_merkledb::Snapshot;

pub mod service;
