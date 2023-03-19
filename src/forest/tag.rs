use serde::{Deserialize, Serialize};
use crate::forest::utils;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
  id: u64,
  pub title: String,
  pub tag_id: u64,
  user_id: u64,
  deleted: bool,
  created_at: String,
  updated_at: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TagResult {
  update_since: String,
  tags: Vec<Tag>
}

#[tokio::main]
pub async fn get_tag() ->  Result<HashMap<u64, String>, Box<dyn std::error::Error>>  {
  let client = reqwest::Client::new();

  let data =
    client.get("https://forest-china.upwardsware.com/api/v1/tags?seekrua=extension_chrome-5.8.0")
    .headers(utils::construct_headers()).send().await?.json::<TagResult>().await?;

  let mut tagMap = HashMap::new();

  // vec -> hashmap
  for x in data.tags.iter() {
      tagMap.insert(x.tag_id, x.title.clone());
  }

  Ok(tagMap)
}
