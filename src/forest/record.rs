use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl std::fmt::Display for Record {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {} {} {} {}", self.start_time, self.end_time, self.tag, self.note, self.tree_type, self.is_success)
  }
}

impl ToString for Record {
  fn to_string(&self) -> String {
      format!("{}", self)
  }
}