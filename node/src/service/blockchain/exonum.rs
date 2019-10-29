use crate::constant;

use exonum::{
    blockchain::{self, Transaction, TransactionSet},
    crypto::Hash,
    messages::RawTransaction,
};

use exonum_merkledb::Snapshot;

pub struct Service;

impl blockchain::Service for Service {
    fn service_id(&self) -> u16 {
        constant::BLOCKCHAIN_SERVICE_ID
    }

    fn service_name(&self) -> &str {
        constant::BLOCKCHAIN_SERVICE_NAME
    }

    fn state_hash(&self, snapshot: &dyn Snapshot) -> Vec<Hash> {
        unimplemented!()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        unimplemented!()
    }
}
