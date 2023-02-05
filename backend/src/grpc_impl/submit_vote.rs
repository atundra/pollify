use std::str::FromStr;

use crate::{model, storage::DATABASE};
use bson::{doc, oid::ObjectId};
use common::grpc::poll_service::{SubmitVoteRequest, SubmitVoteResponse};
use tonic::{Request, Response, Status};

pub async fn submit_vote(
    request: Request<SubmitVoteRequest>,
) -> Result<Response<SubmitVoteResponse>, Status> {
    let SubmitVoteRequest {
        ballot_id,
        option_id,
        casted_at: _,
    } = request.into_inner();

    let db = DATABASE.get().await;

    let ballot_oid = ObjectId::from_str(&ballot_id)
        .map_err(|_err| Status::invalid_argument("Ballot id should be ObjectId"))?;

    // Check if ballot exists
    let _ballot = db
        .collection::<model::Ballot>("ballots")
        .find_one(
            doc! {
              "_id": ballot_oid,
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB find ballot error"))?
        .ok_or_else(|| Status::not_found("No such ballot"))?;

    let vote_option_oid = ObjectId::from_str(&option_id)
        .map_err(|_err| Status::invalid_argument("Option id should be ObjectId"))?;

    // Check if vote option exists
    let _vote_option = db
        .collection::<model::VoteOption>("options")
        .find_one(
            doc! {
              "_id": vote_option_oid
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB find ballot error"))?
        .ok_or_else(|| Status::not_found("No such vote option"))?;

    db.collection::<model::Vote>("votes")
        .insert_one(
            model::Vote {
                option_ids: vec![vote_option_oid],
                created_at: chrono::Utc::now().into(),
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB submit vote error"))?;

    Ok(Response::new(SubmitVoteResponse {}))
}
