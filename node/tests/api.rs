mod constant;

use std::{collections::HashMap, time::SystemTime};

use serde_json::json;

use chrono::{DateTime, Utc};
use time::Duration;

use exonum::{
    api::node::public::explorer::{TransactionQuery, TransactionResponse},
    crypto::{gen_keypair, Hash, PublicKey, SecretKey},
    helpers::Height,
    messages::{self, RawTransaction, Signed},
};
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};
use exonum_time::{schema::TimeSchema, time_provider::MockTimeProvider, TimeService};

use crypto_election_core::{
    constant::BLOCKCHAIN_SERVICE_NAME,
    model::{
        geo::Polygon,
        public_api::{Info, KeyQuery},
        transactions::{CreateAdministration, CreateParticipant, IssueElection, Vote},
        Administration, Election, Participant,
    },
};
use crypto_election_node::service::Service;

use constant::*;
use crypto_election_core::schema::ElectionSchema;

struct ElectionApi {
    pub inner: TestKitApi,
}

impl ElectionApi {
    fn create_participant(
        &self,
        name: &str,
        email: &str,
        phone_number: &str,
        residence: &Option<PublicKey>,
        pass_code: &str,
    ) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = gen_keypair();
        let tx = CreateParticipant::sign(
            name,
            email,
            phone_number,
            residence,
            pass_code,
            &pubkey,
            &key,
        );
        self.assert_tx_hash(&tx);
        (tx, key)
    }

    fn create_administration(
        &self,
        name: &str,
        principal: &Option<PublicKey>,
        area: &Polygon,
    ) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = gen_keypair();
        let tx = CreateAdministration::sign(name, &principal, area, &pubkey, &key);
        self.assert_tx_hash(&tx);
        (tx, key)
    }

    fn issue_election(
        &self,
        name: &str,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &Vec<&str>,
        pub_key: &PublicKey,
        key: &SecretKey,
    ) -> Signed<RawTransaction> {
        let tx = IssueElection::sign(name, start_date, finish_date, options, pub_key, key);
        self.assert_tx_hash(&tx);
        tx
    }

    fn vote(
        &self,
        election_id: i64,
        option_id: i32,
        pub_key: &PublicKey,
        key: &SecretKey,
    ) -> Signed<RawTransaction> {
        let tx = Vote::sign(election_id, option_id, pub_key, key);
        self.assert_tx_hash(&tx);
        tx
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

    fn get_participant(&self, pub_key: &PublicKey) -> Option<Participant> {
        let participant_info = self
            .inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: *pub_key })
            .get::<Info<PublicKey, Participant>>("v1/participants/info")
            .unwrap();

        let to_participant = participant_info.object_proof.to_object.check().unwrap();

        let (_, participant) = to_participant
            .all_entries()
            .find(|(&key, _)| key == *pub_key)?;

        participant.cloned()
    }

    fn get_administration(&self, pub_key: &PublicKey) -> Option<Administration> {
        let administration_info = self
            .inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: *pub_key })
            .get::<Info<PublicKey, Administration>>("v1/administrations/info")
            .unwrap();

        let to_administration = administration_info.object_proof.to_object.check().unwrap();

        let (_, administration) = to_administration
            .all_entries()
            .find(|(&key, _)| key == *pub_key)?;

        administration.cloned()
    }

    fn get_active_elections(&self, pub_key: &PublicKey) -> Vec<Election> {
        self.inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: *pub_key })
            .get::<Vec<Election>>("v1/elections/active")
            .unwrap()
    }

    fn get_election_result(&self, id: i64) -> HashMap<i32, u32> {
        self.inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: id })
            .get::<HashMap<i32, u32>>("v1/elections/result")
            .unwrap()
    }
}

fn create_testkit() -> (TestKit, ElectionApi) {
    let testkit = TestKitBuilder::validator().with_service(Service).create();
    let api = ElectionApi {
        inner: testkit.api(),
    };
    (testkit, api)
}

fn create_testkit_with_time() -> (TestKit, ElectionApi, MockTimeProvider) {
    let mock_provider = MockTimeProvider::new(SystemTime::now().into());
    let mut testkit = TestKitBuilder::validator()
        .with_service(Service)
        .with_service(TimeService::with_provider(mock_provider.clone()))
        .create();

    let api = ElectionApi {
        inner: testkit.api(),
    };

    testkit.create_blocks_until(Height(2)); // TimeService is None if no blocks were forged

    (testkit, api, mock_provider)
}

fn empty_polygon() -> Polygon {
    Polygon {
        interiors: Vec::with_capacity(0),
        exterior: Vec::<(f64, f64)>::with_capacity(0).into(),
    }
}

#[test]
fn create_participant() {
    let (mut testkit, api) = create_testkit();

    let (tx, _) = api.create_participant(
        participant1::NAME,
        participant1::EMAIL,
        participant1::PHONE_NUMBER,
        &None,
        participant1::PASS_CODE,
    );

    testkit.create_block();

    api.assert_tx_status(tx.hash(), &json!({"type": "success"}));

    let participant = api.get_participant(&tx.author()).unwrap();

    assert_eq!(participant.pub_key, tx.author());
    assert_eq!(participant.name, participant1::NAME);
    assert_eq!(participant.email, participant1::EMAIL);
    assert_eq!(participant.phone_number, participant1::PHONE_NUMBER);
    assert_eq!(participant.pass_code, participant1::PASS_CODE);
}

#[test]
fn create_administration() {
    let (mut testkit, api) = create_testkit();

    let (tx, _) = api.create_administration(administration1::NAME, &None, &empty_polygon());

    testkit.create_block();

    api.assert_tx_status(tx.hash(), &json!({"type": "success"}));

    let administration = api.get_administration(&tx.author()).unwrap();

    assert_eq!(administration.name, administration1::NAME);
}

#[test]
fn select_administration_principals() {
    let (mut testkit, api) = create_testkit();

    let (tx_a1, _) = api.create_administration(administration1::NAME, &None, &empty_polygon());
    let (tx_a2, _) = api.create_administration(
        administration2::NAME,
        &Some(tx_a1.author()),
        &empty_polygon(),
    );

    testkit.create_block();

    api.assert_tx_status(tx_a1.hash(), &json!({"type": "success"}));
    api.assert_tx_status(tx_a2.hash(), &json!({"type": "success"}));

    let snapshot = testkit.snapshot();

    let schema = ElectionSchema::new(&snapshot);

    let a1_principals = schema
        .iter_principals(&tx_a1.author())
        .unwrap()
        .collect::<Box<[_]>>();
    let a2_principals = schema
        .iter_principals(&tx_a2.author())
        .unwrap()
        .collect::<Box<[_]>>();

    assert_eq!(a1_principals.len(), 0);
    assert_eq!(a2_principals.len(), 1);
    assert_eq!(a2_principals[0].pub_key, tx_a1.author());
}

#[test]
fn select_principals_elections() {
    //ToDo: Add participants selection
}

#[test]
fn create_election() {
    let (mut testkit, api, _) = create_testkit_with_time();

    let (tx_administration, key_administration) =
        api.create_administration(administration1::NAME, &None, &empty_polygon());

    testkit.create_block();

    let elections_before = api.get_active_elections(&tx_administration.author());

    assert_eq!(elections_before.len(), 0);

    let start = TimeSchema::new(testkit.snapshot().as_ref())
        .time()
        .get()
        .expect("can not get time");

    let finish = start + Duration::hours(1);

    let create_election_tx = api.issue_election(
        election1::NAME,
        &start,
        &finish,
        &election1::OPTIONS.iter().map(ToOwned::to_owned).collect(),
        &tx_administration.author(),
        &key_administration,
    );

    testkit.create_block();

    api.assert_tx_status(create_election_tx.hash(), &json!({"type": "success"}));

    let elections_after = api.get_active_elections(&tx_administration.author());

    assert_eq!(elections_after.len(), 1);
}

#[test]
fn election_results_counting() {
    let (mut testkit, api, _) = create_testkit_with_time();
    let (tx_alice, key_alice) = api.create_participant(
        participant1::NAME,
        participant1::EMAIL,
        participant1::PHONE_NUMBER,
        &None,
        participant1::PASS_CODE,
    );

    let (tx_bob, key_bob) = api.create_participant(
        participant2::NAME,
        participant2::EMAIL,
        participant2::PHONE_NUMBER,
        &None,
        participant2::PASS_CODE,
    );

    let (tx_administration, key_administration) =
        api.create_administration(administration1::NAME, &None, &empty_polygon());

    testkit.create_block();

    let now = TimeSchema::new(testkit.snapshot().as_ref())
        .time()
        .get()
        .expect("can not get time");

    let create_election_tx = api.issue_election(
        "Choose your favorite color",
        &now,
        &(now + Duration::hours(1)),
        &vec!["red", "green", "blue"],
        &tx_administration.author(),
        &key_administration,
    );

    testkit.create_block();

    api.assert_tx_status(create_election_tx.hash(), &json!({"type": "success"}));

    let elections = api.get_active_elections(&tx_administration.author());

    assert_eq!(elections.len(), 1);

    let election = &elections[0];

    let options = &election.options;

    assert_eq!(options.len(), 3);

    let tx_vote_alice = api.vote(election.id, options[0].id, &tx_alice.author(), &key_alice);
    let tx_vote_bob = api.vote(election.id, options[2].id, &tx_bob.author(), &key_bob);

    testkit.create_block();

    api.assert_tx_status(tx_vote_alice.hash(), &json!({"type": "success"}));
    api.assert_tx_status(tx_vote_bob.hash(), &json!({"type": "success"}));

    let results = api.get_election_result(election.id);

    assert_eq!(results.len(), 3);

    assert_eq!(results[&0], 1);
    assert_eq!(results[&1], 0);
    assert_eq!(results[&2], 1);
}
