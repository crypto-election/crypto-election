//! Backend application for election based on blockchain technologies (such as Exonum, Hyperladger).

#![deny(missing_debug_implementations, unsafe_code, bare_trait_objects)] //    missing_docs,

//#[macro_use]
extern crate exonum_derive;
#[macro_use]
extern crate failure;
//#[macro_use]
extern crate serde_derive;

extern crate crypto_election_core as core;

pub mod service;

pub mod api;
