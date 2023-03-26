use chrono::prelude::*;
use chrono::{DateTime, Utc};
mod tag;
mod tree_type;
pub mod plant;
mod utils;
pub mod record;

fn format_time(time:String) -> String {
  let a = time.parse::<DateTime<Local>>().unwrap();
  a.to_rfc3339()
}

pub fn get_data_from_forest(forest_token: String) -> (Vec<record::Record>) {
  let mut time_s = "".to_string();
  let (mut plants, times) = plant::get_plant(time_s, forest_token.clone()).unwrap();
  
  plants.sort_by(|a, b| {
    let a_time = Utc.datetime_from_str(&a.end_time.to_string(), "%Y-%m-%dT%H:%M:%S%.fZ").unwrap();
    let b_time = Utc.datetime_from_str(&b.end_time.to_string(), "%Y-%m-%dT%H:%M:%S%.fZ").unwrap();
    a_time.cmp(&b_time)
  });

  let tree_type = tree_type::get_tree_type(forest_token.clone()).unwrap();
  let tag = tag::get_tag(forest_token.clone()).unwrap();
  let mut data: Vec<record::Record> = vec![];

  for x in plants.iter() {
    // // 构建一个新得返回
    let xx = tree_type.get(&x.trees[0].tree_type).unwrap().to_string();
    let xxx = tag.get(&x.tag).unwrap().to_string();
    data.push(record::Record {
      start_time: format_time(x.start_time.clone()),
      end_time: format_time(x.end_time.clone()),
      tag: xxx,
      note: x.note.clone(),
      tree_type: xx,
      is_success: x.is_success
    })
  }

  (data)
}