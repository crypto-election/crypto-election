use crate::{
    model::{Election, ElectionAddress},
    schema::Repository,
};
use exonum_merkledb::{access::Access, RawProofMapIndex};

#[derive(Debug)]
pub struct ElectionRepository<'a, T: Access> {
    elections: &'a RawProofMapIndex<T::Base, ElectionAddress, Election>,
}

impl<'a, T: Access> ElectionRepository<'a, T> {
    pub(super) fn new(elections: &'a RawProofMapIndex<T::Base, ElectionAddress, Election>) -> Self {
        Self { elections }
    }
}

impl<T: Access> Repository<ElectionAddress, Election> for ElectionRepository<'_, T> {
    fn has(&self, key: &ElectionAddress) -> bool {
        self.elections.contains(key)
    }

    fn get(&self, key: &ElectionAddress) -> Option<Election> {
        self.elections.get(key)
    }

    fn require(&self, key: &ElectionAddress) -> Election {
        self.get(key).unwrap()
    }
}
