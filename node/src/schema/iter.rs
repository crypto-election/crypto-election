use exonum::merkledb::{access::Access, RawProofMapIndex};

use crate::model::{Administration, AdministrationAddress};

#[derive(Debug)]
pub struct PrincipalIterator<'a, T>
where
    T: Access,
{
    pub(super) index: &'a RawProofMapIndex<T::Base, AdministrationAddress, Administration>,
    pub(super) key: Option<AdministrationAddress>,
}

impl<T> Iterator for PrincipalIterator<'_, T>
where
    T: Access,
{
    type Item = Administration;

    fn next(&mut self) -> Option<Self::Item> {
        self.key.map(|key| {
            let principal = self
                .index
                .get(&key)
                .expect("Unable to find administration by public key.");
            self.key = principal.principal_key.0;
            principal
        })
    }
}
