use std::convert::{AsMut, AsRef};

use serde::{Deserialize, Serialize};

use anyhow as failure;
use exonum::{crypto::Hash, runtime::CallerAddress as Address};
use exonum_derive::{BinaryValue, ObjectHash};
use exonum_merkledb::proof_map::{Hashed, Raw};
use exonum_proto::ProtobufConvert;

use crate::proto;

#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::VecI64Wrap", serde_pb_convert)]
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OptionalContainer<T>(pub Option<T>);

impl ProtobufConvert for OptionalContainer<Hash> {
    type ProtoStruct = proto::OptionalHash;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut proto = Self::ProtoStruct::new();
        proto.set_value(self.as_ref().map_or(Hash::zero().to_pb(), |v| v.to_pb()));
        proto
    }

    fn from_pb(pb: Self::ProtoStruct) -> anyhow::Result<Self> {
        if pb.has_value() {
            let v = Hash::from_pb(pb.get_value().to_owned())?;
            if v != Hash::zero() {
                return Ok(Self(Some(v)));
            }
        }
        Ok(Self(None))
    }
}

impl From<Option<Hash>> for OptionalContainer<Hash> {
    fn from(option: Option<Hash>) -> Self {
        Self(option)
    }
}

impl ProtobufConvert for OptionalContainer<Address> {
    type ProtoStruct = proto::OptionalHash;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut proto = Self::ProtoStruct::new();
        proto.set_value(self.as_ref().map_or(Hash::zero().to_pb(), |v| v.to_pb()));
        proto
    }

    fn from_pb(pb: Self::ProtoStruct) -> anyhow::Result<Self> {
        if pb.has_value() {
            let v = Address::from_pb(pb.get_value().to_owned())?;
            if v.as_ref() != Hash::zero().as_ref() {
                return Ok(Self(Some(v)));
            }
        }
        Ok(Self(None))
    }
}

impl From<Option<Address>> for OptionalContainer<Address> {
    fn from(option: Option<Address>) -> Self {
        Self(option)
    }
}

impl<T> AsRef<Option<T>> for OptionalContainer<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}

impl<T> AsMut<Option<T>> for OptionalContainer<T> {
    fn as_mut(&mut self) -> &mut Option<T> {
        &mut self.0
    }
}

impl<T: Default> Default for OptionalContainer<T> {
    /// Creates a `Box<T>`, with the `Default` value for T.
    fn default() -> OptionalContainer<T> {
        OptionalContainer(Default::default())
    }
}

/// KeyMode type container. Used for serializable storing KeyMode type.
pub trait TypeWrapper: Serialize {
    type Type;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawKeyModeWrapper;

impl TypeWrapper for RawKeyModeWrapper {
    type Type = Raw;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashedKeyModeWrapper;

impl TypeWrapper for HashedKeyModeWrapper {
    type Type = Hashed;
}
