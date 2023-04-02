use reqwest::{Client, Url};
use crate::forest::record::Record;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use base64;

#[tokio::main]
pub async fn to_serve(user: String, password: String, file_path: String, data: Vec<Record>) {
  let client = Client::new();

  // let reocrd_str = serde_json::to_string(&data).unwrap();
  
  let mut final_str = "".to_owned();

  for item in data {
    final_str = final_str + item.to_string().as_str();
  }

  let auth_value = format!("{}:{}", user, password);
  let auth_header_value = format!("Basic {}", base64::encode(&auth_value));

  let request = client
        .put(&file_path)
        .header(AUTHORIZATION, auth_header_value)
        .body(final_str);

  let response = request.send().await.unwrap();
  

  println!("{:?}", response);
}