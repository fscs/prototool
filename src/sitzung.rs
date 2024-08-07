use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Sitzung {
    #[serde(rename = "datum")]
    pub date: NaiveDateTime,
}

pub fn fetch_sitzung(api_url: &Url, client: &Client, datetime: &NaiveDateTime) -> Result<Sitzung> {
    let endpoint = api_url.join("api/topmanager/sitzung_by_date/")?;

    let mut params = HashMap::new();
    params.insert("datum", datetime);

    let response = client
        .get(endpoint)
        .json(&params)
        .send()
        .context("unable to fetch next sitzung")?;

    let sitzung = response.json().context("failed to deserialize sitzung")?;

    Ok(sitzung)
}
