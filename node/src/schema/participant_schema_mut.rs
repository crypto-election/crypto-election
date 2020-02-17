use chrono::{DateTime, Utc};

use exonum_crypto::Hash;
use exonum_derive::FromAccess;
use exonum_merkledb::{
    access::{Access, FromAccess},
    ObjectHash,
};

use super::SchemaImpl;
use crate::model::{AdministrationAddress, Participant, ParticipantAddress};
use exonum::runtime::CallerAddress;

pub trait ParticipantSchemaMut {
    fn create(
        &mut self,
        key: &ParticipantAddress,
        name: &str,
        email: &str,
        phone_number: &str,
        residence: &Option<AdministrationAddress>,
        pass_code: &str,
        transaction: &Hash,
    );

    fn submit_location(
        &mut self,
        participant: &ParticipantAddress,
        date: DateTime<Utc>,
        location: &AdministrationAddress,
    );
}

#[derive(FromAccess)]
pub struct ParticipantSchemaMutImpl<T: Access> {
    #[from_access(flatten)]
    schema: SchemaImpl<T>,
}

impl<T: Access> ParticipantSchemaMut for ParticipantSchemaMutImpl<T> {
    fn create(
        &mut self,
        key: &ParticipantAddress,
        name: &str,
        email: &str,
        phone_number: &str,
        residence: &Option<AdministrationAddress>,
        pass_code: &str,
        transaction: &Hash,
    ) {
        let participant = {
            let mut history = self.schema.participant_history.get(key);
            history.push(*transaction);
            let history_hash = history.object_hash();
            Participant::new(
                key,
                name,
                email,
                phone_number,
                pass_code,
                residence,
                history.len(),
                &history_hash,
            )
        };
        self.schema.public.participants.put(key, participant);
    }

    fn submit_location(
        &mut self,
        participant: &ParticipantAddress,
        date: DateTime<Utc>,
        location: &AdministrationAddress,
    ) {
        self.schema
            .public
            .participant_location_history
            .get(participant)
            .push((date, location).into());
    }
}
