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

pub fn get_data_from_forest(time: String) -> (Vec<record::Record>, String) {
  let mut plant_data = plant::get_plant(time).unwrap();
  
  // fake Data to test

  // let mut plant_data = vec![plant::Plant {
  //   id: 241469514,
  //   tag: 11,
  //   note: "".to_string(),
  //   is_success: true,
  //   start_time: "2022-11-01T08:08:05.500Z".to_string(),
  //   end_time: "2022-11-01T08:33:05.500Z".to_string(),
  //   created_at: "2022-11-01T08:33:15.293Z".to_string(),
  //   updated_at: "2022-11-01T08:33:15.293Z".to_string(),
  //   user_id: 2441855,
  //   has_left: false,
  //   deleted: false,
  //   theme: 0,
  //   cheating: false,
  //   room_id: Some(1),
  //   tree_type_gid: 0,
  //   tree_count: 1,
  //   mode: "countdown".to_string(),
  //   trees: vec![plant::Tree {
  //     created_at: "2022-11-01T08:33:15.293Z".to_string(),
  //     updated_at: "2022-11-01T08:33:15.293Z".to_string(),
  //     tree_type: 0,
  //     is_dead: false,
  //     phase: 4
  //   }]
  // }];
  let tree_type = tree_type::get_tree_type().unwrap();
  let tag = tag::get_tag().unwrap();
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