use chrono::prelude::*;
mod tag;
mod tree_type;
mod plant;
mod utils;
pub mod record;

fn format_time(time:String) -> String {
  let a = time.parse::<DateTime<Local>>().unwrap();
  a.to_rfc3339()
}

pub fn get_data_from_forest(time: String, forest_token: String) -> (Vec<record::Record>, String) {
  let mut plant_data = plant::get_plant(time, forest_token.clone()).unwrap();
  
  let tree_type = tree_type::get_tree_type(forest_token.clone()).unwrap();
  let tag = tag::get_tag(forest_token.clone()).unwrap();
  let mut data: Vec<record::Record> = vec![];

  let time;

  for x in plant_data.iter() {
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

  time = format!("{}", &plant_data[plant_data.len() - 1].end_time);

  (data, time)
}