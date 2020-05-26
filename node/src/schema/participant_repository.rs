use crate::{
    model::{Participant, ParticipantAddress},
    schema::Repository,
};
use exonum_merkledb::{access::Access, RawProofMapIndex};

#[derive(Debug)]
pub struct ParticipantRepository<'a, T: Access> {
    participants: &'a RawProofMapIndex<T::Base, ParticipantAddress, Participant>,
}

impl<'a, T: Access> ParticipantRepository<'a, T> {
    pub(super) fn new(
        participants: &'a RawProofMapIndex<T::Base, ParticipantAddress, Participant>,
    ) -> Self {
        Self { participants }
    }
}

impl<T: Access> Repository<ParticipantAddress, Participant> for ParticipantRepository<'_, T> {
    fn has(&self, key: &ParticipantAddress) -> bool {
        self.participants.contains(key)
    }

    fn get(&self, key: &ParticipantAddress) -> Option<Participant> {
        self.participants.get(key)
    }

    fn require(&self, key: &ParticipantAddress) -> Participant {
        self.get(key).expect("Unable to get participant")
    }
}
