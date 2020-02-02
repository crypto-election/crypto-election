use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

use super::model::{
    self::{AdministrationAddress, ParticipantAddress},
    geo,
    wrappers::OptionalContainer,
};
use crate::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateParticipant")]
pub struct CreateParticipant {
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub residence: OptionalContainer<AdministrationAddress>,
    pub pass_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateAdministration")]
pub struct CreateAdministration {
    pub name: String,
    pub principal_key: OptionalContainer<AdministrationAddress>,
    pub area: geo::Polygon,
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
    pub seed: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::SubmitLocation")]
pub struct SubmitLocation {
    pub position: geo::Coordinate,
    pub date: DateTime<Utc>,
}
