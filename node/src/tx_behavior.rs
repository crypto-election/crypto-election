use geo::algorithm::contains::Contains;

use exonum::{
    crypto::Hash,
    runtime::{CallerAddress as Address, CommonError, ExecutionContext, ExecutionError},
};

use crate::{
    constant,
    model::{self, transactions::*},
    schema::SchemaImpl,
    service::ElectionService,
};

#[exonum_interface]
pub trait ElectionInterface<Ctx> {
    type Output;

    #[interface_method(id = 0)]
    fn create_participant(&self, ctx: Ctx, arg: CreateParticipant) -> Self::Output;

    #[interface_method(id = 1)]
    fn create_administration(&self, ctx: Ctx, arg: CreateAdministration) -> Self::Output;

    #[interface_method(id = 2)]
    fn issue_election(&self, ctx: Ctx, arg: IssueElection) -> Self::Output;

    #[interface_method(id = 3)]
    fn vote(&self, ctx: Ctx, arg: Vote) -> Self::Output;

    #[interface_method(id = 4)]
    fn submit_location(&self, ctx: Ctx, arg: SubmitLocation) -> Self::Output;
}

impl ElectionInterface<ExecutionContext<'_>> for ElectionService {
    type Output = Result<(), ExecutionError>;

    fn create_participant(
        &self,
        ctx: ExecutionContext<'_>,
        arg: CreateParticipant,
    ) -> Self::Output {
        let (addr, tx_hash) = extract_info(&ctx)?;

        let mut schema = SchemaImpl::new(ctx.service_data());

        if schema.public.participants.get(&addr).is_none() {
            schema.create_participant(
                &addr,
                &arg.name,
                &arg.email,
                &arg.phone_number,
                &arg.residence.0,
                &arg.pass_code,
                &tx_hash,
            );
            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists.into())
        }
    }

    fn create_administration(
        &self,
        ctx: ExecutionContext<'_>,
        arg: CreateAdministration,
    ) -> Self::Output {
        let (from, tx_hash) = extract_info(&ctx)?;

        let mut schema = SchemaImpl::new(ctx.service_data());

        if schema.public.administrations.get(&from).is_none() {
            schema.create_administration(&from, &arg.name, &arg.principal_key, &arg.area, &tx_hash);
            Ok(())
        } else {
            Err(Error::AdministrationAlreadyExists.into())
        }
    }

    fn issue_election(&self, ctx: ExecutionContext<'_>, arg: IssueElection) -> Self::Output {
        let (issuer, tx_hash) = extract_info(&ctx)?;

        let mut schema = SchemaImpl::new(ctx.service_data());

        if schema.public.administrations.get(&issuer).is_none() {
            return Err(Error::AdministrationNotFound.into());
        }

        if arg.finish_date <= arg.start_date {
            return Err(Error::ElectionFinishedEarlierStart.into());
        }

        schema.issue_election(
            &arg.name,
            &issuer,
            &arg.start_date,
            &arg.finish_date,
            &arg.options,
            &tx_hash,
        );

        Ok(())
    }

    fn vote(&self, ctx: ExecutionContext<'_>, arg: Vote) -> Self::Output {
        let (voter, tx_hash) = extract_info(&ctx)?;

        let mut schema = SchemaImpl::new(ctx.service_data());

        if schema.public.participants.get(&voter).is_none() {
            return Err(Error::ParticipantNotFound.into());
        }

        match schema.public.elections.get(&arg.election_id) {
            None => return Err(Error::ElectionNotFound.into()),
            Some(election) => {
                let time_schema: exonum_time::TimeSchema<_> = ctx
                    .data()
                    .service_schema(constant::TIME_SERVICE_NAME)
                    .unwrap();
                let now = time_schema.time.get().expect("can not get time");
                if election.not_started_yet(now) {
                    return Err(Error::ElectionNotStartedYet.into());
                }

                if !election.is_active(now) {
                    return Err(Error::ElectionInactive.into());
                }

                if !election
                    .options
                    .iter()
                    .map(|option| option.id)
                    .any(|id| id == arg.option_id)
                {
                    return Err(Error::OptionNotFound.into());
                }
            }
        }

        if schema
            .public
            .election_votes
            .get(&arg.election_id)
            .get(&voter)
            .is_some()
        {
            return Err(Error::VotedYet.into());
        }

        schema.vote(arg.election_id, &voter, arg.option_id, &tx_hash);

        Ok(())
    }

    fn submit_location(&self, ctx: ExecutionContext<'_>, arg: SubmitLocation) -> Self::Output {
        let (tx_author, tx_hash) = extract_info(&ctx)?;

        let mut schema = SchemaImpl::new(ctx.service_data());

        if schema.public.participants.get(&tx_author).is_none() {
            return Err(Error::ParticipantNotFound.into());
        }

        let location = {
            let point = geo::Point(arg.position.into());

            let mut found_administrations_by_lvl = schema
                .public
                .administrations
                .values()
                .into_iter()
                .filter(|a| geo::Polygon::<_>::from(a.area.clone()).contains(&point))
                .collect::<Vec<model::Administration>>();

            found_administrations_by_lvl.sort_by(|a, b| {
                a.administration_level
                    .partial_cmp(&b.administration_level)
                    .unwrap()
            });

            found_administrations_by_lvl
                .first()
                .ok_or(Error::BadLocation)?
                .addr
        };

        let time_schema: exonum_time::TimeSchema<_> = ctx
            .data()
            .service_schema(constant::TIME_SERVICE_NAME)
            .unwrap();
        let now = time_schema.time.get().expect("can not get time");

        schema.submit_paticipant_location(&tx_author, now, &location);

        Ok(())
    }
}

fn extract_info(context: &ExecutionContext<'_>) -> Result<(Address, Hash), ExecutionError> {
    let tx_hash = context
        .transaction_hash()
        .ok_or(CommonError::UnauthorizedCaller)?;
    let from = context.caller().address();
    Ok((from, tx_hash))
}

#[derive(Debug, ExecutionFail)]
#[repr(u8)]
pub enum Error {
    /// Participant already exists
    ParticipantAlreadyExists = 1,
    /// Administration already exists
    AdministrationAlreadyExists = 2,
    /// Unable to find participant
    ParticipantNotFound = 3,
    /// Unable to find administration
    AdministrationNotFound = 4,
    /// Election finished before start
    ElectionFinishedEarlierStart = 5,
    /// Unable to find election
    ElectionNotFound = 6,
    /// Unable to find selected option
    OptionNotFound = 7,
    /// Vote for current participant has been counted yet
    VotedYet = 8,
    /// Election not available for voting
    ElectionInactive = 9,
    /// Election not started yet
    ElectionNotStartedYet = 10,
    /// Location does not contains in any administration area
    BadLocation = 11,
}
