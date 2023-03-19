use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "Start Time")]
    pub start_time: String,
    #[serde(rename = "End Time")]
    pub end_time: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "Note")]
    pub note: String,
    #[serde(rename = "Tree Type")]
    pub tree_type: String,
    #[serde(rename = "Is Success")]
    pub is_success: bool
}