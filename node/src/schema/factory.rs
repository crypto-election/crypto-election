use exonum_merkledb::access::{Access, FromAccess};

use super::{ParticipantSchemaMut, ParticipantSchemaMutImpl, SchemaImpl};

#[derive(Debug)]
pub struct Factory<T: Access> {
    access: T,
}

impl<T: Access> Factory<T> {
    pub fn new(access: T) -> Self {
        Self { access }
    }

    pub(crate) fn participant_schema_mut(&self) -> impl ParticipantSchemaMut {
        ParticipantSchemaMutImpl::from_root(self.access.clone()).unwrap()
    }
}
