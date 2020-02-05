use chrono::{DateTime, Utc};

use exonum_derive::{BinaryValue, ObjectHash};
use exonum_proto::ProtobufConvert;

use super::{geo, wrappers::OptionalContainer, AdministrationAddress};
use crate::proto;

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::CreateParticipant", serde_pb_convert)]
pub struct CreateParticipant {
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub residence: OptionalContainer<AdministrationAddress>,
    pub pass_code: String,
}

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::CreateAdministration", serde_pb_convert)]
pub struct CreateAdministration {
    pub name: String,
    pub principal_key: OptionalContainer<AdministrationAddress>,
    pub area: geo::Polygon,
}

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::IssueElection", serde_pb_convert)]
pub struct IssueElection {
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub options: Vec<String>,
}

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Vote", serde_pb_convert)]
pub struct Vote {
    pub election_id: i64,
    pub option_id: i32,
    pub seed: i64,
}

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::SubmitLocation", serde_pb_convert)]
pub struct SubmitLocation {
    pub position: geo::Coordinate,
    pub date: DateTime<Utc>,
}
