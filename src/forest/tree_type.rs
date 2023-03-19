use serde::{Deserialize, Serialize};
use crate::forest::utils;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeType {
  pub gid: u64,
  pub title: String,
  atlas_image_url: String,
  atlas_json_url: String,
  tier: u64,
  owners_only_in_rooms: bool
}

#[tokio::main]
pub async fn get_tree_type() ->  Result<HashMap<u64, String>, Box<dyn std::error::Error>>  {
  let client = reqwest::Client::new();

  let res =
    client.get("https://forest-china.upwardsware.com/api/v1//tree_types?seekrua=extension_chrome-5.8.0")
    .headers(utils::construct_headers()).send().await?;
  
  let json_value: Vec<TreeType> = res.json().await?;

  let mut treeTypeMap = HashMap::new();

  // vec -> hashmap
  for x in json_value.iter() {
      treeTypeMap.insert(x.gid, x.title.clone());
  }

  Ok(treeTypeMap)
}