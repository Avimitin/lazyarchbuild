#![allow(unused)]
use crate::types::MarkList;
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
    pub worklist: Vec<WorkList>,
    #[serde(rename = "markList")]
    pub marklist: Vec<MarkList>,
}

#[derive(Deserialize)]
pub struct WorkList {
    pub alias: Box<str>,
    pub packages: Vec<Box<str>>,
}
