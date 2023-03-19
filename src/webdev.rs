use reqwest::{Client, Url};
use crate::forest::record::Record;

fn vec_string(records: Vec<Record>) {

}

fn to_serve(file_path: String) {
  let client = Client::new();
  let url = Url::parse(&file_path).unwrap();


}