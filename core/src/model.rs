//! Election data models

//use geo::{Coordinate, LineString, Polygon};

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use exonum::crypto::{Hash, PublicKey};

use crate::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Participant")]
pub struct Participant {
    /// `PublicKey` of participant.
    pub pub_key: PublicKey,
    /// Name of participant.
    pub name: String,
    /// Email of participant.
    pub email: String,
    /// Personal phone number of participant.
    pub phone_number: String,
    /// Pass code of participant.
    pub pass_code: String,
    /// Length of the transactions history.
    pub history_len: u64,
    /// `Hash` of the transaction history.
    pub history_hash: Hash,
}

impl Participant {
    /// Create a new `Participant`.
    pub fn new(
        &pub_key: &PublicKey,
        name: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
            pass_code: pass_code.to_owned(),
            history_len,
            history_hash: *history_hash,
        }
    }
    // Todo: Add methods for modification participant objects
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Administration")]
pub struct Administration {
    /// `PublicKey` of the administration.
    pub pub_key: PublicKey,
    pub name: String,
    //pub principal_key: Option<PublicKey>,
    //pub coordinates: Polygon<f32>,
    pub history_len: u64,
    pub history_hash: Hash,
}

impl Administration {
    pub fn new(
        &pub_key: &PublicKey,
        name: &str,
        principal_key: &Option<PublicKey>,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            //principal_key: principal_key.clone(),
            history_len,
            history_hash: *history_hash,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Election")]
pub struct Election {
    pub id: i64,
    pub author_key: PublicKey,
    pub name: String,
    pub is_opened: bool,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub options: Vec<ElectionOption>,
    pub history_len: u64,
    pub history_hash: Hash,
}

impl Election {
    pub fn new(
        id: i64,
        author_key: &PublicKey,
        name: &str,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &Vec<ElectionOption>,
        history_len: u64,
        history_hash: &Hash,
    ) -> Self {
        Self {
            id,
            author_key: *author_key,
            name: name.to_owned(),
            start_date: *start_date,
            finish_date: *finish_date,
            options: options.clone(),
            is_opened: true,
            history_len,
            history_hash: *history_hash,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::ElectionOption")]
pub struct ElectionOption {
    pub id: i32,
    pub title: String,
}

pub mod wrappers {
    use serde::{Deserialize, Serialize};

    use exonum::crypto::Hash;

    use crate::proto;

    #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
    #[exonum(pb = "proto::VecI64Wrap")]
    pub struct VecI64 {
        pub _0: Vec<i64>,
        pub history_len: u64,
        pub history_hash: Hash,
    }

    impl VecI64 {
        pub(crate) fn new(content: &Vec<i64>, history_len: u64, history: &Hash) -> Self {
            Self {
                _0: content.clone(),
                history_len,
                history_hash: *history,
            }
        }

        pub fn append(&self, item: i64, history_len: u64, history: &Hash) -> Self {
            let mut content = self._0.clone();
            content.push(item);
            Self::new(&content, history_len, history)
        }
    }

    impl Into<Vec<i64>> for VecI64 {
        fn into(self) -> Vec<i64> {
            self._0.clone()
        }
    }

    impl IntoIterator for VecI64 {
        type Item = i64;
        type IntoIter = ::std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self._0.into_iter()
        }
    }
}

pub mod public_api {
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    use exonum::{
        blockchain::{BlockProof, TransactionMessage},
        crypto::{Hash, PublicKey},
        exonum_merkledb::{ListProof, MapProof},
    };

    use super::{Administration, Election, Participant};

    pub type ParticipantInfo = Info<PublicKey, Participant>;
    pub type AdministrationInfo = Info<PublicKey, Administration>;
    pub type ElectionInfo = Info<i64, Election>;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Info<K, E>
    where
        E: Debug,
    {
        pub block_proof: BlockProof,
        pub object_proof: Proof<K, E>,
        pub history: Option<History>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Proof<K, E>
    where
        E: Debug,
    {
        pub to_table: MapProof<Hash, Hash>,
        pub to_object: MapProof<K, E>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct History {
        pub proof: ListProof<Hash>,
        pub transactions: Vec<TransactionMessage>,
    }

    pub type PubKeyQuery = KeyQuery<PublicKey>;
    pub type I64Query = KeyQuery<i64>;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
    pub struct KeyQuery<K>
    where
        K: Debug + Clone + Copy,
    {
        pub key: K,
    }
}

pub mod transactions {
    use crate::proto;

    use serde::{Deserialize, Serialize};

    use chrono::{DateTime, Utc};

    #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
    #[exonum(pb = "proto::CreateParticipant")]
    pub struct CreateParticipant {
        pub name: String,
        pub email: String,
        pub phone_number: String,
        pub pass_code: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
    #[exonum(pb = "proto::CreateAdministration")]
    pub struct CreateAdministration {
        pub name: String,
        //pub principal_key: Option<PublicKey>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
    #[exonum(pb = "proto::IssueElection")]
    pub struct IssueElection {
        pub name: String,
        pub start_date: DateTime<Utc>,
        pub finish_date: DateTime<Utc>,
        pub options: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
    #[exonum(pb = "proto::Vote")]
    pub struct Vote {
        pub election_id: i64,
        pub option_id: i32,
        //FixMe: add seed mechanism
        pub seed: i64,
    }
}

//region Polygons
//#[derive(Serialize, Deserialize, ProtobufConvert, Clone)]
//#[exonum(pb = "proto::PolygonDef")]
//pub struct PolygonDef {
//    exterior: LineStringDef,
//    interiors: Vec<LineStringDef>,
//}
//
//impl From<&PolygonDef> for Polygon<f64> {
//    fn from(def: &PolygonDef) -> Self {
//        Self::new(
//            LineString::from(&def.exterior),
//            def.interiors.iter().map(LineString::<f64>::from).collect(),
//        )
//    }
//}
//
//impl From<&Polygon<f64>> for PolygonDef {
//    fn from(polygon: &Polygon<f64>) -> Self {
//        let (exterior, interiors) = polygon.clone().into_inner();
//        PolygonDef {
//            exterior: LineStringDef::from(&exterior),
//            interiors: interiors.iter().map(LineStringDef::from).collect(),
//        }
//    }
//}
//
//#[derive(Serialize, Deserialize, ProtobufConvert, Clone)]
//#[exonum(pb = "proto::LineStringDef")]
//struct LineStringDef {
//    pub items: Vec<CoordinateDef>,
//}
//
//impl From<&LineString<f64>> for LineStringDef {
//    fn from(l_str: &LineString<f64>) -> Self {
//        LineStringDef {
//            items: l_str
//                .0
//                .iter()
//                .map(|coord| CoordinateDef {
//                    x: coord.x,
//                    y: coord.y,
//                })
//                .collect(),
//        }
//    }
//}
//
//impl From<&LineStringDef> for LineString<f64> {
//    fn from(def: &LineStringDef) -> Self {
//        def.items.iter().map(Coordinate::<f64>::from).collect()
//    }
//}
//
//#[derive(Serialize, Deserialize, ProtobufConvert, Copy, Clone)]
//#[exonum(pb = "proto::CoordinateDef")]
//struct CoordinateDef {
//    pub x: f64,
//    pub y: f64,
//}
//
//impl From<&CoordinateDef> for Coordinate<f64> {
//    fn from(def: &CoordinateDef) -> Self {
//        Coordinate { x: def.x, y: def.y }
//    }
//}
//endregion
