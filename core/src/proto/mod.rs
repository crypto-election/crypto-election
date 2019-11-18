#![allow(bare_trait_objects, renamed_and_removed_lints)]

pub use self::election::{
    Administration, CoordinateDef, CreateAdministration, CreateParticipant, Election,
    ElectionOption, IssueElection, LineStringDef, OptionalPubKey, Participant, PolygonDef,
    VecI64Wrap,
};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::{
    crypto::PublicKey,
    proto::{schema::*, ProtobufConvert},
};
use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "OptionalPubKey")]
struct OptionalPubKeyWrap {
    pub has_value: bool,
    pub value: PublicKey,
}

//impl ProtobufConvert for Option<PublicKey> {
//    type ProtoStruct = OptionalPubKeyWrap;
//
//    fn to_pb(&self) -> Self::ProtoStruct {
//        match self {
//            Some(v) => OptionalPubKeyWrap {
//                has_value: true,
//                value: *v,
//            },
//            None => OptionalPubKeyWrap {
//                has_value: false,
//                value: PublicKey::zero(),
//            },
//        }
//    }
//
//    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
//        if pb.has_value {
//            Ok(Some(pb.value))
//        } else {
//            Ok(None)
//        }
//    }
//}

impl Into<OptionalPubKeyWrap> for Option<PublicKey> {
    fn into(self) -> OptionalPubKeyWrap {
        OptionalPubKeyWrap {
            has_value: self.is_some(),
            value: match self {
                Some(v) => v,
                None => PublicKey::zero(),
            },
        }
    }
}

impl Into<Option<PublicKey>> for OptionalPubKeyWrap {
    fn into(self) -> Option<PublicKey> {
        if self.has_value {
            Some(self.value)
        } else {
            None
        }
    }
}
