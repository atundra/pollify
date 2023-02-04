use crate::{model, storage::DATABASE};
use bson::doc;
use common::grpc::poll_service::{CreatePollRequest, CreatePollResponse, NewVoteOption};
use nanoid::nanoid;
use tonic::{Request, Response, Status};

pub async fn create_poll(
    request: Request<CreatePollRequest>,
) -> Result<Response<CreatePollResponse>, Status> {
    let CreatePollRequest {
        title,
        kind,
        slug,
        options,
    } = request.into_inner();

    let kind_id = kind
        .map(|kind| kind.id)
        .ok_or_else(|| Status::invalid_argument("Poll kind should not be empty"))?;

    let slug = slug.unwrap_or_else(|| nanoid!());

    let db = DATABASE.get().await;
    let poll_collection = db.collection::<model::Poll>("polls");

    let same_slug_poll = poll_collection
        .find_one(
            doc! {
                "slug": slug.clone(),
            },
            None,
        )
        .await
        .map_err(|_err| Status::internal("DB unique slug check error"))?;

    if same_slug_poll.is_some() {
        return Err(Status::invalid_argument("Non unique slug"));
    }

    let poll = model::Poll {
        name: title,
        created_at: chrono::Utc::now().into(),
        slug: slug.clone(),
        kind: kind_id,
    };

    let poll_insert_result = poll_collection
        .insert_one(poll, None)
        .await
        .map_err(|_err| Status::internal("DB poll insertion error"))?;

    let poll_id = poll_insert_result.inserted_id.as_object_id().unwrap();

    let ballot = model::Ballot {
        poll_id,
        finished_at: None,
        created_at: chrono::Utc::now().into(),
        number_of_winners: 1,
    };

    let ballot_insert_result = db
        .collection::<model::Ballot>("ballots")
        .insert_one(ballot, None)
        .await
        .map_err(|_err| Status::internal("DB ballot insertion error"))?;

    let ballot_id = ballot_insert_result.inserted_id.as_object_id().unwrap();

    let options: Vec<model::VoteOption> = options
        .iter()
        .enumerate()
        .map(
            |(index, NewVoteOption { title, description })| model::VoteOption {
                ballot_id,
                name: title.to_string(),
                description: description.clone(),
                sort: index.try_into().unwrap(),
            },
        )
        .collect();

    db.collection::<model::VoteOption>("options")
        .insert_many(options, None)
        .await
        .map_err(|_err| Status::internal("DB options insertion error"))?;

    Ok(Response::new(CreatePollResponse {
        id: poll_id.to_string(),
        slug,
    }))
}
