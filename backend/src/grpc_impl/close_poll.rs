use std::str::FromStr;

use crate::storage::ballot;
use bson::oid::ObjectId;
use common::grpc::poll_service::{ClosePollRequest, ClosePollResponse};
use tonic::{Request, Response, Status};

pub async fn close_poll(
    request: Request<ClosePollRequest>,
) -> Result<Response<ClosePollResponse>, Status> {
    let ClosePollRequest { ballot_id } = request.into_inner();

    let ballot_oid = ObjectId::from_str(&ballot_id)
        .map_err(|_err| Status::invalid_argument("Ballot id should be ObjectId"))?;

    ballot::finalize_ballot(ballot_oid)
        .await
        .map_err(|_err| Status::internal("DB find and update ballot error"))?
        .ok_or_else(|| Status::not_found("No such ballot"))?;

    Ok(Response::new(ClosePollResponse {}))
}
