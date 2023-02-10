use bson::{doc, oid::ObjectId};
use mongodb::options::UpdateModifications;

use crate::{model, storage::DATABASE};

pub async fn finalize_ballot(
    ballot_oid: ObjectId,
) -> Result<Option<model::Ballot>, mongodb::error::Error> {
    let db = DATABASE.get().await;

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
}
