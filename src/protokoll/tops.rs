use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum TopType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "sonstiges")]
    Sonstige,
}

#[derive(Debug, Deserialize)]
pub struct Top {
    pub weight: i64,
    pub name: String,
    pub anträge: Vec<Antrag>,
    pub top_type: TopType,
}

pub fn fetch_tops(api_url: &Url, client: &Client, datetime: &NaiveDateTime) -> Result<Vec<Top>> {
    let endpoint = api_url.join("api/topmanager/tops_by_date/")?;

    let mut params = HashMap::new();
    params.insert("datum", datetime.format("%Y-%m-%dT%H:%M:%S").to_string());

    let response = client
        .get(endpoint)
        .json(&params)
        .send()
        .context("unable to fetch tops")?;

    let mut tops: Vec<Top> = response.json().context("failed to deserialize tops")?;

    #[allow(clippy::unwrap_used)]
    tops.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    Ok(tops)
}
