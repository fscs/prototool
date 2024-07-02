use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
}

#[derive(Debug, Deserialize, PartialEq)]
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

pub fn fetch_current_tops(api_url: &Url, client: &Client) -> Result<Vec<Top>> {
    let endpoint = api_url.join("api/topmanager/current_tops/")?;

    let response = client
        .get(endpoint)
        .send()
        .context("unable to fetch current tops")?;

    let mut tops: Vec<Top> = response.json().context("failed to deserialize tops")?;

    #[allow(clippy::unwrap_used)]
    tops.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    Ok(tops)
}
