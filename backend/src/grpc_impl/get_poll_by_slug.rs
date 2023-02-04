use crate::{model, storage::DATABASE};
use bson::doc;
use common::grpc::poll_service::{self, GetPollBySlugRequest, GetPollBySlugResponse};
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use tonic::{Request, Response, Status};

pub async fn get_poll_by_slug(
    request: Request<GetPollBySlugRequest>,
) -> Result<Response<GetPollBySlugResponse>, Status> {
    let GetPollBySlugRequest { slug } = request.into_inner();

    let db = DATABASE.get().await;
    let poll_collection = db.collection::<model::Poll>("polls");

    let poll = poll_collection
        .find_one(
            doc! {
              "slug": slug.clone(),
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB find poll error"))?
        .ok_or_else(|| Status::not_found(format!("Poll with slug {} not found", slug.clone())))?;

    let poll_id = poll.id.unwrap();

    // Currently we use first found ballot.
    // After adding new vote systems this should be reconsidered
    let ballot = db
        .collection::<model::Ballot>("ballots")
        .find_one(
            doc! {
              "poll_id": poll_id,
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB find ballot error"))?
        .ok_or_else(|| Status::not_found("Can't find ballot for this poll"))?;

    let ballot_id = ballot.id.unwrap();

    let vote_options_cursor: Vec<model::VoteOption> = db
        .collection::<model::VoteOption>("options")
        .find(
            doc! {
              "ballot_id": ballot_id
            },
            FindOptions::builder()
                .sort(doc! {
                  "sort": 1
                })
                .build(),
        )
        .await
        .map_err(|_err| Status::internal("DB find vote options error"))?
        .try_collect()
        .await
        .map_err(|_err| Status::internal("DB collecting vote options error"))?;

    let options: Vec<poll_service::VoteOption> = vote_options_cursor
        .into_iter()
        .map(
            |model::VoteOption {
                 id,
                 name,
                 description,
                 ..
             }| poll_service::VoteOption {
                id: id.unwrap().to_hex(),
                title: name,
                description,
            },
        )
        .collect();

    Ok(Response::new(GetPollBySlugResponse {
        id: poll_id.to_hex(),
        title: poll.name,
        kind: Some(poll_service::PollKind { id: poll.kind }),
        slug: poll.slug,
        options,
        finished: ballot.finished_at.is_some(),
        ballot_id: ballot_id.to_hex(),
    }))
}
