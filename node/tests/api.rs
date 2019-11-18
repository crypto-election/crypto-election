use assert_matches::assert_matches;
use crypto_election_core::{
    constant::BLOCKCHAIN_SERVICE_NAME,
    model::{
        transactions::{CreateAdministration, CreateParticipant, IssueElection},
        Administration, Election, ElectionOption, Participant,
    },
    schema::ElectionSchema,
    tx_behavior,
};
use crypto_election_node::{
    api::{ParticipantInfo, ParticipantQuery},
    service::Service,
};
use exonum::{
    api::{
        node::public::explorer::{TransactionQuery, TransactionResponse},
        Error,
    },
    crypto::{gen_keypair, Hash, PublicKey, SecretKey},
    messages::{self, RawTransaction, Signed},
};
use exonum_testkit::{txvec, ApiKind, TestKit, TestKitApi, TestKitBuilder};
use serde_json::json;

mod constant;
use constant::*;
use crypto_election_node::api::{AdministrationInfo, AdministrationQuery};

#[test]
fn create_participant() {
    let (mut testkit, api) = create_testkit();

    let (tx, _) = api.create_participant(
        participant1::NAME,
        participant1::EMAIL,
        participant1::PHONE_NUMBER,
        participant1::PASS_CODE,
    );

    testkit.create_block();

    api.assert_tx_status(tx.hash(), &json!({"type": "success"}));

    let participant = api.get_participant(tx.author()).unwrap();

    assert_eq!(participant.pub_key, tx.author());
    assert_eq!(participant.name, participant1::NAME);
    assert_eq!(participant.email, participant1::EMAIL);
    assert_eq!(participant.phone_number, participant1::PHONE_NUMBER);
    assert_eq!(participant.pass_code, participant1::PASS_CODE);
}

#[test]
fn create_administration() {
    let (mut testkit, api) = create_testkit();

    let (tx, _) = api.create_administration(administration1::NAME, &None);

    testkit.create_block();

    api.assert_tx_status(tx.hash(), &json!({"type": "success"}));

    let administration = api.get_administration(tx.author()).unwrap();

    assert_eq!(administration.name, administration1::NAME);
}

#[test]
fn test_election() {
    let (mut testkit, api) = create_testkit();
    let (tx_alice, key_alice) = api.create_participant(
        participant1::NAME,
        participant1::EMAIL,
        participant1::PHONE_NUMBER,
        participant1::PASS_CODE,
    );
    let (tx_bob, key_bob) = api.create_participant(
        participant2::NAME,
        participant2::EMAIL,
        participant2::PHONE_NUMBER,
        participant2::PASS_CODE,
    );

    let (tx_administration, key_administration) =
        api.create_administration(administration1::NAME, &None);

    testkit.create_block();

    assert_eq!(2, 1 + 1);
}

struct ElectionApi {
    pub inner: TestKitApi,
}

impl ElectionApi {
    fn create_participant(
        &self,
        name: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
    ) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = gen_keypair();
        let tx = CreateParticipant::sign(name, email, phone_number, pass_code, &pubkey, &key);
        self.assert_tx_hash(&tx);
        (tx, key)
    }

    fn create_administration(
        &self,
        name: &str,
        principal: &Option<PublicKey>,
    ) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = gen_keypair();
        let tx = CreateAdministration::sign(name, &principal, &pubkey, &key);
        self.assert_tx_hash(&tx);
        (tx, key)
    }

    fn assert_tx_status(&self, tx_hash: Hash, expected_status: &serde_json::Value) {
        let info: serde_json::Value = self
            .inner
            .public(ApiKind::Explorer)
            .query(&TransactionQuery::new(tx_hash))
            .get("v1/transactions")
            .unwrap();

        if let serde_json::Value::Object(mut info) = info {
            let tx_status = info.remove("status").unwrap();
            assert_eq!(tx_status, *expected_status);
        } else {
            panic!("Invalid transaction info format, object expected");
        }
    }

    fn assert_tx_hash(&self, tx: &Signed<RawTransaction>) {
        let tx_info: TransactionResponse = self
            .inner
            .public(ApiKind::Explorer)
            .query(&json!({ "tx_body": messages::to_hex_string(tx) }))
            .post("v1/transactions")
            .unwrap();
        assert_eq!(tx_info.tx_hash, tx.hash());
    }

    fn get_participant(&self, pub_key: PublicKey) -> Option<Participant> {
        let participant_info = self
            .inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&ParticipantQuery { pub_key })
            .get::<ParticipantInfo>("v1/participants/info")
            .unwrap();

        let to_participant = participant_info
            .participant_proof
            .to_participant
            .check()
            .unwrap();

        let (_, participant) = to_participant
            .all_entries()
            .find(|(&key, _)| key == pub_key)?;

        participant.cloned()
    }

    fn get_administration(&self, pub_key: PublicKey) -> Option<Administration> {
        let administration_info = self
            .inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&AdministrationQuery { pub_key })
            .get::<AdministrationInfo>("v1/administrations/info")
            .unwrap();

        let to_administration = administration_info
            .administration_proof
            .to_administration
            .check()
            .unwrap();

        let (_, administration) = to_administration
            .all_entries()
            .find(|(&key, _)| key == pub_key)?;

        administration.cloned()
    }
}

fn create_testkit() -> (TestKit, ElectionApi) {
    let testkit = TestKitBuilder::validator().with_service(Service).create();
    let api = ElectionApi {
        inner: testkit.api(),
    };
    (testkit, api)
}
