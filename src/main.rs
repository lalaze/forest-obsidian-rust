use std::error::Error;
use serde::Deserialize;
use chrono::prelude::{ DateTime, Utc, Local};
use std::{thread, time};
use csv::Writer;
// mod record;
mod forest;
#[derive(Debug, Deserialize)]
struct Config {
  notion_token: String,
  database_id: String,
  time: String
}

fn write_time(path: &str, database_id: &str, notion_token: &str, time: String) {
  let mut wtr =Writer::from_path(path).unwrap();
  wtr.write_record(&["notion_token".to_string(), "database_id".to_string(), "time".to_string()]).unwrap();
  wtr.write_record(&[notion_token.to_string(), database_id.to_string(), time]).unwrap();
  wtr.flush().unwrap();
}

fn main() {

  // 请求数据
  // let (data, time) = forest::get_data_from_forest(config.time.to_string());

  // // 写入time
  // write_time("./config/config.csv", &config.database_id, &config.notion_token, time)

}