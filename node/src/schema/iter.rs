use exonum::{
    merkledb::{access::Access, RawProofMapIndex},
    runtime::CallerAddress as Address,
};

use crate::model::Administration;

#[derive(Debug)]
pub struct PrincipalIterator<T>
where
    T: Access,
{
    index: RawProofMapIndex<T, Address, Administration>,
    key: Option<Address>,
}

impl<T> Iterator for PrincipalIterator<T>
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
