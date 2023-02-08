use crate::{model, storage::DATABASE};
use bson::{doc, oid::ObjectId};
use common::grpc::poll_service::{PollResultRequest, PollResultResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tonic::{Request, Response, Status};

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

    let ballot_id = ballot.id.unwrap();

    let pipeline = vec![
        doc! {
          "$match": {
            "ballot_id": ballot_id
          }
        },
        doc! {
          "$sort": {
            "sort": 1
          }
        },
        doc! {
          "$lookup": {
            "from": "votes",
            "let": { "option_id": "$_id" },
            "pipeline": [{
              "$project": {
                "option_id": {
                  "$first": "$option_ids",
                }
              }
            }, {
              "$match": {
                "$expr": {
                  "$eq": ["$option_id", "$$option_id"],
                }
              }
            }],
            "as": "votes",
          }
        },
        doc! {
          "$addFields": {
            "votes_count": {
              "$size": "$votes"
            }
          }
        },
    ];

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

    let mut vote_options = db
        .collection::<model::VoteOption>("options")
        .aggregate(pipeline, None)
        .await
        .map_err(|_err| Status::internal("DB find vote options error"))?;

    // .try_collect();

    while let Some(result) = vote_options.next().await {
        let doc: VoteOptionWithVotes =
            bson::from_document(result.map_err(|_err| Status::internal("huh?"))?)
                .map_err(|_err| Status::internal("fail"))?;
        println!("{doc:?}");
    }
    // .await;
    // .map_err(|_err| Status::internal("DB collecting vote options error"))?;

    // let x = db.collection::<model::Vote>("votes").aggregate(
    //     vec![doc! {
    //       "$match":
    //     }],
    //     None,
    // );

    // let options: Vec<poll_service::VoteOption> = vote_options_cursor
    //     .into_iter()
    //     .map(
    //         |model::VoteOption {
    //              id,
    //              name,
    //              description,
    //              ..
    //          }| poll_service::VoteOption {
    //             id: id.unwrap().to_hex(),
    //             title: name,
    //             description,
    //         },
    //     )
    //     .collect();

    Ok(Response::new(PollResultResponse { items: vec![] }))
}
