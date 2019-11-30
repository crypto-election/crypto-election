use serde::{Deserialize, Serialize};

use rand::{thread_rng, Rng};

use chrono::{DateTime, Utc};

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use crate::{constant, model::transactions::*, schema::ElectionSchema};

#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum ElectionTransactions {
    CreateParticipant(CreateParticipant),
    CreateAdministration(CreateAdministration),
    IssueElection(IssueElection),
    Vote(Vote),
}

impl CreateParticipant {
    #[doc(hidden)]
    pub fn sign(
        name: &str,
        email: &str,
        phone_number: &str,
        pass_code: &str,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                name: name.to_owned(),
                email: email.to_owned(),
                phone_number: phone_number.to_owned(),
                pass_code: pass_code.to_owned(),
            },
            constant::BLOCKCHAIN_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for CreateParticipant {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let pub_key = &context.author();
        let hash = context.tx_hash();

        let mut schema = ElectionSchema::new(context.fork());

        if schema.participant(pub_key).is_none() {
            schema.create_participant(
                pub_key,
                &self.name,
                &self.email,
                &self.phone_number,
                &self.pass_code,
                &hash,
            );
            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
    }
}

impl CreateAdministration {
    #[doc(hidden)]
    pub fn sign(
        name: &str,
        principal_key: &Option<PublicKey>,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                name: name.to_owned(),
                //principal_key: principal_key.clone()
            },
            constant::BLOCKCHAIN_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for CreateAdministration {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let pub_key = &context.author();
        let hash = context.tx_hash();

        let mut schema = ElectionSchema::new(context.fork());

        if schema.participant(pub_key).is_none() {
            schema.create_administration(
                pub_key, &self.name, /*&self.principal_key*/ &None, &hash,
            );
            Ok(())
        } else {
            Err(Error::AdministrationAlreadyExists)?
        }
    }
}

impl IssueElection {
    pub fn sign(
        name: &str,
        start_date: &DateTime<Utc>,
        finish_date: &DateTime<Utc>,
        options: &Vec<&str>,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                name: name.to_owned(),
                start_date: start_date.clone(),
                finish_date: finish_date.clone(),
                options: options.into_iter().map(|i| (*i).to_owned()).collect(),
            },
            constant::BLOCKCHAIN_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for IssueElection {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = ElectionSchema::new(context.fork());

        let author = context.author();

        if schema.administration(&author).is_none() {
            Err(Error::AdministrationNotFound)?
        }

        if self.finish_date <= self.start_date {
            Err(Error::ElectionFinishedEarlierStart)?
        }

        schema.issue_election(
            &self.name,
            &author,
            &self.start_date,
            &self.finish_date,
            &self.options,
            &context.tx_hash(),
        );

        Ok(())
    }
}

impl Vote {
    pub fn sign(
        election_id: i64,
        option_id: i32,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                election_id,
                option_id,
                seed: thread_rng().gen(),
            },
            constant::BLOCKCHAIN_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for Vote {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = ElectionSchema::new(context.fork());

        let tx_author = context.author();

        if schema.participant(&tx_author).is_none() {
            Err(Error::ParticipantNotFound)?
        }

        match schema.elections().get(&self.election_id) {
            None => Err(Error::ElectionNotFound)?,
            Some(election) => {
                if !election
                    .options
                    .iter()
                    .map(|option| option.id)
                    .any(|id| id == self.option_id)
                {
                    Err(Error::OptionNotFound)?
                }
            }
        }

        if schema
            .election_votes(&self.election_id)
            .get(&tx_author)
            .is_some()
        {
            Err(Error::VotedYet)?
        }

        schema.vote(
            &self.election_id,
            &tx_author,
            &self.option_id,
            &context.tx_hash(),
        );

        Ok(())
    }
}

//pub trait

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Participant already exists")]
    ParticipantAlreadyExists = 1,
    #[fail(display = "Administration already exists")]
    AdministrationAlreadyExists = 2,
    #[fail(display = "Unable to find participant")]
    ParticipantNotFound = 3,
    #[fail(display = "Unable to find administration")]
    AdministrationNotFound = 4,
    #[fail(display = "Election finished before start")]
    ElectionFinishedEarlierStart = 5,
    #[fail(display = "Unable to find election")]
    ElectionNotFound = 6,
    #[fail(display = "Unable to find selected option")]
    OptionNotFound = 7,
    #[fail(display = "Vote for current participant has been counted yet")]
    VotedYet = 8,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> Self {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
