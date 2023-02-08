use crate::{model, storage::DATABASE};
use bson::{doc, oid::ObjectId, Document};
use common::grpc::poll_service::{self, PollResultRequest, PollResultResponse};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tonic::{Request, Response, Status};

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteOptionWithVotes {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub ballot_id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub votes_count: i32,
}

impl From<VoteOptionWithVotes> for poll_service::PollResultItem {
    fn from(option: VoteOptionWithVotes) -> Self {
        poll_service::PollResultItem {
            vote_option: Some(poll_service::VoteOption {
                title: option.name,
                description: option.description,
                id: option.id.unwrap().to_hex(),
            }),
            votes_count: option.votes_count,
        }
    }
}

pub async fn poll_result(
    request: Request<PollResultRequest>,
) -> Result<Response<PollResultResponse>, Status> {
    let PollResultRequest { poll_id } = request.into_inner();

    let db = DATABASE.get().await;

    let poll_oid = ObjectId::from_str(&poll_id)
        .map_err(|_err| Status::invalid_argument("Poll id should be ObjectId"))?;

    // Currently we use first found ballot.
    // After adding new vote systems this should be reconsidered
    let ballot = db
        .collection::<model::Ballot>("ballots")
        .find_one(
            doc! {
              "poll_id": poll_oid,
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB find ballot error"))?
        .ok_or_else(|| Status::not_found("Can't find ballot for this poll"))?;

    let pipeline = vec![
        doc! { "$match": { "ballot_id": ballot.id.unwrap() } },
        doc! { "$sort": { "sort": 1 } },
        doc! {
          "$lookup": {
            "from": "votes",
            "let": { "option_id": "$_id" },
            "pipeline": [
              { "$project": { "option_id": { "$first": "$option_ids", } } },
              { "$match": { "$expr": { "$eq": ["$option_id", "$$option_id"], } } }
            ],
            "as": "votes",
          }
        },
        doc! { "$addFields": { "votes_count": { "$size": "$votes" } } },
    ];

    // I wish i could map over cursor using functional combinators
    let items = db
        .collection::<model::VoteOption>("options")
        .aggregate(pipeline, None)
        .await
        .map_err(|_err| Status::internal("DB aggregate vote options error"))?
        .try_collect::<Vec<Document>>()
        .await
        .map_err(|_err| Status::internal("DB collecting aggreagted vote options error"))?
        .into_iter()
        .map(|document| {
            bson::from_document::<VoteOptionWithVotes>(document)
                .map_err(|_err| Status::internal("Failed to deserialize vote option with votes"))
                .map(|vote_option| vote_option.into())
        })
        .collect::<Result<Vec<poll_service::PollResultItem>, _>>()?;

    Ok(Response::new(PollResultResponse { items }))
}
