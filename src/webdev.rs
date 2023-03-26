use reqwest::{Client, Url};
use crate::forest::record::Record;

fn to_serve(user: String, password: String, file_path: String, data: Vec<Record>) {
  let client = Client::new();
  let url = Url::parse(&file_path).unwrap();

  let url = "https://webdav.lalaze.com/obsidian-data/forest/forest.md";


}