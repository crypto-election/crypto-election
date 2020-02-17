use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use exonum_crypto::Hash;
use exonum_derive::FromAccess;
use exonum_merkledb::{
    access::{Access, FromAccess},
    ObjectHash,
};

use super::Schema;
use crate::model::{AdministrationAddress, Participant, ParticipantAddress};
use exonum::runtime::CallerAddress;
use rust_decimal::prelude::{FromPrimitive, One, Zero};
use std::collections::HashMap;

pub trait ParticipantSchema {
    fn all<'a>(&'a self) -> Box<dyn Iterator<Item = Participant> + 'a>;

    fn location_history_with_weight<'a>(
        &'a self,
        addr: ParticipantAddress,
    ) -> Option<dyn Iterator<Item = (AdministrationAddress, Decimal)> + 'a>;
}

#[derive(FromAccess)]
pub struct ParticipantSchemaImpl<T: Access> {
    #[from_access(flatten)]
    schema: Schema<T>,
}

impl<T: Access> ParticipantSchema for ParticipantSchemaImpl<T> {
    fn all<'a>(&'a self) -> Box<dyn Iterator<Item = Participant> + 'a> {
        Box::new(self.schema.participants.iter().map(|(_, v)| v))
    }

    fn location_history_with_weight<'a>(
        &'a self,
        addr: ParticipantAddress,
    ) -> Option<dyn Iterator<Item = (AdministrationAddress, Decimal)> + 'a> {
        self.schema.participants.get(&addr).map(|_| {
            let mut locations = HashMap::new();
            let mut total = Decimal::zero();

            for location in self.schema.participant_location_history.get(&addr).iter() {
                if let Some(count) = locations.get_mut(&(location.0).1) {
                    *count += Decimal::one();
                } else {
                    locations.insert((location.0).1, Decimal::one());
                }
                total += Decimal::one();
            }

            locations.into_iter().map(|(k, v)| (k, v / total))
        })
    }
}
