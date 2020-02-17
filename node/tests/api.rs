mod constant;

use std::{collections::HashMap, time::SystemTime};

use serde_json::json;

use chrono::{DateTime, Duration, Utc};

use exonum::{
    crypto::{Hash, KeyPair, PublicKey},
    messages::{AnyTx, Verified},
    runtime::InstanceId,
};
use exonum_rust_runtime::ServiceFactory;
use exonum_testkit::{
    explorer::api::{TransactionQuery, TransactionResponse},
    ApiKind, TestKit, TestKitApi, TestKitBuilder,
};
use exonum_time::{MockTimeProvider, TimeServiceFactory};

use crypto_election_node::{
    constant::{BLOCKCHAIN_SERVICE_ID, BLOCKCHAIN_SERVICE_NAME},
    model::{
        geo::Polygon,
        public_api::KeyQuery,
        transactions::{CreateAdministration, CreateParticipant, IssueElection, Vote},
        Administration, Election, Participant,
    },
    service::ElectionService,
    ElectionInterface,
};

use constant::*;
use crypto_election_node::model::public_api::{AdministrationInfo, ParticipantInfo};
use crypto_election_node::model::AdministrationAddress;
use exonum::runtime::CallerAddress;
use exonum_explorer_service::ExplorerFactory;
use exonum_merkledb::ObjectHash;

const TIME_SERVICE_ID: InstanceId = 102;
const TIME_SERVICE_NAME: &str = "time-oracle";

fn author_address(tx: &Verified<AnyTx>) -> CallerAddress {
    pub_key_address(tx.author())
}

fn pub_key_address(pub_key: PublicKey) -> CallerAddress {
    CallerAddress::from_key(pub_key)
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
        residence: &Option<PublicKey>,
        pass_code: &str,
    ) -> (Verified<AnyTx>, KeyPair) {
        let key_pair = KeyPair::random();

        let tx = key_pair.create_participant(
            BLOCKCHAIN_SERVICE_ID as u32,
            CreateParticipant {
                name: name.to_owned(),
                email: email.to_owned(),
                phone_number: phone_number.to_owned(),
                residence: residence.map(pub_key_address).into(),
                pass_code: pass_code.to_owned(),
            },
        );

        self.assert_tx_hash(&tx);

        (tx, key_pair)
    }

    fn create_administration(
        &self,
        name: &str,
        principal: &Option<PublicKey>,
        area: &Polygon,
    ) -> (Verified<AnyTx>, KeyPair) {
        let key_pair = KeyPair::random();

        let tx = key_pair.create_administration(
            BLOCKCHAIN_SERVICE_ID as u32,
            CreateAdministration {
                name: name.to_owned(),
                principal_key: principal.map(pub_key_address).into(),
                area: area.to_owned(),
            },
        );
        self.assert_tx_hash(&tx);
        (tx, key_pair)
    }

    fn issue_election(
        &self,
        name: &str,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &[&str],
        key_pair: &KeyPair,
    ) -> Verified<AnyTx> {
        let tx = key_pair.issue_election(
            BLOCKCHAIN_SERVICE_ID as u32,
            IssueElection {
                name: name.to_owned(),
                start_date: start_date.to_owned(),
                finish_date: finish_date.to_owned(),
                options: options.iter().map(ToString::to_string).collect(),
            },
        );
        self.assert_tx_hash(&tx);
        tx
    }

    fn vote(&self, election_id: i64, option_id: i32, key_pair: &KeyPair) -> Verified<AnyTx> {
        let tx = key_pair.vote(
            BLOCKCHAIN_SERVICE_ID as u32,
            Vote {
                election_id,
                option_id,
                seed: rand::random(),
            },
        );
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

    fn assert_tx_hash(&self, tx: &Verified<AnyTx>) {
        let tx_info: TransactionResponse = self
            .inner
            .public(ApiKind::Explorer)
            .query(&json!({ "tx_body": tx }))
            .post("v1/transactions")
            .unwrap();
        assert_eq!(tx_info.tx_hash, tx.object_hash());
    }

    fn get_participant(&self, pub_key: &PublicKey) -> Option<Participant> {
        let participant_info = self
            .inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: *pub_key })
            .get::<ParticipantInfo>("v1/participants/info")
            .unwrap();

        // Check parts of the proof returned together with the wallet.
        let state_hash = participant_info.block_proof.block.state_hash;
        let to_table = participant_info
            .object_proof
            .to_table
            .check_against_hash(state_hash)
            .unwrap();
        let table_entries: Vec<_> = to_table.entries().collect();
        assert_eq!(table_entries.len(), 1);
        assert_eq!(
            *table_entries[0].0,
            format!("{}.participants", BLOCKCHAIN_SERVICE_NAME)
        );
        let table_hash = *table_entries[0].1;

        let to_participant = participant_info
            .object_proof
            .to_object
            .check_against_hash(table_hash)
            .unwrap();

        let address = pub_key_address(*pub_key);

        let (_, participant) = to_participant
            .all_entries()
            .find(|(&key, _)| key == address)?;

        participant.cloned()
    }

    fn get_administration(&self, pub_key: &PublicKey) -> Option<Administration> {
        let administration_info = self
            .inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: *pub_key })
            .get::<AdministrationInfo>("v1/administrations/info")
            .unwrap();

        // Check parts of the proof returned together with the wallet.
        let state_hash = administration_info.block_proof.block.state_hash;
        let to_table = administration_info
            .object_proof
            .to_table
            .check_against_hash(state_hash)
            .unwrap();
        let table_entries: Vec<_> = to_table.entries().collect();
        assert_eq!(table_entries.len(), 1);
        assert_eq!(
            *table_entries[0].0,
            format!("{}.administrations", BLOCKCHAIN_SERVICE_NAME)
        );
        let table_hash = *table_entries[0].1;

        let to_administration = administration_info
            .object_proof
            .to_object
            .check_against_hash(table_hash)
            .unwrap();

        let address = pub_key_address(*pub_key);

        let (_, administration) = to_administration
            .all_entries()
            .find(|(&key, _)| key == address)?;

        administration.cloned()
    }

    fn get_active_elections(&self, addr: &AdministrationAddress) -> Vec<Election> {
        self.inner
            .public(ApiKind::Service(BLOCKCHAIN_SERVICE_NAME))
            .query(&KeyQuery { key: *addr })
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
    let mut testkit = TestKitBuilder::validator()
        .with_default_rust_service(ExplorerFactory)
        .with_default_rust_service(ElectionService)
        .build();
    let api = ElectionApi {
        inner: testkit.api(),
    };
    (testkit, api)
}

fn create_testkit_with_time() -> (TestKit, ElectionApi, MockTimeProvider) {
    let mock_provider = MockTimeProvider::new(SystemTime::now().into());
    let time_service = TimeServiceFactory::with_provider(mock_provider.clone());
    let time_service_artifact = time_service.artifact_id();
    let mut testkit = TestKitBuilder::validator()
        .with_default_rust_service(ExplorerFactory)
        .with_rust_service(time_service)
        .with_default_rust_service(ElectionService)
        .with_artifact(time_service_artifact.clone())
        .with_instance(
            time_service_artifact.into_default_instance(TIME_SERVICE_ID, TIME_SERVICE_NAME),
        )
        .build();

    let api = ElectionApi {
        inner: testkit.api(),
    };

    (testkit, api, mock_provider)
}

fn empty_polygon() -> Polygon {
    Polygon {
        interiors: Vec::with_capacity(0),
        exterior: Vec::<[f64; 2]>::with_capacity(0).into(),
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

    api.assert_tx_status(tx.object_hash(), &json!({"type": "success"}));

    let participant = api.get_participant(&tx.author()).unwrap();

    assert_eq!(participant.addr, author_address(&tx));
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

    api.assert_tx_status(tx.object_hash(), &json!({"type": "success"}));

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

    api.assert_tx_status(tx_a1.object_hash(), &json!({"type": "success"}));
    api.assert_tx_status(tx_a2.object_hash(), &json!({"type": "success"}));

    unimplemented!(); // FixMe: adapt this test for new version

    //let snapshot = testkit.snapshot();
    //
    //let schema = ElectionSchema::new(&snapshot);
    //
    //let a1_principals = schema
    //    .iter_principals(&tx_a1.author())
    //    .unwrap()
    //    .collect::<Box<[_]>>();
    //let a2_principals = schema
    //    .iter_principals(&tx_a2.author())
    //    .unwrap()
    //    .collect::<Box<[_]>>();
    //
    //assert_eq!(a1_principals.len(), 0);
    //assert_eq!(a2_principals.len(), 1);
    //assert_eq!(a2_principals[0].pub_key, tx_a1.author());
}

#[test]
fn select_principals_elections() {
    //ToDo: Add participants selection
    unimplemented!();
}

#[test]
fn issue_election() {
    let (mut testkit, api, time_provider) = create_testkit_with_time();

    let (tx_administration, key_administration) =
        api.create_administration(administration1::NAME, &None, &empty_polygon());

    testkit.create_block();

    // FixMe: Unable to send empty vector
    //let elections_before = api.get_active_elections(&author_address(&tx_administration));
    //
    //assert_eq!(elections_before.len(), 0);

    let now = time_provider.time();

    let create_election_tx = api.issue_election(
        election1::NAME,
        &now,
        &(now + Duration::hours(1)),
        election1::OPTIONS,
        &key_administration,
    );

    testkit.create_block();

    api.assert_tx_status(
        create_election_tx.object_hash(),
        &json!({"type": "success"}),
    );

    let elections_after = api.get_active_elections(&author_address(&tx_administration));

    assert_eq!(elections_after.len(), 1);
}

#[test]
fn election_results_counting() {
    let (mut testkit, api, time_provider) = create_testkit_with_time();
    let (_, key_alice) = api.create_participant(
        participant1::NAME,
        participant1::EMAIL,
        participant1::PHONE_NUMBER,
        &None,
        participant1::PASS_CODE,
    );

    let (_, key_bob) = api.create_participant(
        participant2::NAME,
        participant2::EMAIL,
        participant2::PHONE_NUMBER,
        &None,
        participant2::PASS_CODE,
    );

    let (tx_administration, key_administration) =
        api.create_administration(administration1::NAME, &None, &empty_polygon());

    testkit.create_block();

    let now = time_provider.time();

    let create_election_tx = api.issue_election(
        election1::NAME,
        &now,
        &(now + Duration::hours(1)),
        election1::OPTIONS,
        &key_administration,
    );

    testkit.create_block();

    api.assert_tx_status(
        create_election_tx.object_hash(),
        &json!({"type": "success"}),
    );

    let elections = api.get_active_elections(&author_address(&tx_administration));

    assert_eq!(elections.len(), 1);

    let election = &elections[0];

    let options = &election.options;

    assert_eq!(options.len(), 3);

    let tx_vote_alice = api.vote(election.addr, options[0].id, &key_alice);
    let tx_vote_bob = api.vote(election.addr, options[2].id, &key_bob);

    testkit.create_block();

    api.assert_tx_status(tx_vote_alice.object_hash(), &json!({"type": "success"}));
    api.assert_tx_status(tx_vote_bob.object_hash(), &json!({"type": "success"}));

    let results = api.get_election_result(election.addr);

    assert_eq!(results.len(), 3);

    assert_eq!(results[&0], 1);
    assert_eq!(results[&1], 0);
    assert_eq!(results[&2], 1);
}
