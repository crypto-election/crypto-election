use crate::{
    model::{Administration, AdministrationAddress},
    schema::Repository,
};
use exonum_merkledb::{access::Access, RawProofMapIndex};

#[derive(Debug)]
pub struct AdministrationRepository<'a, T: Access> {
    administrations: &'a RawProofMapIndex<T::Base, AdministrationAddress, Administration>,
}

impl<'a, T: Access> AdministrationRepository<'a, T> {
    pub(super) fn new(
        administrations: &'a RawProofMapIndex<T::Base, AdministrationAddress, Administration>,
    ) -> Self {
        Self { administrations }
    }
}

impl<T: Access> Repository<AdministrationAddress, Administration>
    for AdministrationRepository<'_, T>
{
    fn has(&self, key: &AdministrationAddress) -> bool {
        self.administrations.contains(key)
    }

    fn get(&self, key: &AdministrationAddress) -> Option<Administration> {
        self.administrations.get(key)
    }

    fn require(&self, key: &AdministrationAddress) -> Administration {
        self.get(key).expect("Unable to get participant")
    }
}
