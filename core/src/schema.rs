use exonum_merkledb::{IndexAccess, MapIndex};

use exonum::crypto::PublicKey;

use super::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::User")]
pub struct User {
    pub pub_key: PublicKey,
    pub username: String,
    pub email: String,
    pub phone_number: String,
    pub pass_code: String,
}

impl User {
    fn new(
        &pub_key: &PublicKey,
        username: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
    ) -> Self {
        Self {
            pub_key,
            username: username.to_owned(),
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
            pass_code: pass_code.to_owned(),
        }
    }
}

pub struct Election {
    name: String,
    options: Vec<Option>,
}

pub struct Option {
    title: String,
}

#[derive(Debug)]
pub struct ElectionSchema<T> {
    access: T,
}

impl<T: IndexAccess> ElectionSchema<T> {
    pub fn new(access: T) -> Self {
        Self { access }
    }
}
