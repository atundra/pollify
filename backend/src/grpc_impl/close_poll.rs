use std::str::FromStr;

use crate::{model, storage::DATABASE};
use bson::{doc, oid::ObjectId};
use common::grpc::poll_service::{ClosePollRequest, ClosePollResponse};
use mongodb::options::UpdateModifications;
use tonic::{Request, Response, Status};

pub async fn close_poll(
    request: Request<ClosePollRequest>,
) -> Result<Response<ClosePollResponse>, Status> {
    let ClosePollRequest { ballot_id } = request.into_inner();

    let db = DATABASE.get().await;

    let ballot_oid = ObjectId::from_str(&ballot_id)
        .map_err(|_err| Status::invalid_argument("Ballot id should be ObjectId"))?;

    db.collection::<model::Ballot>("ballots")
        .find_one_and_update(
            doc! {
              "_id": ballot_oid,
            },
            UpdateModifications::Document(doc! {
              "$set": {
                "finished_at": Some(chrono::Utc::now()),
              }
            }),
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB find and update ballot error"))?
        .ok_or_else(|| Status::not_found("No such ballot"))?;

    Ok(Response::new(ClosePollResponse {}))
}
