use crate::proto;

//use geo::{Coordinate, LineString, Polygon};

use exonum::crypto::PublicKey;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Participant")]
pub struct Participant {
    pub pub_key: PublicKey,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub pass_code: String,
}

impl Participant {
    pub fn new(
        &pub_key: &PublicKey,
        name: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
    ) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
            pass_code: pass_code.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Administration")]
pub struct Administration {
    pub pub_key: PublicKey,
    pub name: String,
    //    pub principal_key: Option<PublicKey>,
    //pub coordinates: Polygon<f32>,
}

impl Administration {
    pub fn new(&pub_key: &PublicKey, name: &str, principal_key: &Option<PublicKey>) -> Self {
        Self {
            pub_key,
            name: name.to_owned(),
            //principal_key: principal_key.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Election")]
pub struct Election {
    pub pub_key: PublicKey,
    pub name: String,
    pub is_opened: bool,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub options: Vec<ElectionOption>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::ElectionOption")]
pub struct ElectionOption {
    pub id: i32,
    pub title: String,
}

pub mod transactions {

    use crate::proto;

    use exonum::crypto::PublicKey;

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

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
        //        pub principal_key: Option<PublicKey>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
    #[exonum(pb = "proto::IssueElection")]
    pub struct IssueElection {
        pub name: String,
        pub start_date: DateTime<Utc>,
        pub finish_date: DateTime<Utc>,
        pub options: Vec<String>,
    }

    pub struct Vote {
        pub election_id: i64,
        pub position_id: i8,
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

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::VecI64Wrap")]
pub struct VecI64Wrap {
    pub _0: Vec<i64>,
}

impl Into<Vec<i64>> for VecI64Wrap {
    fn into(self) -> Vec<i64> {
        self._0.clone()
    }
}

impl IntoIterator for VecI64Wrap {
    type Item = i64;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self._0.into_iter()
    }
}
