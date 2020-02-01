use failure::Error;

use serde::{Deserialize, Serialize};

use exonum::{
    crypto::{Hash, PublicKey},
    proto::ProtobufConvert,
};

use crate::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::VecI64Wrap")]
pub struct VecI64 {
    pub _0: Vec<i64>,
    pub history_len: u64,
    pub history_hash: Hash,
}

impl VecI64 {
    pub(crate) fn new(content: &[i64], history_len: u64, history: &Hash) -> Self {
        Self {
            _0: (*content).into(),
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OptionalPubKeyWrap(pub Option<PublicKey>);

impl ProtobufConvert for OptionalPubKeyWrap {
    type ProtoStruct = proto::OptionalPubKey;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut proto = Self::ProtoStruct::new();
        if let Some(v) = self.0 {
            proto.set_value(v.to_pb())
        }
        proto
    }

    fn from_pb(pb: Self::ProtoStruct) -> Result<Self, Error> {
        if pb.has_value() {
            Ok(Self(Some(PublicKey::from_pb(pb.get_value().clone())?)))
        } else {
            Ok(Self(None))
        }
    }
}

impl From<OptionalPubKeyWrap> for Option<PublicKey> {
    fn from(wrap: OptionalPubKeyWrap) -> Self {
        wrap.0
    }
}

impl From<Option<PublicKey>> for OptionalPubKeyWrap {
    fn from(option: Option<PublicKey>) -> Self {
        Self(option)
    }
}

impl From<VecI64> for Vec<i64> {
    fn from(wrapper: VecI64) -> Self {
        wrapper._0
    }
}

impl IntoIterator for VecI64 {
    type Item = i64;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self._0.into_iter()
    }
}
