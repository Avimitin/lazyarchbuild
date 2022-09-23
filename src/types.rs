use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MarkList {
    pub name: Box<str>,
    pub marks: Vec<Mark>,
}

#[derive(Deserialize)]
pub struct Mark {
    pub name: Box<str>,
    #[serde(deserialize_with = "flatten")]
    pub by: Box<str>,
    pub comment: Box<str>,
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
