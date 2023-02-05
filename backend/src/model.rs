use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Poll {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub kind: i32,
    pub slug: String,
    pub created_at: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Ballot {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub poll_id: ObjectId,
    pub finished_at: Option<DateTime>,
    pub created_at: DateTime,
    pub number_of_winners: i32,
}

#[derive(Serialize, Deserialize)]
pub struct VoteOption {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub ballot_id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub option_ids: Vec<ObjectId>,
    pub created_at: DateTime,
}
