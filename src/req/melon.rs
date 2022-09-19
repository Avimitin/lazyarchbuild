#![allow(unused)]
use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

pub async fn fetch() -> anyhow::Result<Response> {
    let endpoint = "https://plct-arv.de2670dd.top/pkg";
    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .header("user-agent", "PLCT::ArchRV.StatusWorker")
        .send()
        .await?;
    let response = response.bytes().await?;
    let response: Response = serde_json::from_slice(&response)?;

    Ok(response)
}

#[derive(Deserialize)]
pub struct Response {
    #[serde(rename = "workList")]
    worklist: Vec<WorkList>,
    #[serde(rename = "markList")]
    marklist: Vec<MarkList>,
}

#[derive(Deserialize)]
pub struct WorkList {
    alias: Box<str>,
    packages: Vec<Box<str>>,
}

#[derive(Deserialize)]
pub struct MarkList {
    name: Box<str>,
    marks: Vec<Mark>,
}

#[derive(Deserialize)]
pub struct Mark {
    name: Box<str>,
    #[serde(deserialize_with = "flatten")]
    by: Box<str>,
    comment: Box<str>,
}

fn flatten<'de, D>(d: D) -> Result<Box<str>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: HashMap<String, String> = HashMap::deserialize(d)?;

    let alias: Box<str> = s["alias"].as_str().into();

    Ok(alias)
}

#[test]
fn test_flatten() {
    let raw = r#"   {
      "name": "bear",
      "marks": [
        {
          "name": "failing",
          "by": {
            "alias": "null (bot)"
          },
          "comment": "2022/9/4 15:25:49 (UTC+8)"
        },
        {
          "name": "noqemu",
          "by": {
            "alias": "Moody"
          },
          "comment": ""
        }
      ]
    }"#;

    let marklist: MarkList = serde_json::from_str(raw).unwrap();
    let marker = &marklist.marks[1].by;
    assert_eq!(marker.as_ref(), "Moody");
}
