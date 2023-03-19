use std::collections::HashMap;
use reqwest::header::HeaderMap;

pub fn construct_headers() -> HeaderMap {
  let mut map = HashMap::new();
  map.insert("cookie".to_string(), "".to_string());
  let headers: HeaderMap = (&map).try_into().expect("valid headers");
  headers
}