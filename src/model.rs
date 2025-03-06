use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueSkyMsg {
    pub commit: Option<Commit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub record: Option<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub tags: Option<Vec<String>>,
    pub text: String,
}