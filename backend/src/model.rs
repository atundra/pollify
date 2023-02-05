use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Poll {
    pub name: String,
    pub kind: i32,
    pub slug: String,
    pub created_at: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Ballot {
    pub poll_id: ObjectId,
    pub finished_at: Option<DateTime>,
    pub created_at: DateTime,
    pub number_of_winners: i32,
}

#[derive(Serialize, Deserialize)]
pub struct VoteOption {
    pub ballot_id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    option_ids: Vec<ObjectId>,
    created_at: DateTime,
}
