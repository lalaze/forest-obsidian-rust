use serde::{Deserialize, Serialize};
use json::JsonValue;
use crate::forest::utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tree {
  pub created_at: String,
  pub updated_at: String,
  pub tree_type: u64,
  pub is_dead: bool,
  pub phase: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plant {
  pub id: u64,
  pub tag: u64,
  pub note: String,
  pub is_success: bool,
  pub start_time: String,
  pub end_time: String,
  pub created_at: String,
  pub updated_at: String,
  pub user_id: u64,
  pub has_left: bool,
  pub deleted: bool,
  pub theme: u64,
  pub cheating: bool,
  pub room_id: Option<u64>,
  pub tree_type_gid: u64,
  pub tree_count: u64,
  pub mode: String,
  pub trees: Vec<Tree>
}

impl std::fmt::Display for Plant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.start_time, self.end_time, self.note, self.tree);
  }
}

#[tokio::main]
pub async fn get_plant(time: String, forest_token: String) ->  Result<(Vec<Plant>, String), Box<dyn std::error::Error>>  {
  let client = reqwest::Client::new();

  let mut url;

  if time.is_empty() {
    url = "https://forest-china.upwardsware.com/api/v1/plants?seekrua=extension_chrome-5.8.0".to_string();
  } else {
    url = format!(r#"https://forest-china.upwardsware.com/api/v1/plants?seekrua=extension_chrome-5.8.0&from_date={}"#, time);
  }

  let res =
    client.get(url)
    .headers(utils::construct_headers(forest_token)).send().await?;

  let json_value: Vec<Plant> = res.json().await?;

  let time: String;
  if !json_value.is_empty() {
    time =  json_value[json_value.len() - 1].end_time.clone();
  } else {
    time = "".to_string();
  }

  Ok((json_value, time))
}
