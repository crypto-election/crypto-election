//! Backend application for election based on blockchain technologies (such as Exonum, Hyperladger).

#![deny(missing_debug_implementations, unsafe_code, bare_trait_objects)] //    missing_docs,

#[macro_use]
extern crate exonum_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum_merkledb;

pub mod service;

pub mod api;

pub mod model;

mod proto;

pub mod constant;

pub mod schema;

mod tx_behavior;

pub mod cli;
